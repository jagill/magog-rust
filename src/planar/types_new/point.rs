use crate::planar::primitives::{Envelope, HasEnvelope, Position};
use crate::planar::types_new::{Empty, Geometric};
use crate::Coordinate;

#[derive(Debug, PartialEq)]
pub struct Point<C: Coordinate>(pub Position<C>);

impl<C: Coordinate> HasEnvelope<C> for Point<C> {
    fn envelope(&self) -> Envelope<C> {
        self.0.envelope()
    }
}

/// Turn a `Position`-ish object into a `Point`.
// impl<C: Coordinate, P: Into<Position<C>>> From<P> for Point<C> {
//     fn from(p: P) -> Self {
//         Point(p.into())
//     }
// }

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
impl<C: Coordinate> Geometric<C> for Point<C> {
    fn dimension(&self) -> u8 {
        0
    }

    fn geometry_type(&self) -> &'static str {
        "Point"
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn boundary(&self) -> Empty<C> {
        Empty::empty()
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use core::f32;

    #[test]
    fn check_constructor() {
        let p = Point(Position { x: 0.1, y: 1.0 });
        assert_eq!(p.x(), 0.1);
        assert_eq!(p.y(), 1.0);
    }
}
