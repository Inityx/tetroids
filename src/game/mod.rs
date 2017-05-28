mod color;
mod piece;
mod board;
mod aux;

use self::board::Board;
use self::piece::Piece;

pub struct Game {
    score: u32,
    board: Board,
    cursor: Option<Piece>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            score: 0,
            board: Board::new(),
            cursor: None,
        }
    }

    pub fn print(&self) {
        println!("\n  Score: {}\n", self.score);
        self.board.print();
    }

    pub fn tick(&mut self) {
        
    }
}
