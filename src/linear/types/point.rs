use crate::linear::primitives::{Envelope, HasEnvelope, Position};
use crate::Coordinate;

#[derive(Debug, PartialEq)]
pub struct Point<C: Coordinate>(pub Position<C>);

impl<C: Coordinate> HasEnvelope<C> for Point<C> {
    fn envelope(&self) -> Envelope<C> {
        if self.is_empty() {
            Envelope::empty()
        } else {
            self.0.envelope()
        }
    }
}

/// Turn a `Position`-ish object into a `Point`.
impl<C: Coordinate, P: Into<Position<C>>> From<P> for Point<C> {
    fn from(p: P) -> Self {
        Point(p.into())
    }
}

impl<C: Coordinate> Point<C> {
    pub fn empty() -> Self {
        Point(Position::new(C::nan()))
    }

    pub fn new(position: Position<C>) -> Self {
        Point(position)
    }

    pub fn is_empty(&self) -> bool {
        C::is_nan(self.0.x)
    }

    pub fn x(&self) -> Option<C> {
        if self.is_empty() {
            None
        } else {
            Some(self.0.x)
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use core::f32;

    #[test]
    fn test_empty_point() {
        let empty_point = Point::<f32>::empty();
        assert!(empty_point.is_empty());
        assert_eq!(empty_point.x(), None);
        assert!(empty_point.envelope().is_empty());
    }

    #[test]
    fn test_point() {
        let point = Point::from(1.);
        assert!(!point.is_empty());
        assert_eq!(point.x().unwrap(), 1.);
        assert_eq!(point.envelope(), Envelope::from((1., 1.)));
    }
}
