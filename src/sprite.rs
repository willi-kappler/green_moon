
use vector2d::Vector2D;
use sprite_sheet::SpriteSheet;
use animation::Animation;

pub enum Behaviour {
    None,
    StopOnBounds,
    BounceOnBounds,
    WrapOnBounds,
}

pub struct Sprite<'a> {
    // General
    pub alive: bool,
    pub id: u32,
    pub group: u32,
    pub behaviour: Behaviour,

    // Physics
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub acceleration: Vector2D,

    // Shape and size
    pub width: u32,
    pub height: u32,

    // Animation
    pub sprite_sheets: Vec<&'a SpriteSheet>,
    pub current_sprite_sheet: usize,
    pub animations: Vec<Animation>,
    pub current_animation: usize,

}

impl<'a> Sprite<'a> {
    pub fn update(&mut self, dt: u32) {
        if self.alive {
            self.position += self.velocity;
            self.velocity += self.acceleration;
            self.animations[self.current_animation].next(dt);
        }
    }

    pub fn draw(&self) {
        if self.alive {
            // TODO
        }
    }

    pub fn move_to(&mut self, new_position: Vector2D) {
        self.position = new_position;
    }

    pub fn move_to_xy(&mut self, x: f64, y: f64) {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn move_by(&mut self, offset: Vector2D) {
        self.position += offset;
    }

    pub fn move_by_xy(&mut self, offset_x: f64, offset_y: f64) {
        self.position.x += offset_x;
        self.position.y += offset_y;
    }

    pub fn set_velocity(&mut self, new_velocity: Vector2D) {
        self.velocity = new_velocity;
    }

    pub fn set_velocity_xy(&mut self, vx: f64, vy: f64) {
        self.velocity.x = vx;
        self.velocity.y = vy;
    }

    // TODO acceleration
}
