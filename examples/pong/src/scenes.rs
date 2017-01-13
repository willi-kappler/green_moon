
use green_moon::{
    Scene,
    SceneMessage,
    ResourceManager,
    GameObjects,
};

pub struct MainScene {
    score_left: u8,
    score_right: u8,
}

impl Scene for MainScene {
    fn enter(&mut self, resource_manager: &ResourceManager, game_objects: &mut GameObjects) {

    }

    fn leave(&mut self, resource_manager: &ResourceManager, game_objects: &mut GameObjects) {

    }

    fn update(&mut self, resource_manager: &ResourceManager, game_objects: &mut GameObjects) -> SceneMessage {
        SceneMessage::QuitGame
    }

    fn draw(&mut self, resource_manager: &ResourceManager, game_objects: &mut GameObjects) {
        game_objects.draw(resource_manager);
    }
}

impl MainScene {
    pub fn new() -> MainScene {
        MainScene {
            score_left: 0,
            score_right: 0,
        }
    }
}
