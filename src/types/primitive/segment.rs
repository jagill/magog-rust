use crate::types::primitive::{Coordinate, CoordinateType, Rect};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Segment<T>
where
    T: CoordinateType,
{
    pub start: Coordinate<T>,
    pub end: Coordinate<T>,
}

/// Location of a point in relation to a line
#[derive(PartialEq, Clone, Debug)]
pub enum PointLocation {
    Left,
    On,
    Right,
}

// (T, T) -> Segment
impl<T: CoordinateType, IC: Into<Coordinate<T>>> From<(IC, IC)> for Segment<T> {
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
    pub fn new(start: Coordinate<T>, end: Coordinate<T>) -> Segment<T> {
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
    pub fn coord_position(&self, coord: Coordinate<T>) -> PointLocation {
        let test = Coordinate::cross(self.end - self.start, coord - self.start);
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

    pub fn contains(self, c: Coordinate<T>) -> bool {
        Rect::from(self).contains(c) && self.coord_position(c) == PointLocation::On
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
            start: Coordinate {
                x: start_x,
                y: start_y,
            },
            end: Coordinate { x: end_x, y: end_y },
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
            start: Coordinate {
                x: start_x,
                y: start_y,
            },
            end: Coordinate { x: end_x, y: end_y },
        };
        assert_eq!(s.start.x, start_x);
        assert_eq!(s.start.y, start_y);
        assert_eq!(s.end.x, end_x);
        assert_eq!(s.end.y, end_y);
    }

    #[test]
    fn check_segment_equals() {
        let s1 = Segment {
            start: Coordinate { x: 1., y: 2. },
            end: Coordinate { x: 3., y: 4. },
        };
        let s2 = Segment {
            start: Coordinate { x: 1., y: 2. },
            end: Coordinate { x: 3., y: 4. },
        };
        assert_eq!(s1, s2);
    }

    #[test]
    fn check_segment_not_equals() {
        let s1 = Segment {
            start: Coordinate { x: 1., y: 2.1 },
            end: Coordinate { x: 3., y: 4. },
        };
        let s2 = Segment {
            start: Coordinate { x: 1., y: 2.2 },
            end: Coordinate { x: 3., y: 4. },
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

}
