use rust_pixel_engine::prelude::*;

struct HelloGame;

impl Game for HelloGame {
    fn init(ctx: &mut Context) -> Result<Self, GameError> {
        println!("[Game] Initialized");

        let (width, height) = ctx.logical_size();

        println!("[Game] Logical resolution: {}x{}", width, height,);

        Ok(Self)
    }

    fn update(&mut self, ctx: &mut Context, dt: f32) {
        let frame = ctx.time().frame_count();

        if frame % 60 == 0 {
            println!(
                "[Game] frame={frame}, dt={dt:.5}, elapsed={:.2}",
                ctx.time().elapsed_seconds(),
            );
        }

        if ctx.time().elapsed_seconds() >= 10.0 {
            ctx.request_exit();
        }
    }

    fn shutdown(&mut self, ctx: &mut Context) {
        println!("[Game] Shutdown after {} updates", ctx.time().frame_count(),);
    }
}

fn main() -> Result<(), EngineError> {
    Engine::builder()
        .title("RustPixelEngine — Hello Window")
        .resolution(320, 180)
        .window_size(1280, 720)
        .run::<HelloGame>()
}
