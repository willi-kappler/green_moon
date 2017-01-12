
pub trait Scene {
    fn enter(&mut self) {}

    fn leave(&mut self) {}

    fn update(&mut self) -> bool { false }

    fn draw(&mut self) {}
}
