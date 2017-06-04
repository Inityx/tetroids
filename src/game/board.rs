#![allow(dead_code)]

use super::color::Color;
use super::piece::Piece;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

pub const INSERTION_POINT: i8 = BOARD_HEIGHT as i8 - 1;

#[derive(Debug, Copy, Clone)]
pub struct BoardSquare(Color);

pub struct Board {
    data: [[Option<BoardSquare>; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Board {
        Board {
            data: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    fn set(&mut self, x: usize, y: usize, board_square: BoardSquare) {
        self.data[y][x] = Some(board_square);
    }

    pub fn get(&self, x: usize, y: usize) -> Option<BoardSquare> {
        self.data[y][x]
    }

    pub fn place(&mut self, piece: &Piece) {
        for square_offset in piece.offsets.iter() {
            let location = square_offset + piece.coord;
            
            self.set(
                location.0 as usize,
                location.1 as usize,
                BoardSquare(piece.color)
            );
        }
    }
    
    pub fn clear_lines(&mut self) -> u8 {
        let mut board = &mut self.data;
        let mut found = 0;
        
        // Collapse full lines
        for index in 0..board.len() {
            if board[index].iter().all( |item| item.is_some() ) {
                found += 1;
            } else if found > 0 {
                board[index - found] = board[index];
            }
        }
        
        // Fill in top with None
        for index in (board.len() - found)..board.len() {
            board[index] = [None::<BoardSquare>; BOARD_WIDTH];
        }
        
        found as u8
    }
    
    pub fn iter_with_index<'a>(&'a self) -> IterWithIndex<'a> {
        IterWithIndex::over(&self)
    }
}

pub struct IterWithIndex<'a> {
    x: usize,
    y: usize,
    board: &'a Board,
}

impl<'a> IterWithIndex<'a> {
    fn over(board: &'a Board) -> IterWithIndex<'a> {
        IterWithIndex {
            x: 0,
            y: 0,
            board: board,
        }
    }
}

impl<'a> Iterator for IterWithIndex<'a> {
    type Item = (usize, usize, Option<BoardSquare>);
    
    fn next(&mut self) -> Option<(usize, usize, Option<BoardSquare>)> {
        if self.y >= BOARD_HEIGHT { return None; }
        
        let retval = Some(
            (
                self.x,
                self.y,
                self.board.get(self.x, self.y)
            )
        );
        
        if self.x < BOARD_WIDTH-1 {
            self.x += 1
        } else {
            self.x = 0;
            self.y += 1
        }
        retval
    }
}
