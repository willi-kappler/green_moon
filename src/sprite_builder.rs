
use sprite::{Sprite, Behaviour};
use vector2d::Vector2D;
use sprite_sheet::SpriteSheet;
use animation::{Animation};
use animation_builder::{AnimationBuilder};

error_chain! {
    errors {
        SpriteSheetsUndefined
        AnimationsUndefined
    }
}

pub struct SpriteBuilder<'a> {
    // General
    alive: bool,
    id: u32,
    group: u32,
    behaviour: Behaviour,

    // Physics
    position: Vector2D,
    velocity: Vector2D,
    acceleration: Vector2D,

    // Shape and size
    width: u32,
    height: u32,

    // Animation
    sprite_sheets: Vec<&'a SpriteSheet>,
    current_sprite_sheet: usize,
    animations: Vec<Animation>,
    current_animation: usize,
}

impl<'a> SpriteBuilder<'a> {
    pub fn new() -> SpriteBuilder<'a> {
        SpriteBuilder {
            alive: false,
            id: 0,
            group: 0,
            behaviour: Behaviour::None,

            position: Vector2D { x: 0.0, y: 0.0 },
            velocity: Vector2D { x: 0.0, y: 0.0 },
            acceleration: Vector2D { x: 0.0, y: 0.0 },

            width: 0,
            height: 0,

            sprite_sheets: Vec::new(),
            current_sprite_sheet: 0,
            animations: Vec::new(),
            current_animation: 0,
        }
    }

    pub fn alive(mut self, alive: bool) -> SpriteBuilder<'a> {
        self.alive = alive;

        self
    }

    pub fn id(mut self, id: u32) -> SpriteBuilder<'a> {
        self.id = id;

        self
    }

    pub fn group(mut self, group: u32) -> SpriteBuilder<'a> {
        self.group = group;

        self
    }

    // TODO; position, velocity and acceleration with Vector2D

    pub fn position_xy(mut self, x: f64, y: f64) -> SpriteBuilder<'a> {
        self.position.x = x;
        self.position.y = y;

        self
    }

    pub fn velocity_xy(mut self, x: f64, y: f64) -> SpriteBuilder<'a> {
        self.velocity.x = x;
        self.velocity.y = y;

        self
    }

    pub fn acceleration_xy(mut self, x: f64, y: f64) -> SpriteBuilder<'a> {
        self.acceleration.x = x;
        self.acceleration.y = y;

        self
    }

    pub fn add_sprite_sheet(mut self, sprite_sheet: &'a SpriteSheet) -> SpriteBuilder<'a> {
        self.sprite_sheets.push(sprite_sheet);

        self
    }

    pub fn add_animation(mut self, animation: Animation) -> SpriteBuilder<'a> {
        self.animations.push(animation);

        self
    }

    pub fn no_animation(self) -> SpriteBuilder<'a> {
        let animation = AnimationBuilder::new().add_frame((0, 0));
        self.add_animation(animation.build().unwrap())
    }

    pub fn build(self) -> Result<Sprite<'a>> {
        match self {
            SpriteBuilder { ref sprite_sheets, .. } if sprite_sheets.is_empty() => Err(ErrorKind::SpriteSheetsUndefined.into()),
            SpriteBuilder { ref animations, .. } if animations.is_empty() => Err(ErrorKind::AnimationsUndefined.into()),
            _ => Ok(Sprite {
                    alive: self.alive,
                    id: self.id,
                    group: self.group,
                    behaviour: self.behaviour,

                    position: self.position,
                    velocity: self.velocity,
                    acceleration: self.acceleration,

                    width: self.width,
                    height: self.height,

                    sprite_sheets: self.sprite_sheets,
                    current_sprite_sheet: self.current_sprite_sheet,
                    animations: self.animations,
                    current_animation: self.current_animation,
            })
        }
    }
}
