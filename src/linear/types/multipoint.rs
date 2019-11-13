use crate::linear::primitives::{Envelope, HasEnvelope, Position};
use crate::linear::types::Point;
use crate::Coordinate;

#[derive(Debug, PartialEq)]
pub struct MultiPoint<C: Coordinate> {
    pub positions: Vec<Position<C>>,
    _envelope: Envelope<C>,
}

impl<C: Coordinate> HasEnvelope<C> for MultiPoint<C> {
    fn envelope(&self) -> Envelope<C> {
        self._envelope
    }
}

// Vec<Into<Position>> -> MultiPoint
impl<C: Coordinate, IP: Into<Position<C>>> From<Vec<IP>> for MultiPoint<C> {
    fn from(positions: Vec<IP>) -> MultiPoint<C> {
        MultiPoint::new(positions.into_iter().map(|p| p.into()).collect())
    }
}

impl<C: Coordinate> MultiPoint<C> {
    pub fn empty() -> Self {
        MultiPoint {
            positions: Vec::new(),
            _envelope: Envelope::empty(),
        }
    }

    pub fn new(positions: Vec<Position<C>>) -> Self {
        let _envelope: Envelope<C> = Envelope::of(positions.iter());
        MultiPoint {
            positions,
            _envelope,
        }
    }

    pub fn num_points(&self) -> usize {
        self.positions.len()
    }

    pub fn get_point(&self, n: usize) -> Option<Point<C>> {
        let position = self.positions.get(n)?;
        Some(Point::new(*position))
    }
}
