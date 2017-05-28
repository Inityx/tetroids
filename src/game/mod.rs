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

    fn can_sink_piece(&self) -> bool {
        if let Some(ref cursor) = self.cursor {
            cursor.offsets.iter()
                .map( |offset| offset + &cursor.coord )
                .all( |location|
                    self.board.get(
                        location.0 as usize,
                        location.1 as usize
                    ).is_none()
                )
        } else {
            false
        }
    }

    pub fn play(&mut self) {
        
    }
}
