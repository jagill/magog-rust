mod coordinate;
mod envelope;
mod rect;
mod segment;
mod triangle;

pub use crate::types::primitive::{
    coordinate::Coordinate,
    envelope::Envelope,
    rect::Rect,
    segment::{PointLocation, Segment, SegmentIntersection},
    triangle::Triangle,
};
pub use crate::types::CoordinateType;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_coordinate() {
        let c = Coordinate::new(0.1, 1.0);
        assert_eq!(c.x, 0.1);
        assert_eq!(c.y, 1.0);
    }

    #[test]
    fn check_segment() {
        let s = Segment::new(Coordinate::from((0.0, 1.0)), Coordinate::from((2.0, 3.0)));
        assert_eq!(s.start, Coordinate::new(0.0, 1.0));
        assert_eq!(s.end, Coordinate::new(2.0, 3.0));
    }

    #[test]
    fn check_triangle() {
        let t = Triangle::new(
            Coordinate::from((0.0, 0.0)),
            Coordinate::from((0.0, 1.0)),
            Coordinate::from((1.0, 0.0)),
        );
        assert_eq!(
            t.to_array(),
            [
                Coordinate::new(0.0, 0.0),
                Coordinate::new(0.0, 1.0),
                Coordinate::new(1.0, 0.0)
            ]
        );
    }

    #[test]
    fn check_rect() {
        let r = Rect::new(Coordinate::from((0.0, 0.1)), Coordinate::from((1.0, 1.1)));
        assert_eq!(r.min.x, 0.0);
        assert_eq!(r.min.y, 0.1);
        assert_eq!(r.max.x, 1.0);
        assert_eq!(r.max.y, 1.1);
    }

    #[test]
    fn check_envelope() {
        let e = Envelope::new(Some(Rect::from(((0.0, 0.1), (1.0, 1.1)))));
        let r = e.rect.unwrap();
        assert_eq!(r.min.x, 0.0);
        assert_eq!(r.min.y, 0.1);
        assert_eq!(r.max.x, 1.0);
        assert_eq!(r.max.y, 1.1);
    }

}
