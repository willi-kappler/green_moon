
extern crate green_moon;

use green_moon::{
    Game,
    Animation,
    ANIMATE_LOOP,
    SpriteBuilder,
};

mod constants;
mod scenes;

use constants::{
    WINDOW_WIDTH,
    WINDOW_HEIGHT,
    BALL_GROUP,
    BALL_SPRITE_SHEET,
    PADDLE_GROUP,
    PADDLE_SPRITE_SHEET,
    PADDLE_HALF_WIDTH,
};

use scenes::MainScene;

fn main() {
    println!("Simple pong game written in Rust using the Green Moon game engine");

    // Game objects
    // frames: [(index1, duration1), (index2, duration2), ...], duration in ms.
    let ball_animation = Animation::new(vec![(0, 200), (1, 200), (2, 200), (3, 200)], ANIMATE_LOOP);

    let ball = SpriteBuilder::new()
        .group(BALL_GROUP)
        .pos((WINDOW_WIDTH / 2) as f64, (WINDOW_HEIGHT / 2) as f64)
        .animation(ball_animation)
        .sprite_sheet(BALL_SPRITE_SHEET)
        .build();

    let paddle_left = SpriteBuilder::new()
        .group(PADDLE_GROUP)
        .pos(PADDLE_HALF_WIDTH as f64, (WINDOW_HEIGHT / 2) as f64)
        .sprite_sheet(PADDLE_SPRITE_SHEET)
        .build();

    let paddle_right = SpriteBuilder::new()
        .group(PADDLE_GROUP)
        .pos((WINDOW_WIDTH - PADDLE_HALF_WIDTH) as f64, (WINDOW_HEIGHT / 2) as f64)
        .sprite_sheet(PADDLE_SPRITE_SHEET)
        .build();

    let mut game = Game::new("Green Moon Pong", WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

    game.add_sprite(ball);
    game.add_sprite(paddle_left);
    game.add_sprite(paddle_right);

    game.add_sprite_sheet("resources/gfx/ball.png", 64, 64);
    game.add_sprite_sheet("resources/gfx/paddle.png", 64, 64);

    game.add_scene(MainScene::new());

    game.run();
}
