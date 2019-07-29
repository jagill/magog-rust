mod coordinate;
mod envelope;
mod position;
mod rect;
mod segment;
mod triangle;

pub use crate::primitives::{
    coordinate::Coordinate,
    envelope::{Envelope, HasEnvelope},
    position::{Position, SafePosition},
    rect::Rect,
    segment::{PositionLocation, Segment, SegmentIntersection},
    triangle::Triangle,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_coordinate() {
        let c = Position::new(0.1, 1.0);
        assert_eq!(c.x, 0.1);
        assert_eq!(c.y, 1.0);
    }

    #[test]
    fn check_segment() {
        let s = Segment::from(((0.0, 1.0), (2.0, 3.0)));
        assert_eq!(s.start, Position::new(0.0, 1.0));
        assert_eq!(s.end, Position::new(2.0, 3.0));
    }

    #[test]
    fn check_triangle() {
        let t = Triangle::new(
            Position::from((0.0, 0.0)),
            Position::from((0.0, 1.0)),
            Position::from((1.0, 0.0)),
        );
        assert_eq!(
            t.to_array(),
            [
                Position::new(0.0, 0.0),
                Position::new(0.0, 1.0),
                Position::new(1.0, 0.0)
            ]
        );
    }

    #[test]
    fn check_rect() {
        let r = Rect::new(Position::from((0.0, 0.1)), Position::from((1.0, 1.1)));
        assert_eq!(r.min.x, 0.0);
        assert_eq!(r.min.y, 0.1);
        assert_eq!(r.max.x, 1.0);
        assert_eq!(r.max.y, 1.1);
    }

    #[test]
    fn check_envelope() {
        let e = Envelope::new(Some(Rect::from(((0.0, 0.1), (1.0, 1.1)))));
        match e {
            Envelope::Empty => panic!("Envelope should not be empty"),
            Envelope::Bounds(r) => {
                assert_eq!(r.min.x, 0.0);
                assert_eq!(r.min.y, 0.1);
                assert_eq!(r.max.x, 1.0);
                assert_eq!(r.max.y, 1.1);
            }
        }
    }

}
