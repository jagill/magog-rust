use crate::types::primitive::{Coordinate, CoordinateType, Segment};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect<T>
where
    T: CoordinateType,
{
    pub min: Coordinate<T>,
    pub max: Coordinate<T>,
}

//// From Conversions

// (Coordinate, Coordinate) -> Rect
impl<T: CoordinateType, IC: Into<Coordinate<T>>> From<(IC, IC)> for Rect<T> {
    fn from(coords: (IC, IC)) -> Self {
        let c1: Coordinate<T> = coords.0.into();
        let c2: Coordinate<T> = coords.1.into();
        let (min_x, max_x) = if c1.x < c2.x {
            (c1.x, c2.x)
        } else {
            (c2.x, c1.x)
        };
        let (min_y, max_y) = if c1.y < c2.y {
            (c1.y, c2.y)
        } else {
            (c2.y, c1.y)
        };
        Rect {
            min: Coordinate::from((min_x, min_y)),
            max: Coordinate::from((max_x, max_y)),
        }
    }
}

// Segment -> Rect
impl<T: CoordinateType> From<Segment<T>> for Rect<T> {
    fn from(seg: Segment<T>) -> Self {
        Rect::from((seg.start, seg.end))
    }
}

impl<T: CoordinateType> Rect<T> {
    pub fn new(min: Coordinate<T>, max: Coordinate<T>) -> Rect<T> {
        Rect { min, max }
    }

    pub fn new_validate(min: Coordinate<T>, max: Coordinate<T>) -> Result<Rect<T>, &'static str> {
        let e = Rect::new(min, max);
        e.validate()?;
        Ok(e)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        &self.min.validate()?;
        &self.max.validate()?;
        if &self.min.x > &self.max.x {
            return Err("Min_x is greater than max_x");
        };
        if &self.min.y > &self.max.y {
            return Err("Min_y is greater than max_y");
        };
        Ok(())
    }

    pub fn contains(&self, c: Coordinate<T>) -> bool {
        self.min.x <= c.x && self.max.x >= c.x && self.min.y <= c.y && self.max.y >= c.y
    }

    pub fn add_coord(&mut self, c: Coordinate<T>) {
        if c.x < self.min.x {
            self.min.x = c.x
        }
        if c.x > self.max.x {
            self.max.x = c.x
        }
        if c.y < self.min.y {
            self.min.y = c.y
        }
        if c.y > self.max.y {
            self.max.y = c.y
        }
    }

    pub fn merge(&self, other: Rect<T>) -> Rect<T> {
        let min_x = other.min.x.min(self.min.x);
        let min_y = other.min.y.min(self.min.y);
        let max_x = other.max.x.max(self.max.x);
        let max_y = other.max.y.max(self.max.y);
        Rect {
            min: Coordinate { x: min_x, y: min_y },
            max: Coordinate { x: max_x, y: max_y },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_basic_rect_f32() {
        let min_x: f32 = 1.;
        let min_y: f32 = 2.;
        let max_x: f32 = 3.;
        let max_y: f32 = 4.;
        let e = Rect {
            min: Coordinate { x: min_x, y: min_y },
            max: Coordinate { x: max_x, y: max_y },
        };
        assert_eq!(e.min.x, min_x);
        assert_eq!(e.min.y, min_y);
        assert_eq!(e.max.x, max_x);
        assert_eq!(e.max.y, max_y);
    }

    #[test]
    fn check_basic_rect_f64() {
        let min_x: f64 = 1.;
        let min_y: f64 = 2.;
        let max_x: f64 = 3.;
        let max_y: f64 = 4.;
        let e = Rect {
            min: Coordinate { x: min_x, y: min_y },
            max: Coordinate { x: max_x, y: max_y },
        };
        assert_eq!(e.min.x, min_x);
        assert_eq!(e.min.y, min_y);
        assert_eq!(e.max.x, max_x);
        assert_eq!(e.max.y, max_y);
    }

    #[test]
    fn check_rect_equals() {
        let e1 = Rect {
            min: Coordinate { x: 1., y: 2. },
            max: Coordinate { x: 3., y: 4. },
        };
        let e2 = Rect {
            min: Coordinate { x: 1., y: 2. },
            max: Coordinate { x: 3., y: 4. },
        };
        assert_eq!(e1, e2);
    }

    #[test]
    fn check_rect_not_equals() {
        let e1 = Rect {
            min: Coordinate { x: 1., y: 2.1 },
            max: Coordinate { x: 3., y: 4. },
        };
        let e2 = Rect {
            min: Coordinate { x: 1., y: 2.2 },
            max: Coordinate { x: 3., y: 4. },
        };
        assert_ne!(e1, e2);
    }

    #[test]
    fn check_new_rect_f32() {
        let min_x: f32 = 1.;
        let min_y: f32 = 2.;
        let max_x: f32 = 3.;
        let max_y: f32 = 4.;
        let e = Rect::new_validate(
            Coordinate { x: min_x, y: min_y },
            Coordinate { x: max_x, y: max_y },
        ).expect("Shouldn't fail construction here.");
        assert_eq!(e.min.x, min_x);
        assert_eq!(e.min.y, min_y);
        assert_eq!(e.max.x, max_x);
        assert_eq!(e.max.y, max_y);
    }

    #[test]
    fn check_new_rect_f64() {
        let min_x: f64 = 1.;
        let min_y: f64 = 2.;
        let max_x: f64 = 3.;
        let max_y: f64 = 4.;
        let e = Rect::new_validate(
            Coordinate { x: min_x, y: min_y },
            Coordinate { x: max_x, y: max_y },
        ).expect("Shouldn't fail construction here.");
        assert_eq!(e.min.x, min_x);
        assert_eq!(e.min.y, min_y);
        assert_eq!(e.max.x, max_x);
        assert_eq!(e.max.y, max_y);
    }

    #[test]
    fn check_new_rect_fail_x() {
        let min_x: f64 = 3.;
        let min_y: f64 = 2.;
        let max_x: f64 = 1.;
        let max_y: f64 = 4.;
        assert!(
            Rect::new_validate(
                Coordinate { x: min_x, y: min_y },
                Coordinate { x: max_x, y: max_y },
            ).is_err(),
            "Min_x > max_x"
        );
    }

    #[test]
    fn check_new_rect_fail_y() {
        let min_x: f64 = 1.;
        let min_y: f64 = 4.;
        let max_x: f64 = 3.;
        let max_y: f64 = 2.;
        assert!(
            Rect::new_validate(
                Coordinate { x: min_x, y: min_y },
                Coordinate { x: max_x, y: max_y },
            ).is_err(),
            "Min_y > max_y"
        );
    }

    #[test]
    fn check_contains_coord() {
        let e = Rect {
            min: Coordinate { x: 0., y: 0. },
            max: Coordinate { x: 1., y: 1. },
        };
        let p = Coordinate { x: 0.5, y: 0.5 };
        assert!(e.contains(p));
    }

    #[test]
    fn check_not_contains_coord() {
        let e = Rect {
            min: Coordinate { x: 0., y: 0. },
            max: Coordinate { x: 1., y: 1. },
        };
        let p = Coordinate { x: 1.5, y: 0.5 };
        assert!(!e.contains(p));
    }

    #[test]
    fn check_from_coord_tuple() {
        let c0 = Coordinate { x: 0.0, y: 3.0 };
        let c1 = Coordinate { x: 1.0, y: 2.0 };
        let e = Rect::from((c0, c1));
        assert_eq!(e.min.x, 0.0);
        assert_eq!(e.min.y, 2.0);
        assert_eq!(e.max.x, 1.0);
        assert_eq!(e.max.y, 3.0);
    }

    #[test]
    fn check_from_tuple_tuple() {
        let e = Rect::from(((0.0, 3.0), (1.0, 2.0)));
        assert_eq!(e.min.x, 0.0);
        assert_eq!(e.min.y, 2.0);
        assert_eq!(e.max.x, 1.0);
        assert_eq!(e.max.y, 3.0);
    }

    #[test]
    fn check_from_rect() {
        let s = Segment::from(((0.0, 2.0), (1.0, 3.0)));
        let e = Rect::from(s);
        assert_eq!(e.min.x, 0.0);
        assert_eq!(e.min.y, 2.0);
        assert_eq!(e.max.x, 1.0);
        assert_eq!(e.max.y, 3.0);
    }

}
