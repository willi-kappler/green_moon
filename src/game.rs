// SDL2 specific:
use sdl2::{Sdl, EventPump};

// internal:
use sprite::{Sprite};
use canvas::Canvas;

pub struct Game<'a> {
    pub width: u32,
    pub height: u32,
    pub name: String,

    pub canvas: Canvas<'a>,

    pub all_sprites: Vec<Sprite<'a>>,


    // SDL2 specific:
    pub context: Sdl,
    pub event_pump: EventPump,
}

impl<'a> Game<'a> {
    pub fn update_all_sprites(&mut self) {
        for sprite in self.all_sprites.iter_mut() {
            // TODO: calculate dt
            sprite.update(0);
        }
    }

    pub fn draw_all_sprites(&mut self) {
        for sprite in self.all_sprites.iter_mut() {
            sprite.draw();
        }
    }

    pub fn check_collision(&self) {
        // TODO
    }
}
