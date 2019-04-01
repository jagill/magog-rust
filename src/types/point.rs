use crate::types::{Coordinate, Envelope, Geometry, Position};

#[derive(Debug, PartialEq)]
pub struct Point<C: Coordinate>(pub Position<C>);

/// Turn a `Position`-ish object into a `Point`.
impl<C: Coordinate, IP: Into<Position<C>>> From<IP> for Point<C> {
    fn from(p: IP) -> Self {
        Point(p.into())
    }
}

impl<C: Coordinate> Point<C> {
    pub fn new(position: Position<C>) -> Point<C> {
        Point(position)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        self.0.validate()
    }

    pub fn x(&self) -> C {
        self.0.x
    }

    pub fn y(&self) -> C {
        self.0.y
    }
}

// GEOMETRY implementation
impl<C: Coordinate> Point<C> {
    pub fn dimension(&self) -> u8 {
        0
    }

    pub fn geometry_type(&self) -> &'static str {
        "Point"
    }

    pub fn envelope(&self) -> Envelope<C> {
        Envelope::from((self.0, self.0))
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn is_simple(&self) -> bool {
        match self.validate() {
            Err(_) => false,
            Ok(_) => true,
        }
    }

    pub fn boundary(&self) -> Geometry<C> {
        Geometry::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32;

    #[test]
    fn check_constructor() {
        let p = Point(Position { x: 0.1, y: 1.0 });
        assert_eq!(p.x(), 0.1);
        assert_eq!(p.y(), 1.0);
    }

    #[test]
    fn check_is_simple() {
        let p = Point(Position { x: 0.1, y: 1.0 });
        assert!(p.is_simple());
    }

    #[test]
    fn check_is_not_simple() {
        let p = Point(Position {
            x: 0.1,
            y: f32::NAN,
        });
        assert!(!p.is_simple());
    }

}
