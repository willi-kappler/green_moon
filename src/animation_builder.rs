
use animation::{Animation, AnimationType, PingPongDirection};

error_chain! {
    errors {
        SpriteSheetsUndefined
        AnimationsUndefined
    }
}

pub struct AnimationBuilder {
    pub frames: Vec<(usize, u32)>,
    pub current_frame: usize,
    pub animation_type: AnimationType,
    pub ping_pong: PingPongDirection
}

impl AnimationBuilder {
    pub fn new() -> AnimationBuilder {
        AnimationBuilder {
            frames: Vec::new(),
            current_frame: 0,
            animation_type: AnimationType::NoAnimation,
            ping_pong: PingPongDirection::Up,
        }
    }

    pub fn animation_type(mut self, animation_type: AnimationType) -> AnimationBuilder {
        self.animation_type = animation_type;

        self
    }

    pub fn add_frames(mut self, frames: Vec<(usize, u32)>) -> AnimationBuilder {
        self.frames = frames;

        self
    }

    pub fn build(self) -> Result<Animation> {
        Ok(Animation {
            frames: self.frames,
            current_frame: self.current_frame,
            animation_type: self.animation_type,
            ping_pong: self.ping_pong
        })
    }
}
