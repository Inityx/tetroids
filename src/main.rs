extern crate x11;
extern crate rand;

mod game;
mod gui;

fn main() {
    let mut game = game::Game::new();
    let mut interface = gui::GUI::new();
    interface.play(&mut game);
}
