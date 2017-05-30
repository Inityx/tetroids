#![allow(dead_code)]

use super::color::Color;
use super::piece::Piece;
use super::coord::Coord;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
const BOARD_TOP: &str    = "╭────────────────────╮";
const BOARD_BOTTOM: &str = "╰────────────────────╯";

pub const INSERTION_POINT: Coord = Coord(
    (BOARD_WIDTH as i8) / 2 - 1,
    (BOARD_HEIGHT as i8) - 1
);

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
        let mut board = self.data;
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

    pub fn print(&self, cursor_option: Option<&Piece>) {
        println!("{}", BOARD_TOP);

        let cursor_locations = if let Some(cursor) = cursor_option {
            Some(cursor.real_locations())
        } else {
            None
        };
        
        for (row_index, row) in self.data.iter().enumerate().rev() {
            let row_string = row.iter().enumerate().map( |(col_index, square)| {
                let current_loc = Coord(col_index as i8, row_index as i8);

                let cursor_on = if let Some(locations) = cursor_locations {
                    locations.iter().any( |location| location == current_loc)
                } else {
                    false
                };
                
                if cursor_on || square.is_some() { "▓▓" } else { "  " }
            }).collect::<String>();

            println!("│{}│", row_string);
            
        }

        println!("{}", BOARD_BOTTOM);
    }
}
