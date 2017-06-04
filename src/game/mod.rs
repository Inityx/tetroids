#![allow(dead_code)]

pub mod color;
mod piece;
mod board;
mod coord;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Movement {
    MoveLeft,
    MoveRight,
    MoveDown,
    
    RotLeft,
    RotRight,
}

use self::Movement::*;

#[derive(Debug)]
enum Selection {
    Cursor,
    Projection,
}

pub struct Game {
    pub score: u32,
    board: board::Board,
    cursor: Option<piece::Piece>,
    projection: Option<piece::Piece>,
    prev_cursor_x: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            score: 0,
            board: board::Board::new(),
            cursor: None,
            projection: None,
            prev_cursor_x: 0,
        }
    }

    fn can_move_piece(&mut self, movement: Movement, selection: Selection) -> bool {
        let piece = match selection {
            Selection::Cursor     => self.cursor.as_ref(),
            Selection::Projection => self.projection.as_ref(),
        };
        
        if piece.is_none() {
            panic!("Tried to evaluate move for nonexistent {:?}.", selection);
        }

        piece.unwrap()
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
        if !self.can_move_piece(movement, Selection::Cursor) {
            return match movement {
                MoveDown => {
                    self.place_cursor();
                    Ok(())
                },
                _ => Err(())
            };
        }

        self.cursor.as_mut().unwrap().do_move(movement);
        if movement != MoveDown { self.project_cursor(); }

        Ok(())
    }
    
    pub fn project_cursor(&mut self) {
        self.projection = self.cursor.clone();
        while self.can_move_piece(MoveDown, Selection::Projection) {
            self.projection.as_mut().unwrap().do_move(MoveDown);
        }
    }

    pub fn place_cursor(&mut self) {
        self.board.place(self.projection.as_ref().unwrap());
        self.prev_cursor_x = self.cursor.as_ref().unwrap().coord.0 as usize;
        self.cursor = None;
        self.projection = None;
    }
    
    pub fn refill_cursor(&mut self) {
        if self.cursor.is_some() {
            panic!("Tried to refill cursor when it already has a piece.");
        }
        
        self.cursor = Some(
            piece::template::random_at(
                coord::Coord(self.prev_cursor_x as i8, board::INSERTION_POINT)
            )
        );

        self.project_cursor();
    }
    
    pub fn evaluate_score(&mut self) -> bool {
        let add = self.board.clear_lines();
        if add > 0 {
            self.score += add as u32;
            true
        } else {
            false
        }
    }
    
    pub fn board_iter_with_index(&self) -> board::IterWithIndex {
        self.board.iter_with_index()
    }
    
    pub fn get_cursor(&self) -> Option<&piece::Piece> {
        self.cursor.as_ref()
    }
    
    pub fn get_projection(&self) -> Option<&piece::Piece> {
        self.projection.as_ref()
    }
}
