
use game::Game;
use scene::Scene;

pub struct SceneManager {
    pub all_scenes: Vec<Box<Scene>>,
    pub current_scene: usize,
}

impl SceneManager {
    pub fn run(&mut self, game: &mut Game) {
        let mut quit = false;

        while !quit {
            let ref mut current_scene = self.all_scenes[self.current_scene];

            game.canvas.renderer.clear();

            quit = current_scene.update(game);

            current_scene.draw(game);

            game.update_all_sprites();

            game.draw_all_sprites();

            game.canvas.renderer.present();
        }
    }

    pub fn change_scene(&mut self, new_scene: usize, game: &mut Game) {
        self.all_scenes[self.current_scene].leave(game);

        self.current_scene = new_scene;

        self.all_scenes[self.current_scene].enter(game);
    }
}
