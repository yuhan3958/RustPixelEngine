use rust_pixel_engine::prelude::*;

struct RendererDemo;


fn rect(frame: &mut RenderFrame, x: i32, y: i32, width: u32, height: u32, color: Color) {
    frame.commands.push(DrawCommand::Rect {
        rect: Rect {
            x,
            y,
            width,
            height,
        },
        color,
    });
}

impl Game for RendererDemo {
    fn init(_: &mut Context) -> Result<Self, GameError> {
        println!("CPU renderer demo: resize the window to check integer scaling and letterboxing.");
        println!("Red/blue overlap should be purple; shapes at screen edges should be clipped.");
        println!("Press Escape to exit.");
        Ok(Self)
    }

    fn update(&mut self, ctx: &mut Context, _: f32) {
        if ctx.input().just_pressed(Key::Escape) {
            ctx.request_exit();
        }
    }

    fn render(&mut self, ctx: &mut Context, frame: &mut RenderFrame) {
        frame
            .commands
            .push(DrawCommand::Clear(Color::rgba(24, 30, 45, 255)));

        // Opaque red followed by translucent blue demonstrates source-over order.
        rect(frame, 32, 35, 90, 85, Color::rgba(255, 0, 0, 255));
        rect(frame, 76, 65, 90, 85, Color::rgba(0, 0, 255, 128));

        // These shapes cross the framebuffer bounds on all four sides.
        rect(frame, -12, 8, 30, 20, Color::rgba(255, 210, 0, 255));
        rect(frame, 302, 8, 30, 20, Color::rgba(255, 210, 0, 255));
        rect(frame, 180, -10, 20, 25, Color::rgba(0, 220, 160, 255));
        rect(frame, 180, 165, 20, 25, Color::rgba(0, 220, 160, 255));

        // A moving pixel makes updates and frame submission visible.
        let x = (ctx.time().frame_count() % 280) as i32 + 20;
        frame.commands.push(DrawCommand::Pixel {
            x,
            y: 160,
            color: Color::rgba(255, 255, 255, 255),
        });
    }
}

fn main() -> Result<(), EngineError> {
    Engine::builder()
        .title("CPU Renderer Demo")
        .resolution(320, 180)
        .window_size(960, 540)
        .run::<RendererDemo>()
}
