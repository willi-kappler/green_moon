
extern crate green_moon;

use green_moon::{
    GameBuilder,
    SpriteSheet,
    SpriteBuilder,
    AnimationBuilder,
    ANIMATE_PING_PONG,
    ANIMATE_NONE,
};

mod scenes;
mod constants;

use scenes::MainScene;
use constants::*;

fn main() {
    let ball_sprite_sheet = SpriteSheet::new("resources/gfx/ball_sprite_sheet.png", 64, 64).unwrap();

    let ball_animation = AnimationBuilder::new()
        .animation_type(ANIMATE_PING_PONG)
        .set_frames(vec![(0, 100), (1, 100), (2, 100), (3, 100)])
        .build()
        .unwrap();

    let ball_sprite = SpriteBuilder::new()
        .alive(true)
        .group(BALL_GROUP)
        .position_xy((SCREEN_WIDTH / 2) as f64, (SCREEN_HEIGHT / 2) as f64)
        .velocity_xy(10.0, 10.0)
        .add_sprite_sheet(BALL_SPRITE_SHEET)
        .add_animation(BALL_ANIMATION)
        .build()
        .unwrap();

    let paddle_sprite_sheet = SpriteSheet::new("resources/gfx/paddle_sprite_sheet.png", 64, 64).unwrap();

    let paddle_animation = AnimationBuilder::new()
        .animation_type(ANIMATE_NONE)
        .build()
        .unwrap();

    let paddle_sprite_left = SpriteBuilder::new()
        .alive(true)
        .group(PADDLE_GROUP)
        .position_xy(0.0, (SCREEN_HEIGHT / 2) as f64)
        .add_sprite_sheet(PADDLE_SPRITE_SHEET)
        .add_animation(PADDLE_ANIMATION)
        .build()
        .unwrap();

    let paddle_sprite_right = SpriteBuilder::new()
        .alive(true)
        .group(PADDLE_GROUP)
        .position_xy(SCREEN_WIDTH as f64, (SCREEN_HEIGHT / 2) as f64)
        .add_sprite_sheet(PADDLE_SPRITE_SHEET)
        .add_animation(PADDLE_ANIMATION)
        .build()
        .unwrap();

    let main_scene = MainScene::new();

    let mut my_game = GameBuilder::new()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .name("Green Moon Pong")
        .add_scene(main_scene)
        .add_sprite(ball_sprite)
        .add_sprite(paddle_sprite_left)
        .add_sprite(paddle_sprite_right)
        .build()
        .unwrap();

    my_game.add_sprite_sheet(ball_sprite_sheet); // index 0
    my_game.add_sprite_sheet(paddle_sprite_sheet); // index 1
    my_game.add_animation(ball_animation); // index 0
    my_game.add_animation(paddle_animation); // index 1
    my_game.run();
}
