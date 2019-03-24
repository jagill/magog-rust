use crate::types::primitive::{Coord2, CoordinateType, Rect};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Segment<T>
where
    T: CoordinateType,
{
    pub start: Coord2<T>,
    pub end: Coord2<T>,
}

/// Location of a point in relation to a line
#[derive(PartialEq, Clone, Debug)]
pub enum PointLocation {
    Left,
    On,
    Right,
}

/// Intersection type of two segments.
/// Two segments can be disjoint, intersect at a point, or overlap in a segment.
#[derive(PartialEq, Clone, Debug)]
pub enum SegmentIntersection<T: CoordinateType> {
    None,
    Coord2(Coord2<T>),
    Segment(Segment<T>),
}

// (T, T) -> Segment
impl<T: CoordinateType, IC: Into<Coord2<T>>> From<(IC, IC)> for Segment<T> {
    fn from(coords: (IC, IC)) -> Self {
        Segment {
            start: coords.0.into(),
            end: coords.1.into(),
        }
    }
}

// Segment -> Rect
impl<T: CoordinateType> From<Segment<T>> for Rect<T> {
    fn from(seg: Segment<T>) -> Self {
        Rect::from((seg.start, seg.end))
    }
}

impl<T: CoordinateType> Segment<T> {
    pub fn new(start: Coord2<T>, end: Coord2<T>) -> Segment<T> {
        Segment { start, end }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        &self.start.validate()?;
        &self.end.validate()?;
        Ok(())
    }

    pub fn length_squared(&self) -> T {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        dx * dx + dy * dy
    }

    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    /// Tests if a coordinate is Left|On|Right of the infinite line determined by the segment.
    ///    Return: PointLocation for location of c relative to [start, end]
    pub fn coord_position(&self, coord: Coord2<T>) -> PointLocation {
        let test = Coord2::cross(self.end - self.start, coord - self.start);
        if test > T::zero() {
            PointLocation::Left
        } else if test == T::zero() {
            PointLocation::On
        } else {
            PointLocation::Right
        }
    }

    /// Determinant of segment
    pub fn determinant(&self) -> T {
        self.start.x * self.end.y - self.start.y * self.end.x
    }

    pub fn contains(self, c: Coord2<T>) -> bool {
        Rect::from(self).contains(c) && self.coord_position(c) == PointLocation::On
    }

    /**
     * Check the intersection of two segments.
     *
     * NB: This does not do an initial check with Envelopes; the caller should do that.
     */
    pub fn intersect_segment(&self, other: Segment<T>) -> SegmentIntersection<T> {
        // check intersection
        if self == &other {
            return SegmentIntersection::Segment(*self);
        }

        let da = self.end - self.start; // The vector for the segment
        let db = other.end - other.start; // The vector for the other segment
        let offset = other.start - self.start; // The offset between segments (starts)

        let da_x_db = Coord2::cross(da, db);
        let offset_x_da = Coord2::cross(offset, da);

        if da_x_db == T::zero() {
            // This means the two segments are parallel.
            // If the offset is not also parallel, they must be disjoint.
            if offset_x_da != T::zero() {
                return SegmentIntersection::None;
            } else {
                // If the offset is also parallel, check for overlap.
                let da_2 = Coord2::dot(da, da);
                // Offset, in units of da.
                let t0 = Coord2::dot(offset, da) / da_2;
                // self.start to other end, in units of da.
                let t1 = t0 + Coord2::dot(da, db) / da_2;
                let (t_min, t_max) = Coord2::min_max(t0, t1);
                if t_min > T::one() || t_max < T::zero() {
                    // if min(t0, t1) > 1 or max(t0, t1) < 0, they don't intersect.
                    return SegmentIntersection::None;
                } else {
                    // Else, the intersect
                    return SegmentIntersection::Segment(Segment::new(
                        self.start + da * t_min.max(T::zero()),
                        self.start + da * t_max.min(T::one()),
                    ));
                }
            }
        } else {
            // The segments are not parallel, so they are disjoint or intersect at a point
            // Calculate where the infinite lines would intersect; if these are on the segments
            // then the segments intersect.
            let ta = Coord2::cross(offset, db) / da_x_db;
            let tb = offset_x_da / da_x_db;
            if T::zero() <= ta && ta <= T::one() && T::zero() <= tb && tb <= T::one() {
                return SegmentIntersection::Coord2(self.start + da * ta);
            }
        }
        SegmentIntersection::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_basic_segment_f32() {
        let start_x: f32 = 1.;
        let start_y: f32 = 2.;
        let end_x: f32 = 3.;
        let end_y: f32 = 4.;
        let s = Segment {
            start: Coord2 {
                x: start_x,
                y: start_y,
            },
            end: Coord2 { x: end_x, y: end_y },
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
            start: Coord2 {
                x: start_x,
                y: start_y,
            },
            end: Coord2 { x: end_x, y: end_y },
        };
        assert_eq!(s.start.x, start_x);
        assert_eq!(s.start.y, start_y);
        assert_eq!(s.end.x, end_x);
        assert_eq!(s.end.y, end_y);
    }

    #[test]
    fn check_segment_equals() {
        let s1 = Segment {
            start: Coord2 { x: 1., y: 2. },
            end: Coord2 { x: 3., y: 4. },
        };
        let s2 = Segment {
            start: Coord2 { x: 1., y: 2. },
            end: Coord2 { x: 3., y: 4. },
        };
        assert_eq!(s1, s2);
    }

    #[test]
    fn check_segment_not_equals() {
        let s1 = Segment {
            start: Coord2 { x: 1., y: 2.1 },
            end: Coord2 { x: 3., y: 4. },
        };
        let s2 = Segment {
            start: Coord2 { x: 1., y: 2.2 },
            end: Coord2 { x: 3., y: 4. },
        };
        assert_ne!(s1, s2);
    }

    #[test]
    fn check_to_rect() {
        let s = Segment::from(((0.0, 2.0), (1.0, 3.0)));
        let e = Rect::from(s);
        assert_eq!(e.min.x, 0.0);
        assert_eq!(e.min.y, 2.0);
        assert_eq!(e.max.x, 1.0);
        assert_eq!(e.max.y, 3.0);
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
            SegmentIntersection::Coord2((1.0, 0.0).into())
        );
    }

    #[test]
    fn check_intersect_segment_midpoint() {
        let s1 = Segment::from(((0.0, 0.0), (1.0, 1.0)));
        let s2 = Segment::from(((1.0, 0.0), (0.0, 1.0)));
        assert_eq!(
            s1.intersect_segment(s2),
            SegmentIntersection::Coord2((0.5, 0.5).into())
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

}
