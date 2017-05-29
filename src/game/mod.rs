mod color;
mod piece;
mod board;
mod aux;

use std::thread;
use std::time;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Down
}

use self::Direction::*;

pub struct Game {
    score: u32,
    board: board::Board,
    cursor: Option<piece::Piece>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            score: 0,
            board: board::Board::new(),
            cursor: None,
        }
    }

    pub fn print(&self) {
        let cursor = match self.cursor {
            Some(ref cursor) => cursor.coord,
            None => aux::Coord(0,0)
        };
        
        println!("\n  Score: {}\n  Cursor: {:?}\n", self.score, cursor);
        self.board.print();
    }
    
    fn can_move_cursor(&mut self, direction: Direction) -> bool {
        let cursor = match self.cursor {
            Some(ref cursor) => cursor,
            None => return false
        };
        
        let displacement = aux::Coord(
            match direction { Left => -1, Right => 1, Down => 0 },
            match direction { Down => -1, _ => 0 }
        );
        
        cursor.offsets.iter()
            .map( |offset| offset + cursor.coord + displacement )
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
    
    fn try_move_cursor(&mut self, direction: Direction) -> Result<(),()> {
        if !self.can_move_cursor(direction) { return Err(()); }
        
        let cursor = self.cursor.as_mut().unwrap();
        match direction {
            Left =>  cursor.coord.0 -= 1,
            Right => cursor.coord.0 += 1,
            Down =>  cursor.coord.1 -= 1
        }
        
        Ok(())
    }
    
    fn try_place_cursor(&mut self) -> Result<(),()> {
        if let Some(ref cursor) = self.cursor {
            self.board.place(cursor);
        } else {
            return Err(());
        }
        
        self.cursor = None;
        Ok(())
    }
    
    pub fn play(&mut self) {
        loop {
            self.cursor = Some(
                piece::template::random_at(board::INSERTION_POINT),
            );
            loop {
                self.print();
                thread::sleep(time::Duration::from_millis(500));
                
                if self.try_move_cursor(Down).is_err() {
                    println!("Unable to sink piece; placing...");
                    self.try_place_cursor().unwrap();
                    break;
                }
            }
        }
    }
}
