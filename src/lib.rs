#![recursion_limit = "1024"]

// External crates
#[macro_use]
extern crate error_chain;

extern crate sdl2;

// Internal
pub mod game_builder;
pub mod game_manager;
pub mod game;
pub mod canvas;
pub mod scene_manager;
pub mod scene;
pub mod vector2d;
pub mod sprite;
pub mod sprite_builder;
pub mod sprite_sheet;
pub mod animation;
pub mod animation_builder;

pub use game_builder::GameBuilder;
pub use game_manager::GameManager;
pub use game::Game;
pub use canvas::Canvas;
pub use scene_manager::SceneManager;
pub use scene::Scene;
pub use sprite::Sprite;
pub use sprite_builder::SpriteBuilder;
pub use vector2d::Vector2D;
pub use sprite_sheet::SpriteSheet;
pub use animation::{Animation, AnimationType};
pub use animation_builder::AnimationBuilder;

// From SDL2

pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
