extern crate green_moon;

use green_moon::{GameBuilder, Game, Scene, Canvas};

#[test]
fn game_update_and_draw() {
    #[derive(Debug)]
    struct MainScene {
        num_of_updates: u32,
        num_of_draws: u32
    }

    impl Scene for MainScene {
        fn update(&mut self, game: &mut Game) -> bool {
            println!("MainScene.update()");
            self.num_of_updates = self.num_of_updates + 1;

            if self.num_of_updates >= 3 {
                false
            } else { true }
        }

        fn draw(&mut self, game: &mut Game) {
            println!("MainScene.draw()");
            self.num_of_draws = self.num_of_draws + 1;
        }
    }

    let mut main_scene = MainScene { num_of_updates: 0, num_of_draws: 0 };

    let mut my_game = GameBuilder::new()
        .size(800, 600)
        .name("test game 1")
        .add_scene(main_scene)
        .build().unwrap();

    my_game.run();

    // TODO: try to figure out how to test this
    //assert_eq!(main_scene.num_of_updates, 3);
    //assert_eq!(main_scene.num_of_draws, 3);
}

#[test]
fn game_change_scene() {
    #[derive(Debug)]
    struct Scene1;

    impl Scene for Scene1 {};

    #[derive(Debug)]
    struct Scene2;

    impl Scene for Scene2 {};

    let mut my_game = GameBuilder::new()
        .size(800, 600)
        .name("test game 2")
        .add_scene(Scene1)
        .add_scene(Scene2)
        .build().unwrap();

    my_game.scene_manager.change_scene(1, &mut my_game.game);

    assert_eq!(my_game.scene_manager.current_scene, 1 as usize);

    my_game.scene_manager.change_scene(0, &mut my_game.game);

    assert_eq!(my_game.scene_manager.current_scene, 0 as usize);

}
