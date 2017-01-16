#![recursion_limit = "1024"]

// External crates
#[macro_use] extern crate error_chain;
extern crate simple_vector2d;
extern crate sdl2;



type Vec2 = simple_vector2d::Vector2<f64>;


pub mod game;
pub mod scene;
pub mod animation;
pub mod sprite;
pub mod sprite_sheet;
pub mod resource_manager;
pub mod game_objects;
pub mod canvas;
pub mod bounding_shape;

pub use game::Game;
pub use game_objects::GameObjects;
pub use canvas::Canvas;
pub use scene::{Scene, SceneMessage};
pub use animation::{Animation, NO_ANIMATION, ANIMATE_ONCE, ANIMATE_LOOP, ANIMATE_PINGPONG};
pub use sprite::{Sprite, SpriteBuilder};
pub use resource_manager::ResourceManager;
pub use bounding_shape::BoundingShape;
