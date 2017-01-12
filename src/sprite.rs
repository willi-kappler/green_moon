use simple_vector2d::Vector2;
use simple_vector2d::consts::ZERO_F64;

use animation::{Animation, NO_ANIMATION};

pub struct SpriteBuilder {
    group: u32,
    pos: Vector2<f64>,
    vel: Vector2<f64>,
    animation: Box<Animation>,
    sprite_sheet: usize,
    alive: bool,
}

pub struct Sprite {
    group: u32,
    pos: Vector2<f64>,
    vel: Vector2<f64>,
    animation: Box<Animation>,
    sprite_sheet: usize,
    alive: bool,
}

impl SpriteBuilder {
    pub fn new() -> SpriteBuilder {
        SpriteBuilder {
            group: 0,
            pos: ZERO_F64,
            vel: ZERO_F64,
            animation: Box::new(Animation::new(vec![], NO_ANIMATION)),
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
        self.animation = Box::new(animation);
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
