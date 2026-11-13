pub use glam::{IVec2, UVec2, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size2D {
    pub width: u32,
    pub height: u32,
}

impl Size2D {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
