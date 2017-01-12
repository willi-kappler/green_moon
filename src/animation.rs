
pub trait AnimationType {
    fn next(&mut self, current_frame: usize, last_frame: usize) -> usize { 0 }
}

pub struct Animation {
    frames: Vec<(usize, u16)>,
    animation_type: Box<AnimationType>,
    current_frame: usize,
    paused: bool,
}

impl Animation {
    pub fn new<T: AnimationType + 'static>(frames: Vec<(usize, u16)>, animation_type: T) -> Animation {
        Animation {
            frames: frames,
            animation_type: Box::new(animation_type),
            current_frame: 0,
            paused: false,
        }
    }

    pub fn set_animation_frames(&mut self, frames: Vec<(usize, u16)>) {
        self.frames = frames;
        self.reset();
    }

    pub fn set_animation_type<T: AnimationType + 'static>(&mut self, animation_type: T) {
        self.animation_type = Box::new(animation_type);
        self.reset();
    }

    pub fn current_sprite_frame(&self) -> usize {
        let (sprite_frame, _) = self.frames[self.current_frame];
        sprite_frame
    }

    pub fn next(&mut self, dt: u16) {
        if !self.paused {
            let (_, frame_duration) = self.frames[self.current_frame];

            if dt >= frame_duration {
                self.current_frame = self.animation_type.next(self.current_frame, self.frames.len() - 1);
            }
        }
    }

    /// Pause animation and always show the current frame without advancing when `next` is called.
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Continue animation when `next` is called.
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Resets the animation
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.paused = false;
    }
}

/// Just show one frame all the time
pub struct NoAnimation {}
impl AnimationType for NoAnimation {}
pub const NO_ANIMATION: NoAnimation = NoAnimation {};

/// Run the animation exactly one and stop at the last frame.
/// The last frame is then showed for the rest of the time.
pub struct AnimateOnce {}
impl AnimationType for AnimateOnce {
    fn next(&mut self, current_frame: usize, last_frame: usize) -> usize {
        if current_frame < last_frame {
            // Move on to the next frame
            current_frame + 1
        } else {
            // Stay with the current frame
            current_frame
        }
    }
}
pub const ANIMATE_ONCE: AnimateOnce = AnimateOnce {};

/// Run the animation from the first frame to the last and then start again with the first frame.
/// Repeat for the rest of the time.
pub struct AnimateLoop {}
impl AnimationType for AnimateLoop {
    fn next(&mut self, current_frame: usize, last_frame: usize) -> usize {
        if current_frame < last_frame {
            // Move on to the next frame
            current_frame + 1
        } else {
            // Start again from the beginning
            0
        }
    }
}
pub const ANIMATE_LOOP: AnimateLoop = AnimateLoop {};

/// Run the animation from the first frame to the last and then backwards to the first frame.
/// Repeat for the rest of the time.
pub struct AnimatePingPong {
    direction: PingPongDirection,
}
impl AnimationType for AnimatePingPong {
    fn next(&mut self, current_frame: usize, last_frame: usize) -> usize {
        match self.direction {
            PingPongDirection::Up => {
                if current_frame < last_frame {
                    // Move on to the next frame
                    current_frame + 1
                } else {
                    // Move backwards
                    self.direction = PingPongDirection::Down;
                    current_frame
                }
            },
            PingPongDirection::Down => {
                if current_frame > 0 {
                    // Move on to the next frame
                    current_frame - 1
                } else {
                    // Move backwards
                    self.direction = PingPongDirection::Up;
                    0
                }
            }
        }
    }
}
pub const ANIMATE_PINGPONG: AnimatePingPong = AnimatePingPong { direction: PingPongDirection::Up };
enum PingPongDirection {
    Up,
    Down,
}
