//! CPU framebuffer storage.
use rpe_render::Color;
use std::{collections::TryReserveError, fmt};

#[derive(Debug)]
pub enum FramebufferError {
    ZeroSize,
    SizeOverflow,
    Allocation(TryReserveError),
}
impl fmt::Display for FramebufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroSize => write!(f, "framebuffer dimensions must be nonzero"),
            Self::SizeOverflow => write!(f, "framebuffer size overflows"),
            Self::Allocation(e) => write!(f, "framebuffer allocation failed: {e}"),
        }
    }
}
impl std::error::Error for FramebufferError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Allocation(e) => Some(e),
            _ => None,
        }
    }
}

/// Row-major, straight-alpha RGBA8 pixels.
pub struct Framebuffer {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}
impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Result<Self, FramebufferError> {
        if width == 0 || height == 0 {
            return Err(FramebufferError::ZeroSize);
        }
        let len = (width as usize)
            .checked_mul(height as usize)
            .and_then(|n| n.checked_mul(4))
            .ok_or(FramebufferError::SizeOverflow)?;
        let mut pixels = Vec::new();
        pixels
            .try_reserve_exact(len)
            .map_err(FramebufferError::Allocation)?;
        pixels.resize(len, 0);
        Ok(Self {
            width,
            height,
            pixels,
        })
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
    fn offset(&self, x: i32, y: i32) -> Option<usize> {
        let x = u32::try_from(x).ok()?;
        let y = u32::try_from(y).ok()?;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(((y as usize * self.width as usize) + x as usize) * 4)
    }
    pub fn pixel(&self, x: i32, y: i32) -> Option<Color> {
        let i = self.offset(x, y)?;
        Some(Color {
            r: self.pixels[i],
            g: self.pixels[i + 1],
            b: self.pixels[i + 2],
            a: self.pixels[i + 3],
        })
    }
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if let Some(i) = self.offset(x, y) {
            self.pixels[i..i + 4].copy_from_slice(&[color.r, color.g, color.b, color.a]);
        }
    }
    pub fn clear(&mut self, color: Color) {
        for pixel in self.pixels.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[color.r, color.g, color.b, color.a]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boundaries_and_clear() {
        assert!(Framebuffer::new(0, 1).is_err());
        assert!(Framebuffer::new(u32::MAX, u32::MAX).is_err());
        let mut f = Framebuffer::new(2, 1).expect("framebuffer");
        let c = Color {
            r: 1,
            g: 2,
            b: 3,
            a: 4,
        };
        f.clear(c);
        assert_eq!(f.pixel(1, 0), Some(c));
        assert_eq!(f.pixel(-1, 0), None);
        f.set_pixel(2, 0, c);
        assert_eq!(f.pixels().len(), 8);
    }
}

/// Executes draw commands against a CPU framebuffer.
pub struct CpuRenderer {
    framebuffer: Framebuffer,
}
impl CpuRenderer {
    pub fn new(width: u32, height: u32) -> Result<Self, FramebufferError> {
        Ok(Self {
            framebuffer: Framebuffer::new(width, height)?,
        })
    }
    pub fn framebuffer(&self) -> &Framebuffer {
        &self.framebuffer
    }
    fn blend_pixel(&mut self, x: i32, y: i32, source: Color) {
        let Some(dest) = self.framebuffer.pixel(x, y) else {
            return;
        };
        let asrc = u64::from(source.a);
        let adst = u64::from(dest.a);
        let alpha = asrc * 255 + adst * (255 - asrc);
        let channel = |s: u8, d: u8| -> u8 {
            if alpha == 0 {
                0
            } else {
                ((u64::from(s) * asrc * 255 + u64::from(d) * adst * (255 - asrc) + alpha / 2)
                    / alpha) as u8
            }
        };
        self.framebuffer.set_pixel(
            x,
            y,
            Color {
                r: channel(source.r, dest.r),
                g: channel(source.g, dest.g),
                b: channel(source.b, dest.b),
                a: ((alpha + 127) / 255) as u8,
            },
        );
    }
}
impl rpe_render::RenderBackend for CpuRenderer {
    type Error = std::convert::Infallible;
    fn render(&mut self, frame: &rpe_render::RenderFrame) -> Result<(), Self::Error> {
        for command in &frame.commands {
            match *command {
                rpe_render::DrawCommand::Clear(color) => self.framebuffer.clear(color),
                rpe_render::DrawCommand::Pixel { x, y, color } => self.blend_pixel(x, y, color),
                rpe_render::DrawCommand::Rect { rect, color } => {
                    let left = i64::from(rect.x).max(0);
                    let top = i64::from(rect.y).max(0);
                    let right = (i64::from(rect.x) + i64::from(rect.width))
                        .min(i64::from(self.framebuffer.width()))
                        .min(i64::from(i32::MAX) + 1);
                    let bottom = (i64::from(rect.y) + i64::from(rect.height))
                        .min(i64::from(self.framebuffer.height()))
                        .min(i64::from(i32::MAX) + 1);
                    for y in top..bottom {
                        for x in left..right {
                            self.blend_pixel(x as i32, y as i32, color);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod raster_tests {
    use super::*;
    use rpe_render::{DrawCommand, Rect, RenderBackend, RenderFrame};
    #[test]
    fn clips_and_blends_in_order() {
        let mut renderer = CpuRenderer::new(1, 1).expect("renderer");
        let red = Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        let blue = Color {
            r: 0,
            g: 0,
            b: 255,
            a: 128,
        };
        renderer
            .render(&RenderFrame {
                commands: vec![
                    DrawCommand::Clear(red),
                    DrawCommand::Rect {
                        rect: Rect {
                            x: -1,
                            y: -1,
                            width: 2,
                            height: 2,
                        },
                        color: blue,
                    },
                ],
            })
            .expect("render");
        assert_eq!(
            renderer.framebuffer().pixel(0, 0),
            Some(Color {
                r: 127,
                g: 0,
                b: 128,
                a: 255
            })
        );
    }
}
