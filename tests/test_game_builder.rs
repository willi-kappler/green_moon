extern crate green_moon;

use green_moon::GameBuilder;
use green_moon::Scene;
use green_moon::game_builder;

#[test]
fn game_init_no_scene() {
    let my_game = GameBuilder::new().size(800, 600).name("test game").build();
    match my_game {
        Err(game_builder::Error(game_builder::ErrorKind::ScenesUndefined, _)) => (),
        _ => panic!("NameUndefined failed")
    }
}

#[test]
fn game_init_ok() {
    #[derive(Debug)]
    struct MyScene;

    impl Scene for MyScene {
    }

    let my_game = GameBuilder::new()
        .size(800, 600)
        .name("test game")
        .add_scene(MyScene)
        .build().unwrap();

    assert_eq!(my_game.game.width, 800);
    assert_eq!(my_game.game.height, 600);
    assert_eq!(my_game.game.name, "test game");
}
