
use animation::{Animation, AnimationType, ANIMATE_NONE};

error_chain! {
    errors {
        FramesUndefined
    }
}

pub struct AnimationBuilder {
    pub frames: Vec<(usize, u32)>,
    pub animation_type: Box<AnimationType>,
}

impl AnimationBuilder {
    pub fn new() -> AnimationBuilder {
        AnimationBuilder {
            frames: Vec::new(),
            animation_type: Box::new(ANIMATE_NONE),
        }
    }

    pub fn animation_type<T: AnimationType + 'static>(mut self, animation_type: T) -> AnimationBuilder {
        self.animation_type = Box::new(animation_type);

        self
    }

    pub fn add_frame(mut self, frame:(usize, u32)) -> AnimationBuilder {
        self.frames.push(frame);

        self
    }

    pub fn add_frames(mut self, frames: &mut Vec<(usize, u32)>) -> AnimationBuilder {
        self.frames.append(frames);

        self
    }

    pub fn set_frames(mut self, frames: Vec<(usize, u32)>) -> AnimationBuilder {
        self.frames = frames;

        self
    }

    pub fn build(self) -> Result<Animation> {
        match self {
            AnimationBuilder { ref frames, .. } if frames.is_empty() => Err(ErrorKind::FramesUndefined.into()),
            _ => {
                Ok(Animation {
                    frames: self.frames,
                    animation_type: self.animation_type,
                    paused: false,
                })
            }
        }
    }
}
