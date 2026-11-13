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
    run_with_error(app).map_err(|err| err.to_string())
}

pub fn run_with_error<A: ApplicationHandler>(
    app: &mut A,
) -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new()?;
    event_loop.run_app(app)
}

/// Maps a winit physical key to the platform-independent key type.
pub fn translate_key(code: winit::keyboard::KeyCode) -> Option<rpe_input::Key> {
    use winit::keyboard::KeyCode;
    Some(match code {
        KeyCode::KeyA => rpe_input::Key::A,
        KeyCode::KeyB => rpe_input::Key::B,
        KeyCode::KeyC => rpe_input::Key::C,
        KeyCode::KeyD => rpe_input::Key::D,
        KeyCode::KeyE => rpe_input::Key::E,
        KeyCode::KeyF => rpe_input::Key::F,
        KeyCode::KeyG => rpe_input::Key::G,
        KeyCode::KeyH => rpe_input::Key::H,
        KeyCode::KeyI => rpe_input::Key::I,
        KeyCode::KeyJ => rpe_input::Key::J,
        KeyCode::KeyK => rpe_input::Key::K,
        KeyCode::KeyL => rpe_input::Key::L,
        KeyCode::KeyM => rpe_input::Key::M,
        KeyCode::KeyN => rpe_input::Key::N,
        KeyCode::KeyO => rpe_input::Key::O,
        KeyCode::KeyP => rpe_input::Key::P,
        KeyCode::KeyQ => rpe_input::Key::Q,
        KeyCode::KeyR => rpe_input::Key::R,
        KeyCode::KeyS => rpe_input::Key::S,
        KeyCode::KeyT => rpe_input::Key::T,
        KeyCode::KeyU => rpe_input::Key::U,
        KeyCode::KeyV => rpe_input::Key::V,
        KeyCode::KeyW => rpe_input::Key::W,
        KeyCode::KeyX => rpe_input::Key::X,
        KeyCode::KeyY => rpe_input::Key::Y,
        KeyCode::KeyZ => rpe_input::Key::Z,
        KeyCode::Digit0 => rpe_input::Key::Digit0,
        KeyCode::Digit1 => rpe_input::Key::Digit1,
        KeyCode::Digit2 => rpe_input::Key::Digit2,
        KeyCode::Digit3 => rpe_input::Key::Digit3,
        KeyCode::Digit4 => rpe_input::Key::Digit4,
        KeyCode::Digit5 => rpe_input::Key::Digit5,
        KeyCode::Digit6 => rpe_input::Key::Digit6,
        KeyCode::Digit7 => rpe_input::Key::Digit7,
        KeyCode::Digit8 => rpe_input::Key::Digit8,
        KeyCode::Digit9 => rpe_input::Key::Digit9,
        KeyCode::Escape => rpe_input::Key::Escape,
        KeyCode::Space => rpe_input::Key::Space,
        KeyCode::Enter => rpe_input::Key::Enter,
        KeyCode::Tab => rpe_input::Key::Tab,
        KeyCode::Backspace => rpe_input::Key::Backspace,
        KeyCode::ArrowUp => rpe_input::Key::ArrowUp,
        KeyCode::ArrowDown => rpe_input::Key::ArrowDown,
        KeyCode::ArrowLeft => rpe_input::Key::ArrowLeft,
        KeyCode::ArrowRight => rpe_input::Key::ArrowRight,
        KeyCode::ShiftLeft => rpe_input::Key::ShiftLeft,
        KeyCode::ShiftRight => rpe_input::Key::ShiftRight,
        KeyCode::ControlLeft => rpe_input::Key::ControlLeft,
        KeyCode::ControlRight => rpe_input::Key::ControlRight,
        KeyCode::AltLeft => rpe_input::Key::AltLeft,
        KeyCode::AltRight => rpe_input::Key::AltRight,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn translates_physical_keys() {
        assert_eq!(
            translate_key(winit::keyboard::KeyCode::KeyA),
            Some(rpe_input::Key::A)
        );
        assert_eq!(translate_key(winit::keyboard::KeyCode::F24), None);
    }
}

/// A software surface owned on the window event thread.
pub struct SoftwareSurface {
    surface: softbuffer::Surface<std::sync::Arc<Window>, std::sync::Arc<Window>>,
}
impl SoftwareSurface {
    pub fn new(window: std::sync::Arc<Window>) -> Result<Self, softbuffer::SoftBufferError> {
        let context = softbuffer::Context::new(window.clone())?;
        let surface = softbuffer::Surface::new(&context, window)?;
        Ok(Self { surface })
    }
    /// Present a logical RGBA framebuffer using centered integer scaling.
    pub fn present(&mut self, width: u32, height: u32, logical_width: u32, logical_height: u32, rgba: &[u8]) -> Result<(), softbuffer::SoftBufferError> {
        if logical_width == 0 || logical_height == 0 || (logical_width as usize).checked_mul(logical_height as usize).and_then(|n| n.checked_mul(4)).is_none_or(|n| rgba.len() < n) { return Ok(()); }
        let (Some(w), Some(h)) = (std::num::NonZeroU32::new(width), std::num::NonZeroU32::new(height)) else { return Ok(()); };
        self.surface.resize(w, h)?;
        let mut buffer = self.surface.buffer_mut()?;
        buffer.fill(0);
        let scale = (width / logical_width).min(height / logical_height);
        if scale > 0 {
            let left = (width - logical_width * scale) / 2;
            let top = (height - logical_height * scale) / 2;
            for y in 0..logical_height * scale {
                for x in 0..logical_width * scale {
                    let source = (((y / scale) as usize * logical_width as usize) + (x / scale) as usize) * 4;
                    let dest = (top + y) as usize * width as usize + (left + x) as usize;
                    buffer[dest] = (u32::from(rgba[source]) << 16) | (u32::from(rgba[source + 1]) << 8) | u32::from(rgba[source + 2]);
                }
            }
        }
        buffer.present()
    }
}
