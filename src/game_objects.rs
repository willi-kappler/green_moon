
use sdl2::EventPump;
use sdl2::render::Renderer;

use sprite::Sprite;
use canvas::Canvas;
use resource_manager::ResourceManager;

pub struct GameObjects<'a> {
    sprites: Vec<Sprite>,
    pub canvas: Canvas<'a>,
    event_pump: EventPump,
}

impl<'a> GameObjects<'a> {
    pub fn new(renderer: Renderer, event_pump: EventPump) -> GameObjects {
        GameObjects {
            sprites: Vec::new(),
            canvas: Canvas::new(renderer),
            event_pump: event_pump,
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.sprites.push(sprite);
    }

    pub fn draw(&mut self, resource_manager: &ResourceManager) {
        for sprite in self.sprites.iter() {
            sprite.draw(&mut self.canvas, resource_manager);
        }
    }
}
