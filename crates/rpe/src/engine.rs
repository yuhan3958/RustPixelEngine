use std::time::{Duration, Instant};
use std::sync::Arc;
#[cfg(feature = "cpu")]
use rpe_render::RenderBackend;

use rpe_core::{Context, EngineError, Game, Lifecycle, LifecycleState};
use rpe_platform::winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Window, WindowId},
};

use crate::builder::EngineBuilder;

const UPDATE_INTERVAL: Duration = Duration::from_nanos(16_666_667);

pub struct Engine;

impl Engine {
    pub fn builder() -> EngineBuilder {
        EngineBuilder::new()
    }

    pub(crate) fn run<G: Game>(config: EngineBuilder) -> Result<(), EngineError> {
        log::info!("Starting engine");
        let mut app = Runtime::<G>::new(config);

        let platform_result = rpe_platform::run_with_error(&mut app);

        // Also clean up if the event loop returns an error.
        app.shutdown();

        if let Some(error) = app.error.take() {
            return Err(error);
        }

        log::info!("Event loop stopped");
        platform_result.map_err(|error| EngineError::PlatformSource(Box::new(error)))
    }
}

struct Runtime<G: Game> {
    config: EngineBuilder,
    window: Option<Arc<Window>>,
    #[cfg(feature = "cpu")]
    surface: Option<rpe_platform::SoftwareSurface>,
    #[cfg(feature = "cpu")]
    renderer: Option<rpe_render_cpu::CpuRenderer>,
    #[cfg(feature = "cpu")]
    frame: rpe_render::RenderFrame,
    game: Option<G>,
    context: Context,
    next_update: Instant,
    error: Option<EngineError>,
    lifecycle: Lifecycle,
    suspended: bool,
}

impl<G: Game> Runtime<G> {
    fn new(config: EngineBuilder) -> Self {
        let context = Context::new(config.logical_width, config.logical_height);

        Self {
            config,
            window: None,
            #[cfg(feature = "cpu")]
            surface: None,
            #[cfg(feature = "cpu")]
            renderer: None,
            #[cfg(feature = "cpu")]
            frame: rpe_render::RenderFrame::default(),
            game: None,
            context,
            next_update: Instant::now() + UPDATE_INTERVAL,
            error: None,
            lifecycle: Lifecycle::new(),
            suspended: false,
        }
    }

    fn fail(&mut self, error: EngineError, event_loop: &ActiveEventLoop) {
        self.error = Some(error);
        event_loop.exit();
    }

    fn initialize(&mut self) -> Result<(), EngineError> {
        if self.lifecycle.state() != LifecycleState::Created {
            return Ok(());
        }
        self.lifecycle
            .transition(LifecycleState::Initializing)
            .expect("valid initialization transition");
        match G::init(&mut self.context) {
            Ok(game) => {
                log::info!("Game initialized");
                self.game = Some(game);
                self.lifecycle
                    .transition(LifecycleState::Running)
                    .expect("valid running transition");
                Ok(())
            }
            Err(error) => {
                self.shutdown();
                Err(error.into())
            }
        }
    }

    fn shutdown(&mut self) {
        if matches!(
            self.lifecycle.state(),
            LifecycleState::Initializing | LifecycleState::Running
        ) {
            self.lifecycle
                .transition(LifecycleState::Stopping)
                .expect("valid stopping transition");
        }
        if let Some(mut game) = self.game.take() {
            game.shutdown(&mut self.context);
            log::info!("Game shut down");
        }

        #[cfg(feature = "cpu")]
        { self.surface = None; self.renderer = None; }
        self.window = None;
        if self.lifecycle.state() == LifecycleState::Stopping {
            self.lifecycle
                .transition(LifecycleState::Stopped)
                .expect("valid stopped transition");
        }
    }

    fn update(&mut self, event_loop: &ActiveEventLoop) {
        if self.context.is_exit_requested() {
            event_loop.exit();
            return;
        }

        let now = Instant::now();

        if now < self.next_update {
            return;
        }

        // No catch-up loop for the first implementation.
        self.next_update = now + UPDATE_INTERVAL;

        self.context.advance_time();

        let dt = self.context.time().delta_seconds();

        if let Some(game) = self.game.as_mut() {
            game.update(&mut self.context, dt);
        }

        self.context.input_mut().end_frame();

        if self.context.is_exit_requested() {
            event_loop.exit();
        }
    }
}

impl<G: Game> ApplicationHandler for Runtime<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.suspended = false;
        self.context.reset_time_baseline();
        self.next_update = Instant::now() + UPDATE_INTERVAL;
        if self.window.is_some() {
            event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_update));
            return;
        }

        let attributes = rpe_platform::window_attributes(
            &self.config.title,
            self.config.window_width,
            self.config.window_height,
        );

        let window = match event_loop.create_window(attributes) {
            Ok(window) => window,
            Err(error) => {
                self.fail(EngineError::PlatformSource(Box::new(error)), event_loop);
                return;
            }
        };

        let window = Arc::new(window);
        #[cfg(feature = "cpu")]
        {
            self.surface = match rpe_platform::SoftwareSurface::new(window.clone()) {
                Ok(surface) => Some(surface),
                Err(error) => { self.fail(EngineError::Render(Box::new(error)), event_loop); return; }
            };
            self.renderer = match rpe_render_cpu::CpuRenderer::new(self.config.logical_width, self.config.logical_height) {
                Ok(renderer) => Some(renderer),
                Err(error) => { self.fail(EngineError::Render(Box::new(error)), event_loop); return; }
            };
        }
        let size = window.inner_size();
        self.context.set_window_size(size.width, size.height);
        self.window = Some(window);

        if let Err(error) = self.initialize() {
            self.fail(error, event_loop);
            return;
        }
        self.next_update = Instant::now() + UPDATE_INTERVAL;

        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_update));
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        self.suspended = true;
        self.context.input_mut().release_all();
        event_loop.set_control_flow(ControlFlow::Wait);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                log::info!("Window close requested");
                self.context.request_exit();
                event_loop.exit();
            }
            WindowEvent::Resized(size) => { self.context.set_window_size(size.width, size.height); window.request_redraw(); },
            #[cfg(feature = "cpu")]
            WindowEvent::RedrawRequested => {
                if self.suspended { return; }
                let (Some(game), Some(renderer), Some(surface)) = (self.game.as_mut(), self.renderer.as_mut(), self.surface.as_mut()) else { return; };
                self.frame.commands.clear();
                game.render(&mut self.context, &mut self.frame);
                let _ = renderer.render(&self.frame);
                self.frame.commands.clear();
                let fb = renderer.framebuffer();
                let (width, height) = self.context.window_size();
                if let Err(error) = surface.present(width, height, fb.width(), fb.height(), fb.pixels()) {
                    self.fail(EngineError::Render(Box::new(error)), event_loop);
                }
            },
            WindowEvent::Focused(focused) => {
                self.context.set_focused(focused);
                if !focused {
                    self.context.input_mut().release_all();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let rpe_platform::winit::keyboard::PhysicalKey::Code(code) = event.physical_key
                    && let Some(key) = rpe_platform::translate_key(code)
                {
                    match event.state {
                        rpe_platform::winit::event::ElementState::Pressed => {
                            self.context.input_mut().press(key)
                        }
                        rpe_platform::winit::event::ElementState::Released => {
                            self.context.input_mut().release(key)
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.game.is_none() || self.suspended {
            return;
        }

        let previous_update = self.next_update;
        self.update(event_loop);
        if self.next_update != previous_update {
            if let Some(window) = self.window.as_ref() { window.request_redraw(); }
        }

        if !self.context.is_exit_requested() {
            event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_update));
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static INITS: AtomicUsize = AtomicUsize::new(0);
    static SHUTDOWNS: AtomicUsize = AtomicUsize::new(0);
    struct TestGame;
    impl Game for TestGame {
        fn init(_: &mut Context) -> Result<Self, rpe_core::GameError> {
            INITS.fetch_add(1, Ordering::SeqCst);
            Ok(Self)
        }
        fn shutdown(&mut self, _: &mut Context) {
            SHUTDOWNS.fetch_add(1, Ordering::SeqCst);
        }
    }
    struct FailedGame;
    impl Game for FailedGame {
        fn init(_: &mut Context) -> Result<Self, rpe_core::GameError> {
            Err(rpe_core::GameError::new("init failed"))
        }
    }
    #[test]
    fn initialization_and_shutdown_are_once_only() {
        let mut runtime = Runtime::<TestGame>::new(EngineBuilder::new());
        runtime.initialize().expect("initialization");
        runtime.initialize().expect("duplicate initialization");
        assert_eq!(runtime.lifecycle.state(), LifecycleState::Running);
        runtime.shutdown();
        runtime.shutdown();
        assert_eq!(INITS.load(Ordering::SeqCst), 1);
        assert_eq!(SHUTDOWNS.load(Ordering::SeqCst), 1);
        assert_eq!(runtime.lifecycle.state(), LifecycleState::Stopped);
    }
    #[test]
    fn failed_initialization_stops_without_game() {
        let mut runtime = Runtime::<FailedGame>::new(EngineBuilder::new());
        assert!(matches!(
            runtime.initialize(),
            Err(EngineError::GameInitialization(_))
        ));
        assert!(runtime.game.is_none());
        assert_eq!(runtime.lifecycle.state(), LifecycleState::Stopped);
    }
}
