
// use game::Game;
use canvas::Canvas;
use game::Game;

pub trait Scene {
    fn enter(&mut self, canvas: &mut Game) {}

    fn leave(&mut self, canvas: &mut Game) {}

    fn update(&mut self, game: &mut Game) -> bool { false }

    fn draw(&mut self, canvas: &mut Game) {}
}
