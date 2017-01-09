// SDL2 specific:
use sdl2::{Sdl, EventPump};

// internal:
use sprite::Sprite;
use canvas::Canvas;
use animation::Animation;
use sprite_sheet;
use sprite_sheet::SpriteSheet;
use behaviour::Behaviour;

pub struct Game<'a> {
    pub width: u32,
    pub height: u32,
    pub name: String,

    pub all_behaviours: Vec<Box<Behaviour>>,

    pub canvas: Canvas<'a>,

    pub all_sprites: Vec<Sprite>,
    pub all_sprite_sheets: Vec<SpriteSheet>,
    pub all_animations: Vec<Animation>,

    // SDL2 specific:
    pub context: Sdl,
    pub event_pump: EventPump,
}

impl<'a> Game<'a> {
    pub fn add_animation(&mut self, animation: Animation) {
        self.all_animations.push(animation);
    }

    pub fn add_sprite_sheet(&mut self, mut sprite_sheet: SpriteSheet) -> Result<(), sprite_sheet::ErrorKind> {
        sprite_sheet.load_resources(&mut self.canvas)?;
        self.all_sprite_sheets.push(sprite_sheet);
        Ok(())
    }

    pub fn update_all_sprites(&mut self) {
        // TODO: calculate dt
        let dt = 0;

        for sprite in self.all_sprites.iter_mut() {
            sprite.update(dt);
            let ref mut current_animation = self.all_animations[sprite.current_animation];
            sprite.current_animation_frame = current_animation.next(dt, sprite.current_animation_frame);
            //self.all_behaviours[sprite.current_behaviour].update(sprite, self);
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
