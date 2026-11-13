//! Renderer-independent drawing commands.

/// A color represented by straight RGBA components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// A rectangle in logical pixel coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// A drawing operation. Operations execute in submission order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawCommand {
    Clear(Color),
    Pixel { x: i32, y: i32, color: Color },
    Rect { rect: Rect, color: Color },
}

/// Commands submitted for one frame.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RenderFrame {
    pub commands: Vec<DrawCommand>,
}

/// A renderer consuming a frame of drawing commands.
pub trait RenderBackend {
    type Error: std::error::Error;

    fn render(&mut self, frame: &RenderFrame) -> Result<(), Self::Error>;
}
