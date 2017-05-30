use std::ops;
use std::cmp;
use std::iter;

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl<'a> ops::Add<Coord> for &'a Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Coord {
        Coord(
            self.0 + rhs.0,
            self.1 + rhs.1
        )
    }
}

impl ops::AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl iter::FromIterator<Coord> for [Coord;4] {
    fn from_iter<I: IntoIterator<Item=Coord>>(iter: I) -> Self {
        let mut retval = [Coord(0,0);4];
        
        for (ret, src) in retval.iter_mut().zip(iter) {
            *ret = src;
        }
        
        retval
    }
}

impl<'a> cmp::PartialEq<Coord> for &'a Coord {
    fn eq(&self, rhs: &Coord) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1
    }
}

#[cfg(test)]
mod tests {
    use super::Coord as C;
    
    #[test]
    fn sanity() {
        let coord = C(4,5);
        assert_eq!(4, coord.0);
        assert_eq!(5, coord.1);
    }
    
    #[test]
    fn equality() {
        let coord1 = C(4,5);
        let coord2 = C(4,5);
        assert_eq!(coord1, coord2);
    }
    
    #[test]
    fn equality_ref() {
        let coord1 = C(4,5);
        let coord2 = C(4,5);
        let coord1_ref: &C = &coord1;
        assert!(coord1_ref == coord2);
    }

    #[test]
    fn add_plain() {
        let coord1 = C(1,2);
        let coord2 = C(3,4);
        let coord3 = coord1 + coord2;
        assert_eq!(C(4,6), coord3)
    }
    
    #[test]
    fn add_plain_to_ref() {
        let coord1 = C(1,2);
        let coord1ref = &coord1;
        let coord2 = C(3,4);
        let coord3 = coord1ref + coord2;
        assert_eq!(C(4,6), coord3)
    }
    
    #[test]
    fn add_assign_plain() {
        let mut coord = C(1,2);
        coord += C(4,5);
        assert_eq!(C(5,7), coord);
    }
    
    #[test]
    fn collect_arr4() {
        let src = [C(2,3), C(4,5), C(6,7), C(8,9)];
        let res = src
            .iter()
            .map( |coord| coord + C(1,1) )
            .collect::<[C;4]>();
        assert_eq!(
            [C(3,4), C(5,6), C(7,8), C(9,10)],
            res
        )
    }
}
