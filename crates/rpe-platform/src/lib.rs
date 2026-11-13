pub use winit;

use winit::application::ApplicationHandler;
use winit::event_loop::EventLoop;

pub fn run<A: ApplicationHandler>(app: &mut A) -> Result<(), String> {
    let event_loop = EventLoop::new().map_err(|err| err.to_string())?;

    event_loop.run_app(app).map_err(|err| err.to_string())
}
