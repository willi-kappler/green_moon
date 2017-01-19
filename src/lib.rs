#![recursion_limit = "1024"]

// External crates
#[macro_use] extern crate error_chain;
extern crate sdl2;

pub mod game;
pub mod scene;
pub mod animation;
pub mod sprite;
pub mod sprite_sheet;
pub mod resource_manager;
pub mod game_objects;
pub mod canvas;
pub mod bounding_shape;
pub mod vector2d;

pub use game::Game;
pub use game_objects::GameObjects;
pub use canvas::Canvas;
pub use scene::{Scene, SceneMessage};
pub use animation::{Animation, NO_ANIMATION, ANIMATE_ONCE, ANIMATE_LOOP, ANIMATE_PINGPONG};
pub use sprite::{Sprite, SpriteBuilder};
pub use resource_manager::ResourceManager;
pub use bounding_shape::{BoundingShape, collides};
pub use vector2d::Vector2D;
