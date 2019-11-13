use crate::linear::primitives::{Envelope, HasEnvelope, Position, Segment};
use crate::linear::types::Point;
use crate::Coordinate;

#[derive(Debug, PartialEq)]
pub struct Line<C: Coordinate> {
    segment: Option<Segment<C>>,
}

impl<C: Coordinate> HasEnvelope<C> for Line<C> {
    fn envelope(&self) -> Envelope<C> {
        match self.segment {
            None => Envelope::empty(),
            Some(seg) => seg.envelope(),
        }
    }
}

impl<C: Coordinate, IS: Into<Segment<C>>> From<IS> for Line<C> {
    fn from(segment: IS) -> Self {
        let seg: Segment<C> = segment.into();
        Line::new(seg.start, seg.end)
    }
}

impl<C: Coordinate> Line<C> {
    pub fn empty() -> Self {
        Line { segment: None }
    }

    pub fn new(start: Position<C>, end: Position<C>) -> Self {
        Line {
            segment: Some(Segment::new(start, end)),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.segment.is_none()
    }

    pub fn start(&self) -> Option<Point<C>> {
        let seg = self.segment?;
        Some(Point::new(seg.start))
    }

    pub fn end(&self) -> Option<Point<C>> {
        let seg = self.segment?;
        Some(Point::new(seg.end))
    }
}
