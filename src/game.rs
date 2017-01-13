
use sprite::Sprite;
use scene::{SceneManager, Scene};
use resource_manager::ResourceManager;
use game_objects::GameObjects;

pub struct Game {
    resource_manager: ResourceManager,
    scene_manager: SceneManager,
    game_objects: GameObjects,
}

impl Game {
    pub fn new(title: &str, window_width: u16, window_height: u16) -> Game {
        Game {
            resource_manager: ResourceManager::new(title, window_width, window_height),
            scene_manager: SceneManager::new(),
            game_objects: GameObjects::new(),
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.game_objects.add_sprite(sprite);
    }

    pub fn add_scene<T: Scene + 'static>(&mut self, scene: T) {
        self.scene_manager.add_scene(scene);
    }

    pub fn add_sprite_sheet(&self, file_name: &str, tile_width: u16, tile_height: u16) {
        self.resource_manager.add_sprite_sheet(file_name, tile_width, tile_height);
    }

    pub fn load_resources(&self, file_name: &str) {
        self.resource_manager.load_resources(file_name);
    }

    pub fn run(mut self) {
        self.scene_manager.run(self.resource_manager, self.game_objects);
    }
}
