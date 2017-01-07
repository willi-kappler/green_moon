
extern crate green_moon;

use green_moon::{
    Scene,
    Game,
    GameBuilder,
    SpriteSheet,
    SpriteBuilder,
    AnimationBuilder,
    AnimationType,
    Event,
    Keycode,
};

const BALL_GROUP: u32 = 1;
const PADDLE_GROUP: u32 = 2;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

const BALL: usize = 0;
const PADDLE_LEFT: usize = 1;
const PADDLE_RIGHT: usize = 2;

const MAX_VELO: f64 = 10.0;

struct MainScene {
}

impl Scene for MainScene {
    fn update(&mut self, game: &mut Game) -> bool {
        for event in game.event_pump.poll_iter() {
            match event {
               Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                   return true;
               },

               Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                   game.all_sprites[PADDLE_RIGHT].set_velocity_xy(0.0, -MAX_VELO);
               },

               Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                   game.all_sprites[PADDLE_RIGHT].set_velocity_xy(0.0, MAX_VELO);
               },

               Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                   game.all_sprites[PADDLE_LEFT].set_velocity_xy(0.0, -MAX_VELO);
               },

               Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                   game.all_sprites[PADDLE_LEFT].set_velocity_xy(0.0, MAX_VELO);
               },

               Event::KeyUp {keycode: Some(Keycode::Up), ..} => {
                   game.all_sprites[PADDLE_RIGHT].set_velocity_xy(0.0, 0.0);
               },

               Event::KeyUp {keycode: Some(Keycode::Down), ..} => {
                   game.all_sprites[PADDLE_RIGHT].set_velocity_xy(0.0, 0.0);
               },

               Event::KeyUp {keycode: Some(Keycode::W), ..} => {
                   game.all_sprites[PADDLE_LEFT].set_velocity_xy(0.0, 0.0);
               },

               Event::KeyUp {keycode: Some(Keycode::S), ..} => {
                   game.all_sprites[PADDLE_LEFT].set_velocity_xy(0.0, 0.0);
               },

               _ => {
                    // TODO
               }
           }
        }

        return false;
    }
}

impl MainScene {
    pub fn new() -> MainScene {
        MainScene {}
    }
}

fn main() {
    let ball_sprite_sheet = SpriteSheet::new("resources/gfx/ball_sprite_sheet.png", 64, 64).unwrap();
    let paddle_sprite_sheet = SpriteSheet::new("resources/gfx/paddle_sprite_sheet.png", 64, 64).unwrap();

    let ball_animation = AnimationBuilder::new()
        .animation_type(AnimationType::PingPong)
        .add_frames(vec![(0, 100), (1, 100), (2, 100), (3, 100)])
        .build()
        .unwrap();

    let ball_sprite = SpriteBuilder::new()
        .alive(true)
        .group(BALL_GROUP)
        .position_xy((SCREEN_WIDTH / 2) as f64, (SCREEN_HEIGHT / 2) as f64)
        .velocity_xy(10.0, 10.0)
        .add_sprite_sheet(&ball_sprite_sheet)
        .add_animation(ball_animation)
        .build()
        .unwrap();

    let paddle_animation = AnimationBuilder::new()
        .animation_type(AnimationType::NoAnimation)
        .build()
        .unwrap();

    let paddle_sprite_left = SpriteBuilder::new()
        .alive(true)
        .group(PADDLE_GROUP)
        .position_xy(0.0, (SCREEN_HEIGHT / 2) as f64)
        .add_sprite_sheet(&paddle_sprite_sheet)
        .add_animation(paddle_animation.clone())
        .build()
        .unwrap();

    let paddle_sprite_right = SpriteBuilder::new()
        .alive(true)
        .group(PADDLE_GROUP)
        .position_xy(SCREEN_WIDTH as f64, (SCREEN_HEIGHT / 2) as f64)
        .add_sprite_sheet(&paddle_sprite_sheet)
        .add_animation(paddle_animation.clone())
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

    my_game.run();
}
