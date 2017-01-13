

use sdl2;

use sprite::Sprite;
use scene::{SceneManager, Scene};
use resource_manager::ResourceManager;
use game_objects::GameObjects;

error_chain! {
}

pub struct Game<'a> {
    resource_manager: ResourceManager,
    scene_manager: SceneManager,
    game_objects: GameObjects<'a>,
}

impl<'a> Game<'a> {
    pub fn new(title: &str, window_width: u32, window_height: u32) -> Result<Game> {

        let context = sdl2::init()?;
        let video_subsystem = context.video()?;
        let window = video_subsystem.window(title, window_width, window_height)
            .position_centered().build().chain_err(|| "Could not open SDL2 window")?;

        let renderer = window.renderer().accelerated().build().chain_err(|| "Could not open SDL2 window")?;
        let event_pump = context.event_pump()?;

        Ok(Game {
            resource_manager: ResourceManager::new(title, window_width, window_height),
            scene_manager: SceneManager::new(),
            game_objects: GameObjects::new(renderer, event_pump),
        })
    }

    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.game_objects.add_sprite(sprite);
    }

    pub fn add_scene<T: Scene + 'static>(&mut self, scene: T) {
        self.scene_manager.add_scene(scene);
    }

    pub fn add_sprite_sheet(&mut self, file_name: &str, tile_width: u16, tile_height: u16) {
        self.resource_manager.add_sprite_sheet(file_name, tile_width, tile_height);
    }

    pub fn import_resources(&mut self, file_name: &str) {
        self.resource_manager.import_resources(file_name);
    }

    pub fn load_resources(&mut self) {
        self.resource_manager.load_resources();
    }

    pub fn run(mut self) {
        self.scene_manager.run(self.resource_manager, self.game_objects);
    }
}
