
use sprite::Sprite;
use vector2d::Vector2D;

pub struct SpriteBuilder {
    // General
    alive: bool,
    id: u32,
    group: u32,
    current_behaviour: usize,

    // Physics
    position: Vector2D,
    velocity: Vector2D,
    acceleration: Vector2D,

    // Shape and size
    width: u32,
    height: u32,

    // Animation
    current_sprite_sheet: usize,
    current_animation: usize,
}

impl SpriteBuilder {
    pub fn new() -> SpriteBuilder {
        SpriteBuilder {
            alive: false,
            id: 0,
            group: 0,
            current_behaviour: 0,

            position: Vector2D { x: 0.0, y: 0.0 },
            velocity: Vector2D { x: 0.0, y: 0.0 },
            acceleration: Vector2D { x: 0.0, y: 0.0 },

            width: 0,
            height: 0,

            current_sprite_sheet: 0,
            current_animation: 0,
        }
    }

    pub fn alive(mut self, alive: bool) -> SpriteBuilder {
        self.alive = alive;

        self
    }

    pub fn id(mut self, id: u32) -> SpriteBuilder {
        self.id = id;

        self
    }

    pub fn group(mut self, group: u32) -> SpriteBuilder {
        self.group = group;

        self
    }

    // TODO; position, velocity and acceleration with Vector2D

    pub fn position_xy(mut self, x: f64, y: f64) -> SpriteBuilder {
        self.position.x = x;
        self.position.y = y;

        self
    }

    pub fn velocity_xy(mut self, x: f64, y: f64) -> SpriteBuilder {
        self.velocity.x = x;
        self.velocity.y = y;

        self
    }

    pub fn acceleration_xy(mut self, x: f64, y: f64) -> SpriteBuilder {
        self.acceleration.x = x;
        self.acceleration.y = y;

        self
    }

    pub fn sprite_sheet(mut self, sprite_sheet: usize) -> SpriteBuilder {
        self.current_sprite_sheet = sprite_sheet;

        self
    }

    pub fn animation(mut self, animation: usize) -> SpriteBuilder {
        self.current_animation = animation;

        self
    }

    pub fn behaviour(mut self, behaviour: usize) -> SpriteBuilder {
        self.current_behaviour = behaviour;

        self
    }

    pub fn build(self) -> Sprite {
        Sprite {
            alive: self.alive,
            id: self.id,
            group: self.group,
            current_behaviour: self.current_behaviour,

            position: self.position,
            velocity: self.velocity,
            acceleration: self.acceleration,

            width: self.width,
            height: self.height,

            current_sprite_sheet: self.current_sprite_sheet,
            current_animation: self.current_animation,
            current_animation_frame: 0,
        }
    }
}
