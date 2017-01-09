
use game::Game;
use sprite::Sprite;


pub trait Behaviour {
    fn update(&mut self, sprite: &mut Sprite, game: &Game) {}
}

pub struct BounceAtScreen;

pub const BOUNCE_AT_SCREEN: BounceAtScreen = BounceAtScreen {};

impl Behaviour for BounceAtScreen {
    fn update(&mut self, sprite: &mut Sprite, game: &Game) {

    }
}
