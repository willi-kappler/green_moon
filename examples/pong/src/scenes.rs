
use green_moon::{
    Scene,
    SceneMessage,
    ResourceManager,
    GameObjects,
    Keycode,
    Event,
};

pub struct MainScene {
    score_left: u8,
    score_right: u8,
}

impl Scene for MainScene {
    fn enter(&mut self, resource_manager: &ResourceManager, game_objects: &mut GameObjects) {
        println!("Green Moon Pong: enter main scene");
    }

    fn leave(&mut self, resource_manager: &ResourceManager, game_objects: &mut GameObjects) {
        println!("Green Moon Pong: leave main scene");
    }

    fn update(&mut self, resource_manager: &ResourceManager, game_objects: &mut GameObjects) -> SceneMessage {
        if game_objects.key_pressed(Keycode::Escape) || game_objects.has_quit_event() {
            SceneMessage::QuitGame
        } else {
            SceneMessage::Continue
        }
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
