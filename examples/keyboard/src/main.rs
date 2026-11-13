use rust_pixel_engine::prelude::*;

struct KeyboardExample {
    x: f32,
    y: f32,
}

impl Game for KeyboardExample {
    fn init(_ctx: &mut Context) -> Result<Self, GameError> {
        println!("Use the arrow keys to move. Press Space to report the position.");
        println!("Press Escape or close the window to exit.");
        Ok(Self { x: 0.0, y: 0.0 })
    }

    fn update(&mut self, ctx: &mut Context, dt: f32) {
        let input = ctx.input();
        let speed = 100.0;

        if input.pressed(Key::ArrowLeft) {
            self.x -= speed * dt;
        }
        if input.pressed(Key::ArrowRight) {
            self.x += speed * dt;
        }
        if input.pressed(Key::ArrowUp) {
            self.y -= speed * dt;
        }
        if input.pressed(Key::ArrowDown) {
            self.y += speed * dt;
        }
        if input.just_pressed(Key::Space) {
            println!("Space pressed: position ({:.1}, {:.1})", self.x, self.y);
        }
        if input.just_released(Key::Space) {
            println!("Space released");
        }
        if input.just_pressed(Key::Escape) {
            ctx.request_exit();
        }
    }

    fn shutdown(&mut self, ctx: &mut Context) {
        println!(
            "Stopped after {} updates at ({:.1}, {:.1})",
            ctx.time().frame_count(),
            self.x,
            self.y
        );
    }
}

fn main() -> Result<(), EngineError> {
    Engine::builder()
        .title("RustPixelEngine - Keyboard Example")
        .resolution(320, 180)
        .window_size(960, 540)
        .run::<KeyboardExample>()
}
