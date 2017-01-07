
use scene_manager::SceneManager;
use game::Game;

pub struct GameManager<'a> {
    pub scene_manager: SceneManager,
    pub game: Game<'a>,
}

impl<'a> GameManager<'a> {
    pub fn run(&mut self) {
        self.scene_manager.run(&mut self.game);
    }
}
