
use game::GameObjects;
use resource_manager::ResourceManager;

pub trait Scene {
    fn enter(&mut self) {}

    fn leave(&mut self) {}

    fn update(&mut self) -> bool { false }

    fn draw(&mut self) {}
}

pub struct SceneManager {
    scenes: Vec<Box<Scene>>,
    current_scene: usize,
}

impl SceneManager {
    pub fn new() -> SceneManager{
        SceneManager {
            scenes: Vec::new(),
            current_scene: 0,
        }
    }

    pub fn add_scene<T: Scene + 'static>(&mut self, scene: T) {
        self.scenes.push(Box::new(scene));
    }

    pub fn run(&mut self, resource_manager: &ResourceManager, game_objects: &mut GameObjects) {
        // TODO
    }
}
