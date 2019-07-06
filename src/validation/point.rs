use crate::primitives::Coordinate;
use crate::types::Point;

impl<C: Coordinate> Point<C> {
    pub fn validate(&self) -> Result<(), &'static str> {
        self.0.validate()
    }
}
