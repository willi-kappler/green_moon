
use vector2d::Vector2D;

pub struct Sprite {
    // General
    pub alive: bool,
    pub id: u32,
    pub group: u32,

    pub current_behaviour: usize,

    // Physics
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub acceleration: Vector2D,

    // Shape and size
    pub width: u32,
    pub height: u32,

    // Animation
    pub current_sprite_sheet: usize,
    pub current_animation: usize,
    pub current_animation_frame: usize,
}

impl Sprite {
    pub fn update(&mut self, dt: u32) {
        if self.alive {
            self.position += self.velocity;
            self.velocity += self.acceleration;
            // Update animation frame in game object.
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

    pub fn set_behaviour(&mut self, new_behaviour: usize) {
        self.current_behaviour = new_behaviour;
    }

    pub fn set_sprite_sheet(&mut self, new_sprite_sheet: usize) {
        self.current_sprite_sheet = new_sprite_sheet;
    }

    pub fn set_animation(&mut self, new_animation: usize) {
        self.current_animation = new_animation;
    }
}
