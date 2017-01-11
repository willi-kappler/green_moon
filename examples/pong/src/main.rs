
extern crate green_moon;

use green_moon::{
    Game,
    Animation,
    ANIMATE_LOOP,
    SpriteSheet,
    Sprite,
};

mod constants;
mod scenes;

use constants::{

};

use scenes::MainScene;

fn main() {
    println!("Simple pong game written in Rust using the Green Moon game engine");

    let WINDOW_WIDTH = 800;
    let WINDOW_HEIGHT = 600;
    let PADDLE_HALF_WIDTH = 16;

    // Game objects
    let ball_animation = Animation::new()
        .frames(vec![(0, 200), (1, 200), (2, 200), (3, 200)])
        .type(ANIMATE_LOOP)
        .build().unwrap();

    let ball_sprite_sheet = SpriteSheet::new("resources/gfx/ball.png", 64, 64).unwrap();

    let ball = Sprite::new()
        .id(BALL_ID)
        .group(BALL_GROUP)
        .x(WINDOW_WIDTH / 2)
        .y(WINDOW_HEIGHT / 2)
        .animation(ball_animation)
        .sprite_sheet(ball_sprite_sheet)
        .build().unwrap();

    // NO_ANIMATION is the default here
    let paddle_animation = Animation::new().build().unwrap();

    let paddle_sprite_sheet = SpriteSheet::new("resources/gfx/ball.png", 64, 64).unwrap();

    let paddle_left = Sprite::new()
        .id(PADDLE_ID)
        .group(PADDLE_GROUP)
        .x(PADDLE_HALF_WIDTH)
        .y(WINDOW_HEIGHT / 2)
        .animation(paddle_animation)
        .sprite_sheet(paddle_sprite_sheet)
        .build().unwrap();

    let paddle_right = Sprite::new()
        .id(PADDLE_ID)
        .group(PADDLE_GROUP)
        .x(WINDOW_WIDTH - PADDLE_HALF_WIDTH)
        .y(WINDOW_HEIGHT / 2)
        .animation(paddle_animation)
        .sprite_sheet(paddle_sprite_sheet)
        .build().unwrap();

    let game = Game::new("Green Moon Pong", WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

    game.add_sprite(ball);
    game.add_sprite(paddle_left);
    game.add_sprite(paddle_right);

    game.add_scene(MainScene::new());

    game.run();
}
