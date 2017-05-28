#![allow(dead_code)]

use super::color::Color;
use super::piece::Piece;
use super::aux::Coord;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
pub const BOARD_BORDER: &str = "+--------------------+";
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
        // clear full lines and return number of lines cleared
        0
    }

    pub fn print(&self) {
        println!("{}", BOARD_BORDER);

        for row in self.data.iter().rev() {
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
