use crate::planar::primitives::{Envelope, HasEnvelope, Position};
use crate::Coordinate;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Segment<C: Coordinate> {
    pub start: Position<C>,
    pub end: Position<C>,
}

impl<C: Coordinate> HasEnvelope<C> for Segment<C> {
    fn envelope(&self) -> Envelope<C> {
        Envelope::new(self.start, self.end)
    }
}

/// Location of a point in relation to a line
#[derive(PartialEq, Clone, Debug)]
pub enum PositionLocation {
    Left,
    On,
    Right,
}

/// Intersection type of two segments.
/// Two segments can be disjoint, intersect at a point, or overlap in a segment.
#[derive(PartialEq, Clone, Debug)]
pub enum SegmentIntersection<C: Coordinate> {
    None,
    Position(Position<C>),
    Segment(Segment<C>),
}

// (Position, Position) -> Segment
impl<C: Coordinate, IP: Into<Position<C>>> From<(IP, IP)> for Segment<C> {
    fn from(positions: (IP, IP)) -> Self {
        Segment {
            start: positions.0.into(),
            end: positions.1.into(),
        }
    }
}

impl<C: Coordinate> Segment<C> {
    pub fn new(start: Position<C>, end: Position<C>) -> Self {
        Segment { start, end }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        self.start.validate()?;
        self.end.validate()?;
        Ok(())
    }

    pub fn length_squared(&self) -> C {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        dx * dx + dy * dy
    }

    pub fn length(&self) -> C {
        self.length_squared().sqrt()
    }

    /// Tests if a positions is Left|On|Right of the infinite line determined by the segment.
    ///    Return: PositionLocation for location of p relative to [start, end]
    pub fn position_location(&self, position: Position<C>) -> PositionLocation {
        let test = Position::cross(self.end - self.start, position - self.start);
        if test > C::zero() {
            PositionLocation::Left
        } else if test == C::zero() {
            PositionLocation::On
        } else {
            PositionLocation::Right
        }
    }

    /// Determinant of segment
    pub fn determinant(&self) -> C {
        self.start.x * self.end.y - self.start.y * self.end.x
    }

    pub fn contains(self, p: Position<C>) -> bool {
        self.envelope().contains(p) && self.position_location(p) == PositionLocation::On
    }

    /**
     * Check the intersection of two segments.
     *
     * NB: This does not do an initial check with Envelopes; the caller should do that.
     */
    pub fn intersect_segment(&self, other: Segment<C>) -> SegmentIntersection<C> {
        // check intersection
        if self == &other {
            return SegmentIntersection::Segment(*self);
        }

        let da = self.end - self.start; // The vector for the segment
        let db = other.end - other.start; // The vector for the other segment
        let offset = other.start - self.start; // The offset between segments (starts)

        let da_x_db = Position::cross(da, db);
        let offset_x_da = Position::cross(offset, da);

        if da_x_db == C::zero() {
            // This means the two segments are parallel.
            // If the offset is not also parallel, they must be disjoint.
            if offset_x_da != C::zero() {
                return SegmentIntersection::None;
            } else {
                // If the offset is also parallel, check for overlap.
                let da_2 = Position::dot(da, da);
                // Offset, in units of da.
                let t0 = Position::dot(offset, da) / da_2;
                // self.start to other.end, in units of da.
                let t1 = t0 + Position::dot(da, db) / da_2;
                let (t_min, t_max) = t0.min_max(t1);
                if t_min > C::one() || t_max < C::zero() {
                    // if min(t0, t1) > 1 or max(t0, t1) < 0, they don't intersect.
                    return SegmentIntersection::None;
                } else {
                    // Else, the intersect
                    let start = self.start + da * t_min.max(C::zero());
                    let end = self.start + da * t_max.min(C::one());
                    if start == end {
                        return SegmentIntersection::Position(start);
                    } else {
                        return SegmentIntersection::Segment(Segment::new(start, end));
                    }
                }
            }
        } else {
            // The segments are not parallel, so they are disjoint or intersect at a point
            // Calculate where the infinite lines would intersect; if these are on the segments
            // then the segments intersect.
            let ta = Position::cross(offset, db) / da_x_db;
            let tb = offset_x_da / da_x_db;
            if C::zero() <= ta && ta <= C::one() && C::zero() <= tb && tb <= C::one() {
                return SegmentIntersection::Position(self.start + da * ta);
            }
        }
        SegmentIntersection::None
    }

    /**
     * The winding number is calculated as follows:
     * 1. pos.y must be between the top of the segment (exclusive) and the bottom
     *    of the segment (inclusive).
     *   A. This condition excludes all horizontal segments.
     * 2. The intersection of the ray from pos to x=+inf with the segment must be
     *    strictly to the right of pos (isxn.x > pos.x).
     *
     */
    pub(crate) fn find_winding_number(position: Position<C>, seg: Segment<C>) -> i32 {
        let (seg_min, seg_max) = Position::min_max(seg.start, seg.end);
        let upward = seg.end.y > seg.start.y;

        if position.y >= seg_max.y || position.y < seg_min.y {
            return 0;
        }

        match (seg.position_location(position), upward) {
            // have a valid up intersect
            (PositionLocation::Left, true) => -1,
            // have a valid down intersect
            (PositionLocation::Right, false) => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn check_basic_segment_f32() {
        let start_x: f32 = 1.;
        let start_y: f32 = 2.;
        let end_x: f32 = 3.;
        let end_y: f32 = 4.;
        let s = Segment {
            start: Position {
                x: start_x,
                y: start_y,
            },
            end: Position { x: end_x, y: end_y },
        };
        assert_eq!(s.start.x, start_x);
        assert_eq!(s.start.y, start_y);
        assert_eq!(s.end.x, end_x);
        assert_eq!(s.end.y, end_y);
    }

    #[test]
    fn check_basic_segment_f64() {
        let start_x: f64 = 1.;
        let start_y: f64 = 2.;
        let end_x: f64 = 3.;
        let end_y: f64 = 4.;
        let s = Segment {
            start: Position {
                x: start_x,
                y: start_y,
            },
            end: Position { x: end_x, y: end_y },
        };
        assert_eq!(s.start.x, start_x);
        assert_eq!(s.start.y, start_y);
        assert_eq!(s.end.x, end_x);
        assert_eq!(s.end.y, end_y);
    }

    #[test]
    fn check_segment_equals() {
        let s1 = Segment {
            start: Position { x: 1., y: 2. },
            end: Position { x: 3., y: 4. },
        };
        let s2 = Segment {
            start: Position { x: 1., y: 2. },
            end: Position { x: 3., y: 4. },
        };
        assert_eq!(s1, s2);
    }

    #[test]
    fn check_segment_not_equals() {
        let s1 = Segment {
            start: Position { x: 1., y: 2.1 },
            end: Position { x: 3., y: 4. },
        };
        let s2 = Segment {
            start: Position { x: 1., y: 2.2 },
            end: Position { x: 3., y: 4. },
        };
        assert_ne!(s1, s2);
    }

    #[test]
    fn check_envelope() {
        let s = Segment::from(((0.0, 2.0), (1.0, 3.0)));
        let e = s.envelope();
        assert!(!e.is_empty());
        let min_p = e.min().unwrap();
        let max_p = e.max().unwrap();
        assert_eq!(min_p.x, 0.0);
        assert_eq!(min_p.y, 2.0);
        assert_eq!(max_p.x, 1.0);
        assert_eq!(max_p.y, 3.0);
    }

    // Intersection tests
    /////////

    #[test]
    fn check_intersect_segment_self() {
        let s1 = Segment::from(((0.0, 0.0), (1.0, 1.0)));
        let s2 = Segment::from(((0.0, 0.0), (1.0, 1.0)));
        assert_eq!(
            s1.intersect_segment(s2),
            SegmentIntersection::Segment(((0.0, 0.0), (1.0, 1.0)).into())
        );
    }

    #[test]
    fn check_intersect_segment_skew_disjoint() {
        let s1 = Segment::from(((0.0, 0.0), (1.0, 1.0)));
        let s2 = Segment::from(((1.0, 0.0), (0.5, 0.4)));
        assert_eq!(s1.intersect_segment(s2), SegmentIntersection::None,);
    }

    #[test]
    fn check_intersect_segment_parallel_disjoint() {
        let s1 = Segment::from(((0.0, 0.0), (1.0, 0.0)));
        let s2 = Segment::from(((0.0, 1.0), (1.0, 1.0)));
        assert_eq!(s1.intersect_segment(s2), SegmentIntersection::None);
    }

    #[test]
    fn check_intersect_segment_endpoint() {
        let s1 = Segment::from(((0.0, 0.0), (1.0, 0.0)));
        let s2 = Segment::from(((1.0, 0.0), (1.0, 1.0)));
        assert_eq!(
            s1.intersect_segment(s2),
            SegmentIntersection::Position((1.0, 0.0).into())
        );
    }

    #[test]
    fn check_intersect_segment_midpoint() {
        let s1 = Segment::from(((0.0, 0.0), (1.0, 1.0)));
        let s2 = Segment::from(((1.0, 0.0), (0.0, 1.0)));
        assert_eq!(
            s1.intersect_segment(s2),
            SegmentIntersection::Position((0.5, 0.5).into())
        );
    }

    #[test]
    fn check_intersect_segment_colinear_disjoint() {
        let s1 = Segment::from(((0.0, 0.0), (1.0, 1.0)));
        let s2 = Segment::from(((1.1, 1.1), (2.0, 2.0)));
        assert_eq!(s1.intersect_segment(s2), SegmentIntersection::None);
    }

    #[test]
    fn check_intersect_segment_colinear_half() {
        let s1 = Segment::from(((0.0, 0.0), (1.0, 1.0)));
        let s2 = Segment::from(((0.5, 0.5), (2.0, 2.0)));
        assert_eq!(
            s1.intersect_segment(s2),
            SegmentIntersection::Segment(((0.5, 0.5), (1.0, 1.0)).into())
        );
    }

    #[test]
    fn check_intersect_segment_colinear_half_antiparallel() {
        let s1 = Segment::from(((0.0, 0.0), (1.0, 1.0)));
        let s2 = Segment::from(((2.0, 2.0), (0.5, 0.5)));
        assert_eq!(
            s1.intersect_segment(s2),
            SegmentIntersection::Segment(((0.5, 0.5), (1.0, 1.0)).into())
        );
    }

    #[test]
    fn check_intersect_segment_colinear_contained() {
        let s1 = Segment::from(((0.0, 0.0), (1.0, 1.0)));
        let s2 = Segment::from(((0.2, 0.2), (0.5, 0.5)));
        assert_eq!(
            s1.intersect_segment(s2),
            SegmentIntersection::Segment(((0.2, 0.2), (0.5, 0.5)).into())
        );
    }

    #[test]
    fn check_degenerate_segment_intersection() {
        let s1 = Segment::from(((0.0, 0.0), (1.0, 0.0)));
        let s2 = Segment::from(((1.0, 0.0), (2.0, 0.0)));
        assert_eq!(
            s1.intersect_segment(s2),
            SegmentIntersection::Position((1.0, 0.0).into())
        );
    }
}
