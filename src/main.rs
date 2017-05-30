extern crate x11;

mod game;
mod gui;

fn main() {
    let mut g = game::Game::new();
    g.play();
}
