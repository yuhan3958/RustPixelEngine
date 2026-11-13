use crate::{Context, GameError};

pub trait Game: Sized + 'static {
    fn init(ctx: &mut Context) -> Result<Self, GameError>;

    fn update(&mut self, _ctx: &mut Context, _dt: f32) {}

    fn render(&mut self, _ctx: &mut Context, _frame: &mut rpe_render::RenderFrame) {}

    fn shutdown(&mut self, _ctx: &mut Context) {}
}
