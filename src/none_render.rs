use crate::render::Render;
use crate::world::World;

struct NoneRenderType {}

impl NoneRenderType {
    fn new() -> Self {
        Self {}
    }
}

impl Render for NoneRenderType {
    fn render(&mut self, _: &World) {}
}
