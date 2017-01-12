#![recursion_limit = "1024"]

// External crates
#[macro_use] extern crate error_chain;

extern crate simple_vector2d;

// Internal

pub mod game;
pub mod scene;
pub mod animation;
pub mod sprite;
pub mod sprite_sheet;

pub use game::Game;
pub use scene::Scene;
pub use animation::{Animation, NO_ANIMATION, ANIMATE_ONCE, ANIMATE_LOOP, ANIMATE_PINGPONG};
pub use sprite::{Sprite, SpriteBuilder};
