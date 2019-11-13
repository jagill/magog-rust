use crate::planar::types::Point;
use crate::Coordinate;

impl<C: Coordinate> Point<C> {
    pub fn validate(&self) -> Result<(), &'static str> {
        self.0.validate()
    }
}
