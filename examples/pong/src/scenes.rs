use green_moon::{
    Scene,
    Game,
    Event,
    Keycode,
};

use constants::*;

pub struct MainScene {
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
