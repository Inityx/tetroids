#![allow(dead_code)]

use super::color::Color;
use super::piece::Piece;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
const BOARD_BORDER: &str = "+--------------------+";

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

    fn set(&mut self, row: usize, col: usize, board_square: BoardSquare) {
        self.data[row][col] = Some(board_square);
    }

    pub fn get(&self, row: usize, col: usize) -> Option<BoardSquare> {
        self.data[row][col]
    }

    pub fn place(&mut self, piece: Piece) {
        for square_offset in piece.offsets.iter() {
            self.set(
                (square_offset.0 + piece.coord.0) as usize,
                (square_offset.1 + piece.coord.1) as usize,
                BoardSquare(piece.color)
            );
        }
    }
    
    pub fn clear_lines(&mut self) -> u8 {
        // clear full lines and return number of lines cleared
        0
    }

    pub fn print(&self) {
        println!("{}", BOARD_BORDER);

        for row in self.data.iter() {
            let row_string = row.iter().map( |square|
                match square {
                    &Some(_) => "##",
                    &None => "  "
                }
            ).collect::<String>();

            println!("|{}|", row_string);
        }

        println!("{}", BOARD_BORDER);
    }
}

