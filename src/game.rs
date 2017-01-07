// SDL2 specific:
use sdl2::{Sdl, EventPump};

// internal:
use scene::Scene;
use sprite::{Sprite, Behaviour};
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

    pub fn update_all_alive_sprites(&mut self) {
        for sprite in self.all_sprites.iter_mut() {
            if sprite.alive {
                sprite.update();

                match sprite.behaviour {
                    Behaviour::None => {},
                    Behaviour::StopOnBounds => {
                        if sprite.position.x <= 0.0 {
                            sprite.position.x = 0.0;
                            sprite.velocity.x = 0.0;
                            sprite.acceleration.x = 0.0;
                        } else if sprite.position.x >= (self.width - sprite.width) as f64 {
                            sprite.position.x = (self.width - sprite.width) as f64;
                            sprite.velocity.x = 0.0;
                            sprite.acceleration.x = 0.0;
                        }

                        if sprite.position.y <= 0.0 {
                            sprite.position.y = 0.0;
                            sprite.velocity.y = 0.0;
                            sprite.acceleration.y = 0.0;
                        } else if sprite.position.y >= (self.height - sprite.height) as f64 {
                            sprite.position.y = 0.0;
                            sprite.velocity.y = 0.0;
                            sprite.acceleration.y = (self.height - sprite.height) as f64;
                        }
                    },
                    Behaviour::BounceOnBounds => {

                    },
                    Behaviour::WrapOnBounds => {

                    },
                }


            }
        }
    }

    pub fn draw_all_alive_sprites(&mut self) {
        let ref all_sprites = self.all_sprites;

        for ref sprite in all_sprites {
            if sprite.alive {
                self.canvas.draw(&sprite);
            }
        }
    }

    pub fn check_collision(&self) {
        // TODO
    }
}
