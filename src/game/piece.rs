#![allow(dead_code)]

use super::coord;
use super::color;
use super::Movement;

use self::Movement::*;
use self::coord::Coord;
use self::color::Color;

#[derive(Debug, Copy, Clone)]
pub enum Rotation { OnBlock, BetweenBlocks }

#[derive(Clone, Debug)]
pub struct Piece {
    pub offsets: [Coord;4],
    pub color: Color,
    rotation: Rotation,
    pub coord: Coord,
}

impl Piece {
    pub fn from_preset(preset: &Piece, coord: Coord) -> Piece {
        Piece {
            offsets: preset.offsets.clone(),
            color: preset.color,
            rotation: preset.rotation,
            coord: coord,
        }
    }

    pub fn real_locations(&self) -> [Coord;4] {
        self.offsets.iter().map( |offset| offset + self.coord ).collect::<[Coord;4]>()
    }

    pub fn offsets_when_moved(&self, movement: Movement) -> [Coord;4] {
        match movement {
            MoveLeft | MoveRight | MoveDown => {
                let displacement = match movement {
                    MoveLeft  => Coord(-1,  0),
                    MoveRight => Coord( 1,  0),
                    MoveDown  => Coord( 0, -1),
                    _ => unreachable!(),
                };
                self.offsets
                    .iter()
                    .map( |offset| offset + self.coord + displacement )
                    .collect::<[Coord;4]>()
            },
            RotRight | RotLeft => {
                self.offsets.clone()
            },
        }
    }
    

    pub fn do_move(&mut self, movement: Movement) {
        match movement {
            MoveLeft | MoveRight | MoveDown => {
                let displacement = match movement {
                    MoveLeft => Coord(-1, 0),
                    MoveRight => Coord(1, 0),
                    MoveDown => Coord(0, -1),
                    _ => unreachable!(),
                };
                self.coord += displacement;
            },
            RotRight | RotLeft => {
                
            },
        }
    }
}

pub mod template {
    use super::Piece;
    use super::Coord as C;
    use super::color::named::*;
    use super::Rotation::*;

    pub const SQUARE: Piece = Piece {
        offsets: [C(0,0), C(0,1), C(1,0), C(1, 1)],
        color: YELLOW,
        coord: C(0,0),
        rotation: BetweenBlocks,
    };
    pub const TEE: Piece = Piece {
        offsets: [C(-1,0), C(0,0), C(1,0), C(0,-1)],
        color: PURPLE,
        coord: C(0,0),
        rotation: OnBlock,
    };

    pub fn random_at(coord: C) -> Piece {
        Piece::from_preset(&SQUARE, coord)
    }
}
