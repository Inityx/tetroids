use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Coord(pub i8, pub i8);

impl ops::Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Coord {
        Coord(
            self.0 + rhs.0,
            self.1 + rhs.1
        )
    }
}

impl<'a, 'b> ops::Add<&'b Coord> for &'a Coord {
    type Output = Coord;

    fn add(self, rhs: &'b Coord) -> Coord {
        Coord(
            self.0 + rhs.0,
            self.1 + rhs.1
        )
    }
}
