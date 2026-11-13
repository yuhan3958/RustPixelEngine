use std::time::{Duration, Instant};

use rpe_core::{Context, EngineError, Game};
use rpe_platform::winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
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
        let mut app = Runtime::<G>::new(config);

        let platform_result = rpe_platform::run(&mut app);

        // Also clean up if the event loop returns an error.
        app.shutdown();

        if let Some(error) = app.error.take() {
            return Err(error);
        }

        platform_result.map_err(EngineError::Platform)
    }
}

struct Runtime<G: Game> {
    config: EngineBuilder,
    window: Option<Window>,
    game: Option<G>,
    context: Context,
    next_update: Instant,
    error: Option<EngineError>,
}

impl<G: Game> Runtime<G> {
    fn new(config: EngineBuilder) -> Self {
        let context = Context::new(config.logical_width, config.logical_height);

        Self {
            config,
            window: None,
            game: None,
            context,
            next_update: Instant::now() + UPDATE_INTERVAL,
            error: None,
        }
    }

    fn fail(&mut self, error: EngineError, event_loop: &ActiveEventLoop) {
        self.error = Some(error);
        event_loop.exit();
    }

    fn shutdown(&mut self) {
        if let Some(mut game) = self.game.take() {
            game.shutdown(&mut self.context);
        }

        self.window = None;
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

        if self.context.is_exit_requested() {
            event_loop.exit();
        }
    }
}

impl<G: Game> ApplicationHandler for Runtime<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Resumed can happen more than once.
        if self.window.is_some() {
            return;
        }

        let attributes = Window::default_attributes()
            .with_title(self.config.title.clone())
            .with_inner_size(LogicalSize::new(
                self.config.window_width as f64,
                self.config.window_height as f64,
            ));

        let window = match event_loop.create_window(attributes) {
            Ok(window) => window,
            Err(error) => {
                self.fail(EngineError::Platform(error.to_string()), event_loop);
                return;
            }
        };

        self.window = Some(window);

        // Game initialization must happen exactly once.
        if self.game.is_none() {
            match G::init(&mut self.context) {
                Ok(game) => {
                    self.game = Some(game);
                    self.next_update = Instant::now() + UPDATE_INTERVAL;
                }
                Err(error) => {
                    self.fail(error.into(), event_loop);
                    return;
                }
            }
        }

        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_update));
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

        if let WindowEvent::CloseRequested = event {
            self.context.request_exit();
            event_loop.exit();
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.game.is_none() {
            return;
        }

        self.update(event_loop);

        if !self.context.is_exit_requested() {
            event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_update));
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.shutdown();
    }
}
