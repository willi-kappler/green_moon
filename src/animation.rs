
pub trait AnimationType {
    fn next(&mut self) {}
}

pub struct Animation {
    frames: Vec<(usize, u16)>,
    animation_type: Box<AnimationType>,
    paused: bool,
    current_frame: usize,
}

impl Animation {
    pub fn new<T: AnimationType + 'static>(frames: Vec<(usize, u16)>, animation_type: T) -> Animation {
        Animation {
            frames: frames,
            animation_type: Box::new(animation_type),
            paused: false,
            current_frame: 0,
        }
    }
}

pub struct NoAnimation {}
impl AnimationType for NoAnimation {
}
pub const NO_ANIMATION: NoAnimation = NoAnimation {};

pub struct AnimateLoop {}
impl AnimationType for AnimateLoop {
    fn next(&mut self) {
    }
}
pub const ANIMATE_LOOP: AnimateLoop = AnimateLoop {};
