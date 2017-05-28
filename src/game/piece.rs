#![allow(dead_code)]

use super::aux;
use super::color;

pub struct Piece {
    pub offsets: [aux::Offset;4],
    pub color: color::Color,
    pub coord: aux::Coord,
}

impl Piece {
    fn preset(offsets: [aux::Offset;4], color: color::Color) -> Piece {
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

    pub fn rotate(&mut self) {
        
    }
}

pub mod template {
    use super::Piece;
    use super::aux::Offset as O;
    use super::aux::Coord as C;
    use super::color::named::*;

    pub const SQUARE: Piece = Piece { offsets: [O(0,0), O(0,1), O(1,0), O(1, 1)], color: YELLOW, coord: C(0,0) };
    pub const TEE:    Piece = Piece { offsets: [O(0,0), O(0,1), O(1,0), O(0,-1)], color: PURPLE, coord: C(0,0) };
}
