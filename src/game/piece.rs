#![allow(dead_code)]

use super::coord;
use super::color;
use super::Movement;

use self::Movement::*;
use self::coord::Coord;
use self::color::Color;

#[derive(Clone, Debug)]
pub struct Piece {
    pub offsets: [Coord;4],
    pub color: Color,
    rotation: bool,
    pub coord: Coord,
}

impl Piece {
    pub fn from_preset(preset: &Piece, coord: Coord) -> Piece {
        Piece {
            offsets: preset.offsets,
            color: preset.color,
            rotation: preset.rotation,
            coord: coord,
        }
    }

    pub fn real_locations(&self) -> [Coord;4] {
        self.offsets
            .iter()
            .map( |offset| offset + self.coord )
            .collect::<[Coord;4]>()
    }

    pub fn real_locations_when_moved(&self, movement: Movement) -> [Coord;4] {
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
                if self.rotation == false { return self.offsets.clone(); }
                
                let rotation_function = match movement {
                    RotRight => Coord::turn_right,
                    RotLeft  => Coord::turn_left,
                    _ => unreachable!(),
                };
                self.offsets
                    .iter()
                    .map(rotation_function)
                    .map( |offset| offset + self.coord )
                    .collect::<[Coord;4]>()
            },
        }
    }
    

    pub fn do_move(&mut self, movement: Movement) {
        match movement {
            MoveLeft | MoveRight | MoveDown => {
                let displacement = match movement {
                    MoveLeft  => Coord(-1,  0),
                    MoveRight => Coord( 1,  0),
                    MoveDown  => Coord( 0, -1),
                    _ => unreachable!(),
                };
                self.coord += displacement;
            },
            RotRight | RotLeft => {
                if self.rotation == false { return; }
                
                let rotation_function = match movement {
                    RotRight => Coord::turn_right,
                    RotLeft  => Coord::turn_left,
                    _ => unreachable!(),
                };
                self.offsets = self.offsets
                    .iter()
                    .map(rotation_function)
                    .collect::<[Coord;4]>();
            },
        }
    }
}

pub mod template {
    use ::rand;
    use self::rand::Rng;
    use super::Piece;
    use super::Coord as C;
    use super::color::named::*;

    pub const O: Piece = Piece {
        offsets: [C(0,0), C(0,1), C(1,0), C(1, 1)],
        color: YELLOW,
        coord: C(0,0),
        rotation: false,
    };
    pub const T: Piece = Piece {
        offsets: [C(-1,0), C(0,0), C(1,0), C(0,1)],
        color: PURPLE,
        coord: C(0,0),
        rotation: true,
    };
    pub const L: Piece = Piece {
        offsets: [C(0,2), C(0,1), C(0,0), C(1,0)],
        color: ORANGE,
        coord: C(0,0),
        rotation: true,
    };
    pub const J: Piece = Piece {
        offsets: [C(0,2), C(0,1), C(0,0), C(-1,0)],
        color: BLUE,
        coord: C(0,0),
        rotation: true,
    };
    pub const S: Piece = Piece {
        offsets: [C(-1,0), C(0,0), C(0,1), C(1,1)],
        color: GREEN,
        coord: C(0,0),
        rotation: true,
    };
    pub const Z: Piece = Piece {
        offsets: [C(-1,1), C(0,1), C(0,0), C(1,0)],
        color: RED,
        coord: C(0,0),
        rotation: true,
    };
    pub const I: Piece = Piece {
        offsets: [C(-1,0), C(0,0), C(1,0), C(2,0)],
        color: CYAN,
        coord: C(0,0),
        rotation: true,
    };
    
    const PIECES: [&'static Piece;7] = [&O, &T, &L, &J, &S, &Z, &I];
    
    pub fn random_at(coord: C) -> Piece {
        Piece::from_preset(
            rand::thread_rng().choose(&PIECES).unwrap(),
            coord
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Piece;
    use super::template;
    use super::Movement::*;
    use super::Coord as C;
    
    const ORIGIN: C = C(4,5);
    
    #[test]
    fn real_locations() {
        let piece = Piece::from_preset(&template::O, ORIGIN);
        let locations = piece.real_locations();
        assert_eq!(
            [C(4,5), C(4,6), C(5,5), C(5,6)],
            locations
        );
    }
    
    #[test]
    fn when_translated() {
        let piece = Piece::from_preset(&template::O, ORIGIN);
        assert_eq!(
            [C(3,5), C(3,6), C(4,5), C(4, 6)],
            piece.real_locations_when_moved(MoveLeft)
        );
    }
    
    #[test]
    fn when_rotated() {
        let piece = Piece::from_preset(&template::T, ORIGIN);
        assert_eq!(
            [C(4,4), C(4,5), C(4,6), C(3, 5)],
            piece.real_locations_when_moved(RotLeft)
        );
        assert_eq!(
            [C(4,6), C(4,5), C(4,4), C(5, 5)],
            piece.real_locations_when_moved(RotRight)
        );
    }
    
    #[test]
    fn do_translate() {
        let mut piece = Piece::from_preset(&template::O, ORIGIN);
        piece.do_move(MoveLeft);
        assert_eq!(template::O.offsets, piece.offsets);
        assert_eq!(C(3,5), piece.coord);
    }
    
    #[test]
    fn do_rotate() {
        
        let mut piece_left = Piece::from_preset(&template::T, ORIGIN);
        let mut piece_right = Piece::from_preset(&template::T, ORIGIN);
        
        piece_left.do_move(RotLeft);
        piece_right.do_move(RotRight);
        
        assert_eq!(ORIGIN, piece_left.coord);
        assert_eq!(
            [C(0,-1), C(0,0), C(0,1), C(-1, 0)],
            piece_left.offsets
        );
        
        assert_eq!(ORIGIN, piece_right.coord);
        assert_eq!(
            [C(0,1), C(0,0), C(0,-1), C(1, 0)],
            piece_right.offsets
        );
    }
}
