
use sprite::Sprite;
use sprite_sheet::SpriteSheet;
use scene::Scene;

pub struct Game {
    title: String,
    window_width: u16,
    window_height: u16,

    sprites: Vec<Sprite>,
    sprite_sheets: Vec<SpriteSheet>,
    scenes: Vec<Box<Scene>>,
}

impl Game {
    pub fn new(title: &str, window_width: u16, window_height: u16) -> Game {
        Game {
            title: title.to_string(),
            window_width: window_width,
            window_height: window_height,

            sprites: Vec::new(),
            sprite_sheets: Vec::new(),
            scenes: Vec::new(),
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.sprites.push(sprite)
    }

    pub fn add_scene<T: Scene + 'static>(&mut self, scene: T) {
        self.scenes.push(Box::new(scene))
    }

    pub fn add_sprite_sheet(&mut self, file_name: &str, tile_width: u16, tile_height: u16) {
        // TODO
    }

    pub fn load_resources(&mut self, file_name: &str) {
        // TODO
    }

    pub fn run(self) {
        // TODO
    }

}
