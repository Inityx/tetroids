#![allow(dead_code)]

use super::aux;
use super::color;

pub struct Piece {
    pub offsets: [aux::Coord;4],
    pub color: color::Color,
    pub coord: aux::Coord,
}

impl Piece {
    fn preset(offsets: [aux::Coord;4], color: color::Color) -> Piece {
        Piece {
            offsets: offsets,
            color: color,
            coord: aux::Coord(0,0)
        }
    }

    pub fn from_preset(preset: &Piece, coord: aux::Coord) -> Piece {
        Piece {
            offsets: preset.offsets.clone(),
            color: preset.color,
            coord: coord
        }
    }

    pub fn sink(&mut self) {
        self.coord.1 -= 1;
    }

    pub fn rotate(&mut self) {
        
    }
}

pub mod template {
    use super::Piece;
    use super::aux::Coord as C;
    use super::color::named::*;

    pub const SQUARE: Piece = Piece { offsets: [C(0,0), C(0,1), C(1,0), C(1, 1)], color: YELLOW, coord: C(0,0) };
    pub const TEE:    Piece = Piece { offsets: [C(0,0), C(0,1), C(1,0), C(0,-1)], color: PURPLE, coord: C(0,0) };

    pub fn random_at(coord: C) -> Piece {
        Piece::from_preset(&SQUARE, coord)
    }
}
