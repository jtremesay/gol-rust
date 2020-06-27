use crate::world::World;

pub enum RenderType {
    None,
    Piston,
}

pub trait Render {
    fn render(&mut self, world: &World);
}
