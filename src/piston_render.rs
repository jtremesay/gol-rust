use crate::render::Render;
use crate::world::World;

struct PistonRenderBackend {}

impl PistonRenderBackend {
    fn new() -> Self {
        Self {}
    }
}

impl Render for PistonRenderBackend {
    fn render(&mut self, _: &World) {}
}
