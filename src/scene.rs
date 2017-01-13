
use game::GameObjects;
use resource_manager::ResourceManager;

pub trait Scene {
    fn enter(&mut self, resource_manager: &ResourceManager, game_objects: &mut GameObjects) {}

    fn leave(&mut self, resource_manager: &ResourceManager, game_objects: &mut GameObjects) {}

    fn update(&mut self, resource_manager: &ResourceManager, game_objects: &mut GameObjects) -> bool { false }

    fn draw(&mut self, resource_manager: &ResourceManager, game_objects: &mut GameObjects) {}
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

    pub fn run(mut self, resource_manager: ResourceManager, mut game_objects: GameObjects) {
        let ref mut current_scene = self.scenes[self.current_scene];

        current_scene.enter(&resource_manager, &mut game_objects);

        let mut quit = current_scene.update(&resource_manager, &mut game_objects);
        current_scene.draw(&resource_manager, &mut game_objects);

        while !quit {
            quit = current_scene.update(&resource_manager, &mut game_objects);
            current_scene.draw(&resource_manager, &mut game_objects);
        }

        current_scene.leave(&resource_manager, &mut game_objects);
    }
}
