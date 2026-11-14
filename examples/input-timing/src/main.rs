use rust_pixel_engine::prelude::*;

struct InputTimingExample {
    x: f32,
    y: f32,
}

impl Game for InputTimingExample {
    fn init(ctx: &mut Context) -> Result<Self, GameError> {
        ctx.input_mut().bind_action(
            "move_right",
            vec![Button::Key(Key::ArrowRight), Button::Key(Key::D)],
        );
        println!("Arrow keys move the square; D also moves right.");
        println!("Click or press Space to inspect ordered events. Escape exits.");
        Ok(Self { x: 150.0, y: 80.0 })
    }

    fn fixed_update(&mut self, ctx: &mut Context, dt: f32) {
        let input = ctx.input();
        let speed = 60.0;
        if input.pressed(Key::ArrowLeft) { self.x -= speed * dt; }
        if input.action_pressed("move_right") { self.x += speed * dt; }
        if input.pressed(Key::ArrowUp) { self.y -= speed * dt; }
        if input.pressed(Key::ArrowDown) { self.y += speed * dt; }
        for event in input.events() {
            println!("fixed: {event:?}");
        }
    }

    fn update(&mut self, ctx: &mut Context, _dt: f32) {
        for event in ctx.input().events() {
            println!("frame: {event:?}");
        }
        if ctx.input().mouse_just_pressed(MouseButton::Left) {
            println!("Mouse logical position: {:?}", ctx.input().logical_mouse_position());
        }
        if ctx.input().just_pressed(Key::Escape) { ctx.request_exit(); }
    }

    fn render(&mut self, _ctx: &mut Context, frame: &mut RenderFrame) {
        frame.commands.push(DrawCommand::Clear(Color { r: 24, g: 30, b: 45, a: 255 }));
        frame.commands.push(DrawCommand::Rect {
            rect: Rect { x: self.x as i32, y: self.y as i32, width: 20, height: 20 },
            color: Color { r: 255, g: 160, b: 70, a: 255 },
        });
    }
}

fn main() -> Result<(), EngineError> {
    Engine::builder()
        .title("RustPixelEngine - Input and Timing")
        .resolution(320, 180)
        .window_size(960, 540)
        .timing(TimingConfig { fixed_hz: 30, max_steps: 5 })
        .run::<InputTimingExample>()
}
