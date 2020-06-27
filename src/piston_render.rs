use crate::render::Render;
use crate::world::World;

struct PistonRenderType {}

impl PistonRenderType {
    fn new() -> Self {
        Self {}
    }
}

impl Render for PistonRenderType {
    fn render(&mut self, _: &World) {}
}
