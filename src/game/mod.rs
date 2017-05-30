mod color;
mod piece;
mod board;
mod coord;

use self::board::Board;
use self::coord::Coord;

use std::thread;
use std::time;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Movement {
    MoveLeft,
    MoveRight,
    MoveDown,
    
    RotLeft,
    RotRight,
}

use self::Movement::*;

pub struct Game {
    score: u32,
    board: Board,
    cursor: Option<piece::Piece>,
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
        let cursor = match self.cursor {
            Some(ref cursor) => cursor.coord,
            None => Coord(0,0)
        };
        
        println!("\n  Score: {}\n  Cursor: {:?}\n", self.score, cursor);
        self.board.print(self.cursor.as_ref());
    }
    
    fn can_move_cursor(&mut self, movement: Movement) -> bool {
        self.cursor
            .as_ref()
            .unwrap()
            .offsets_when_moved(movement)
            .iter()
            .all( |location|
                location.0 >= 0 &&
                location.1 >= 0 &&
                (location.0 as usize) < board::BOARD_WIDTH &&
                (
                    (location.1 as usize) >= board::BOARD_HEIGHT ||
                    self.board.get(
                        location.0 as usize,
                        location.1 as usize
                    ).is_none()
                )
            )
    }
    
    pub fn try_move_cursor(&mut self, movement: Movement) -> Result<(),()> {
        if !self.can_move_cursor(movement) { return Err(()); }
        
        self.cursor
            .as_mut()
            .unwrap()
            .do_move(movement);
        
        Ok(())
    }
    
    pub fn place_cursor(&mut self) {
        self.board.place(self.cursor.as_ref().unwrap());
        self.cursor = None;
    }
    
    pub fn refill_cursor(&mut self) {
        if self.cursor.is_some() {
            panic!("Tried to refill cursor when it already has a piece.");
        }
        
        self.cursor = Some(
            piece::template::random_at(
                board::INSERTION_POINT
            )
        );
    }
    
    pub fn play(&mut self) {
        loop {
            self.refill_cursor();
            loop {
                self.print();
                thread::sleep(time::Duration::from_millis(500));
                
                if self.try_move_cursor(MoveDown).is_err() {
                    println!("Unable to sink piece; placing...");
                    self.place_cursor();
                    break;
                }
            }
        }
    }
}
