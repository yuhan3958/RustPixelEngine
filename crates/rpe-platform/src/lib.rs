pub use winit;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::Window;

/// Constructs the attributes used when creating the operating system window.
pub fn window_attributes(title: &str, width: u32, height: u32) -> winit::window::WindowAttributes {
    Window::default_attributes()
        .with_title(title)
        .with_inner_size(LogicalSize::new(width as f64, height as f64))
}

pub fn run<A: ApplicationHandler>(app: &mut A) -> Result<(), String> {
    let event_loop = EventLoop::new().map_err(|err| err.to_string())?;
    event_loop.run_app(app).map_err(|err| err.to_string())
}
