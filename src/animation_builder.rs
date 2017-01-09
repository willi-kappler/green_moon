
use animation::{Animation, AnimationType, ANIMATE_NONE, ANIMATE_ONCE, ANIMATE_LOOP, ANIMATE_PING_PONG};

error_chain! {
    errors {
        AnimationsUndefined
    }
}

pub struct AnimationBuilder {
    pub frames: Vec<(usize, u32)>,
    pub current_frame: usize,
    pub animation_type: Box<AnimationType>,
}

impl AnimationBuilder {
    pub fn new() -> AnimationBuilder {
        AnimationBuilder {
            frames: Vec::new(),
            current_frame: 0,
            animation_type: Box::new(ANIMATE_NONE),
        }
    }

    pub fn animation_type(mut self, animation_type: AnimationType) -> AnimationBuilder {
        self.animation_type = animation_type;

        self
    }

    pub fn add_frame(mut self, frame:(usize, u32)) -> AnimationBuilder {
        self.frames.push(frame);

        self
    }

    pub fn add_frames(mut self, frames: Vec<(usize, u32)>) -> AnimationBuilder {
        self.frames.append(frames);

        self
    }

    pub fn build(self) -> Result<Animation> {
        Ok(Animation {
            frames: self.frames,
            current_frame: self.current_frame,
            animation_type: self.animation_type,
        })
    }
}
