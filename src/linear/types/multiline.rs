use crate::linear::primitives::{Envelope, HasEnvelope, Segment};
use crate::linear::types::Line;
use crate::Coordinate;

#[derive(Debug, PartialEq)]
pub struct MultiLine<C: Coordinate> {
    segments: Vec<Segment<C>>,
    _envelope: Envelope<C>,
}

impl<C: Coordinate> HasEnvelope<C> for MultiLine<C> {
    fn envelope(&self) -> Envelope<C> {
        self._envelope
    }
}

// Vec<Into<Segment>> -> MultiLine
impl<C: Coordinate, IS: Into<Segment<C>>> From<Vec<IS>> for MultiLine<C> {
    fn from(segments: Vec<IS>) -> MultiLine<C> {
        MultiLine::new(segments.into_iter().map(|s| s.into()).collect())
    }
}

impl<C: Coordinate> MultiLine<C> {
    pub fn empty() -> Self {
        MultiLine {
            segments: Vec::new(),
            _envelope: Envelope::empty(),
        }
    }

    pub fn new(segments: Vec<Segment<C>>) -> Self {
        let _envelope = Envelope::of(segments.iter());
        MultiLine {
            segments,
            _envelope,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn get_line(&self, n: usize) -> Option<Line<C>> {
        let segment = self.segments.get(n)?;
        Some(Line::new(segment.start, segment.end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32;

    #[test]
    fn test_empty_multiline() {
        let empty = MultiLine::<f32>::empty();
        assert!(empty.is_empty());
        assert!(empty.envelope().is_empty());
        assert_eq!(empty.get_line(0), None);
    }

    #[test]
    fn test_nonempty_multiline() {
        let segments = vec![
            Segment::from((0., 0.)),
            Segment::from((1., 2.)),
            Segment::from((-2., -3.)),
        ];
        let multiline = MultiLine::new(segments);
        assert!(!multiline.is_empty());
        assert_eq!(multiline.envelope(), Envelope::from((-3., 2.)));
        assert_eq!(multiline.get_line(0).unwrap(), Line::from((0., 0.)));
        assert_eq!(multiline.get_line(1).unwrap(), Line::from((1., 2.)));
        assert_eq!(multiline.get_line(2).unwrap(), Line::from((-2., -3.)));
        assert_eq!(multiline.get_line(3), None);
    }
}
