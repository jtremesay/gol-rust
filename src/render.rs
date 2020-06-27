use crate::world::World;

pub enum RenderBackend {
    None,
    Piston,
}

pub trait Render {
    fn render(&mut self, world: &World);
}
