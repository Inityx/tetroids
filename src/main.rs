extern crate x11;

mod game;
mod gui;

fn main() {
    let mut game = game::Game::new();
    let mut interface = unsafe { gui::GUI::new() };
    unsafe { interface.play(&mut game); }
}
