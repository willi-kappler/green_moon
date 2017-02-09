
use sdl2::EventPump;
use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use sprite::Sprite;
use canvas::Canvas;
use resource_manager::ResourceManager;

pub struct GameObjects<'a> {
    sprites: Vec<Sprite>,
    pub canvas: Canvas<'a>,
    event_pump: EventPump,
}

impl<'a> GameObjects<'a> {
    pub fn new(renderer: Renderer, event_pump: EventPump) -> GameObjects {
        GameObjects {
            sprites: Vec::new(),
            canvas: Canvas::new(renderer),
            event_pump: event_pump,
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.sprites.push(sprite);
    }

    pub fn draw(&mut self, resource_manager: &ResourceManager) {
        for sprite in self.sprites.iter() {
            sprite.draw(&mut self.canvas, resource_manager);
        }
    }

    pub fn update(&mut self) {
        for sprite in self.sprites.iter_mut() {
            sprite.update();
        }
    }

    pub fn key_pressed(&mut self, key: Keycode) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown{keycode: Some(key), ..} => {
                    return true;
                }
                _ => {}
            }
        }

        return false;
    }

    pub fn key_released(&mut self, key: Keycode) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyUp{keycode: Some(key), ..} => {
                    return true;
                }
                _ => {}
            }
        }

        return false;
    }

    pub fn has_quit_event(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit{..} => {
                    return true;
                }
                _ => {}
            }
        }

        return false;
    }
}
