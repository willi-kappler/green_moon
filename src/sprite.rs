
use simple_vector2d::Vector2;
use simple_vector2d::consts::ZERO_F64;

use animation::{Animation, AnimationType, NO_ANIMATION};
use game::Game;
use canvas::Canvas;
use resource_manager::ResourceManager;

pub struct SpriteBuilder {
    group: u32,
    pos: Vector2<f64>,
    vel: Vector2<f64>,
    animation: Animation,
    sprite_sheet: usize,
    alive: bool,
}

impl SpriteBuilder {
    pub fn new() -> SpriteBuilder {
        SpriteBuilder {
            group: 0,
            pos: ZERO_F64,
            vel: ZERO_F64,
            animation: Animation::new(vec![(0, 0)], NO_ANIMATION),
            sprite_sheet: 0,
            alive: true,
        }
    }

    pub fn group(mut self, group: u32) -> Self {
        self.group = group;
        self
    }

    pub fn pos(mut self, x: f64, y: f64) -> Self {
        self.pos = Vector2(x, y);
        self
    }

    pub fn animation(mut self, animation: Animation) -> Self {
        self.animation = animation;
        self
    }

    pub fn sprite_sheet(mut self, sprite_sheet: usize) -> Self {
        self.sprite_sheet = sprite_sheet;
        self
    }

    pub fn alive(mut self, alive: bool) -> Self {
        self.alive = alive;
        self
    }

    pub fn build(self) -> Sprite {
        Sprite {
            group: self.group,
            pos: self.pos,
            vel: self.vel,
            animation: self.animation,
            sprite_sheet: self.sprite_sheet,
            alive: self.alive,
        }
    }

}

pub struct Sprite {
    group: u32,
    pos: Vector2<f64>,
    vel: Vector2<f64>,
    animation: Animation,
    sprite_sheet: usize,
    alive: bool,
}

impl Sprite {
    pub fn set_animation_frames(&mut self, frames: Vec<(usize, u16)>) {
        self.animation.set_animation_frames(frames);
    }

    pub fn set_animation_type<T: AnimationType + 'static>(&mut self, animation_type: T) {
        self.animation.set_animation_type(animation_type);
    }

    pub fn update(&mut self) {
        // TODO
    }

    pub fn draw(&self, canvas: &mut Canvas, resource_manager: &ResourceManager) {
        // TODO
    }
}
