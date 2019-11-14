use crate::planar::primitives::{Envelope, HasEnvelope, Position};
use crate::planar::types::Geometry;
use crate::Coordinate;

#[derive(Debug, PartialEq)]
pub struct Point<C: Coordinate>(pub Position<C>);

impl<C: Coordinate> HasEnvelope<C> for Point<C> {
    fn envelope(&self) -> Envelope<C> {
        Envelope::new(self.0, self.0)
    }
}

/// Turn a `Position`-ish object into a `Point`.
impl<C: Coordinate, P: Into<Position<C>>> From<P> for Point<C> {
    fn from(p: P) -> Self {
        Point(p.into())
    }
}

impl<C: Coordinate> Point<C> {
    pub fn new(position: Position<C>) -> Point<C> {
        Point(position)
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

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn is_simple(&self) -> bool {
        self.validate().is_ok()
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
