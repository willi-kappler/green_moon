
use scene_manager::SceneManager;
use game::Game;
use animation::Animation;
use sprite_sheet;
use sprite_sheet::SpriteSheet;

pub struct GameManager<'a> {
    pub scene_manager: SceneManager,
    pub game: Game<'a>,
}

impl<'a> GameManager<'a> {
    pub fn add_animation(&mut self, animation: Animation) {
        self.game.add_animation(animation);
    }

    pub fn add_sprite_sheet(&mut self, sprite_sheet: SpriteSheet) -> Result<(), sprite_sheet::ErrorKind> {
        self.game.add_sprite_sheet(sprite_sheet)
    }

    pub fn run(&mut self) {
        self.scene_manager.run(&mut self.game);
    }
}
