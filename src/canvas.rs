
// SDL2 specific
use sdl2::render::Renderer;

// Internal:
use sprite::Sprite;

pub struct Canvas<'a> {
    pub renderer: Renderer<'a>,
}

impl<'a> Canvas<'a> {
    pub fn draw(&mut self, sprite: &Sprite) {
        // TODO
    }
}
