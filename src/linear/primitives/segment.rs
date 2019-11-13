use crate::linear::primitives::{Envelope, HasEnvelope, Position};
use crate::Coordinate;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Segment<C: Coordinate> {
    pub start: Position<C>,
    pub end: Position<C>,
}

impl<C: Coordinate> HasEnvelope<C> for Segment<C> {
    fn envelope(&self) -> Envelope<C> {
        return Envelope::new(self.start, self.end);
    }
}

impl<C: Coordinate> Segment<C> {
    pub fn new(start: Position<C>, end: Position<C>) -> Segment<C> {
        Segment { start, end }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        &self.start.validate()?;
        &self.end.validate()?;
        Ok(())
    }

    pub fn min(&self) -> Position<C> {
        self.start.min(self.end)
    }

    pub fn max(&self) -> Position<C> {
        self.start.max(self.end)
    }
}
