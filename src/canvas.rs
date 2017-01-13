
use sdl2::render::Renderer;

pub struct Canvas<'a> {
    renderer: Renderer<'a>,
}

impl<'a> Canvas<'a> {
    pub fn new(renderer: Renderer) -> Canvas {
        Canvas {
            renderer: renderer
        }
    }

    pub fn draw(&mut self) {
        // TODO
    }
}
