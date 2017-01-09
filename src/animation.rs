
pub struct Animation {
    pub frames: Vec<(usize, u32)>,
    pub current_frame: usize,
    pub animation_type: Box<AnimationType>,
    pub paused: bool,
}

impl Animation {
    pub fn next(&mut self, dt: u32) {
        let current_frame = self.current_frame;
        let (_, frame_duration) = self.frames[current_frame];
        let last_frame = self.frames.len() - 1;

        if !self.paused {
            // Wait until frame_duration milli seconds have passed
            // before the next frame is shown.
            if dt >= frame_duration {
                self.current_frame = self.animation_type.next(current_frame, last_frame);
            }
        }
    }

    pub fn get_sprite_index(&self) -> usize {
        let (index, _) = self.frames[self.current_frame];
        index
    }

    pub fn reset(&mut self) {
        self.current_frame = 0 as usize;
    }

    pub fn set_animation_type<T: AnimationType + 'static>(&mut self, animation_type: T) {
        self.animation_type = Box::new(animation_type);
        self.reset();
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }
}

pub trait AnimationType {
    fn next(&mut self, current_frame: usize, last_frame: usize) -> usize { 0 }
}

// No animation at all, just show a single frame all the time
pub struct AnimateNone;

pub const ANIMATE_NONE : AnimateNone = AnimateNone {};

impl AnimationType for AnimateNone {}

// Animation that runs only once and then stays at the last frame.
pub struct AnimateOnce;

pub const ANIMATE_ONCE : AnimateOnce = AnimateOnce {};

impl AnimationType for AnimateOnce {
    fn next(&mut self, current_frame: usize, last_frame: usize) -> usize {
        if current_frame < last_frame {
            // Animation not done, move to the next frame
            current_frame + 1
        } else {
            // Animation finished, stay at the last frame
            current_frame
        }
    }
}

// Animation that runs from the first to the last frame and then starts all over again.
pub struct AnimateLoop;

pub const ANIMATE_LOOP : AnimateLoop = AnimateLoop {};

impl AnimationType for AnimateLoop {
    fn next(&mut self, current_frame: usize, last_frame: usize) -> usize {
        if current_frame < last_frame {
            // Animation not done, move to the next frame
            current_frame + 1
        } else {
            // Animation finished, restart with the first frame
            0
        }
    }
}

#[derive(Clone)]
enum PingPongDirection {
    Up,
    Down
}

// Animation that runs first forewards to the last frame and then backwards to the first frame
// and then repeats
pub struct AnimatePingPong {
    direction: PingPongDirection,
}

pub const ANIMATE_PING_PONG : AnimatePingPong = AnimatePingPong { direction: PingPongDirection::Up };

impl AnimationType for AnimatePingPong {
    fn next(&mut self, current_frame: usize, last_frame: usize) -> usize {
        let direction = self.direction.clone();

        match direction {
            PingPongDirection::Up => {
                if current_frame < last_frame {
                    // Animation not done, move to the next frame
                    current_frame + 1
                } else {
                    // Animation finished, move backwards
                    self.direction = PingPongDirection::Down;
                    current_frame
                }
            },
            PingPongDirection::Down => {
                if current_frame > 0 {
                    // Animation not done, move to the previous frame
                    current_frame - 1
                } else {
                    // Animation finished, move forewards
                    self.direction = PingPongDirection::Up;
                    current_frame
                }
            }
        }
    }
}
