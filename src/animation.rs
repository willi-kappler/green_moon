
#[derive(Clone)]
pub enum AnimationType {
    NoAnimation,
    SingleRun,
    Loop,
    PingPong
}

#[derive(Clone)]
pub enum PingPongDirection {
    Up,
    Down
}

#[derive(Clone)]
pub struct Animation {
    pub frames: Vec<(usize, u32)>,
    pub current_frame: usize,
    pub animation_type: AnimationType,
    pub ping_pong: PingPongDirection
}

impl Animation {
    pub fn next(&mut self) {
        match self.animation_type {
            AnimationType::NoAnimation => {},
            AnimationType::SingleRun => {
                if self.current_frame < self.frames.len() - 1 {
                    self.current_frame += 1;
                }
            },
            AnimationType::Loop => {
                self.current_frame += 1;
                if self.current_frame >= self.frames.len() {
                    self.current_frame = 0;
                }
            },
            AnimationType::PingPong => {
                match self.ping_pong {
                    PingPongDirection::Up => {
                        self.current_frame += 1;
                        if self.current_frame >= self.frames.len() {
                            self.current_frame = self.frames.len() - 1;
                            self.ping_pong = PingPongDirection::Down;
                        }
                    },
                    PingPongDirection::Down => {
                        self.current_frame -= 1;
                        if self.current_frame <= 0 {
                            self.current_frame = 0;
                            self.ping_pong = PingPongDirection::Up;
                        }
                    }
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.current_frame = 0 as usize;
        self.ping_pong = PingPongDirection::Up;
    }

    pub fn set_animation_type(&mut self, animation_type: AnimationType) {
        self.animation_type = animation_type;
        self.reset();
    }
}
