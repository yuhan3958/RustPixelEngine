use rust_pixel_engine::prelude::*;

struct Pixels;
impl Game for Pixels {
    fn init(_: &mut Context) -> Result<Self, GameError> {
        Ok(Self)
    }
    fn render(&mut self, _: &mut Context, frame: &mut RenderFrame) {
        frame.commands.push(DrawCommand::Clear(Color {
            r: 24,
            g: 30,
            b: 45,
            a: 255,
        }));
        frame.commands.push(DrawCommand::Rect {
            rect: Rect {
                x: 20,
                y: 20,
                width: 60,
                height: 40,
            },
            color: Color {
                r: 255,
                g: 100,
                b: 40,
                a: 255,
            },
        });
    }
}
fn main() -> Result<(), EngineError> {
    Engine::builder().run::<Pixels>()
}
