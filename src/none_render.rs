use crate::render::Render;
use crate::world::World;

struct NoneRenderBackend {}

impl NoneRenderBackend {
    fn new() -> Self {
        Self {}
    }
}

impl Render for NoneRenderBackend {
    fn render(&mut self, _: &World) {}
}
