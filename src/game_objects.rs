
use sprite::Sprite;
use canvas::Canvas;
use resource_manager::ResourceManager;

pub struct GameObjects {
    sprites: Vec<Sprite>,
    canvas: Canvas,
}

impl GameObjects {
    pub fn new() -> GameObjects {
        GameObjects {
            sprites: Vec::new(),
            canvas: Canvas::new(),
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
