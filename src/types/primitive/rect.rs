use crate::types::primitive::{Coordinate, CoordinateType};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect<T>
where
    T: CoordinateType,
{
    pub min: Coordinate<T>,
    pub max: Coordinate<T>,
}

//// From Conversions

/**
 * Order z1, z2 into (min, max).
 *
 * If z1 or z2 is NAN, set min/max to be the other.
 * If both are NAN, return (NAN, NAN).
 */
fn min_max<T: CoordinateType>(z1: T, z2: T) -> (T, T) {
    (z1.min(z2), z1.max(z2))
}

// (Coordinate, Coordinate) -> Rect
impl<T: CoordinateType, IC: Into<Coordinate<T>>> From<(IC, IC)> for Rect<T> {
    fn from(coords: (IC, IC)) -> Self {
        Rect::new(coords.0.into(), coords.1.into())
    }
}

impl<T: CoordinateType> Rect<T> {
    pub fn new(c1: Coordinate<T>, c2: Coordinate<T>) -> Rect<T> {
        let (min_x, max_x) = min_max(c1.x, c2.x);
        let (min_y, max_y) = min_max(c1.y, c2.y);
        Rect {
            min: Coordinate::from((min_x, min_y)),
            max: Coordinate::from((max_x, max_y)),
        }
    }

    /// Rect is valid if there coords are finite, and min <= max.
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

    /// Return a rect expanded by c.  Nans absorbed when possible.
    pub fn add_coord(&self, c: Coordinate<T>) -> Rect<T> {
        Rect::new(
            Coordinate::new(self.min.x.min(c.x), self.min.y.min(c.y)),
            Coordinate::new(self.max.x.max(c.x), self.max.y.max(c.y)),
        )
    }

    /// Return a rect expanded by an other rect.  Nans absorbed when possible.
    pub fn merge(&self, other: Rect<T>) -> Rect<T> {
        Rect {
            min: Coordinate {
                x: self.min.x.min(other.min.x),
                y: self.min.y.min(other.min.y),
            },
            max: Coordinate {
                x: self.max.x.max(other.max.x),
                y: self.max.y.max(other.max.y),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32;

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
        let e = Rect::new(
            Coordinate { x: min_x, y: min_y },
            Coordinate { x: max_x, y: max_y },
        );
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
        let e = Rect::new(
            Coordinate { x: min_x, y: min_y },
            Coordinate { x: max_x, y: max_y },
        );
        assert_eq!(e.min.x, min_x);
        assert_eq!(e.min.y, min_y);
        assert_eq!(e.max.x, max_x);
        assert_eq!(e.max.y, max_y);
    }

    #[test]
    fn check_validate_fail_x() {
        let min_x: f64 = 3.;
        let min_y: f64 = 2.;
        let max_x: f64 = 1.;
        let max_y: f64 = 4.;
        let r = Rect {
            min: Coordinate { x: min_x, y: min_y },
            max: Coordinate { x: max_x, y: max_y },
        };
        assert!(r.validate().is_err(), "Min_x > max_x");
    }

    #[test]
    fn check_validate_fail_y() {
        let min_x: f64 = 1.;
        let min_y: f64 = 4.;
        let max_x: f64 = 3.;
        let max_y: f64 = 2.;
        let r = Rect {
            min: Coordinate { x: min_x, y: min_y },
            max: Coordinate { x: max_x, y: max_y },
        };
        assert!(r.validate().is_err(), "Min_y > max_y");
    }

    #[test]
    fn check_new_absorb_nans() {
        let r1 = Rect::new(
            Coordinate::new(0.0, f32::NAN),
            Coordinate::new(f32::NAN, 0.0),
        );
        let r2 = Rect::new(Coordinate::new(0.0, 0.0), Coordinate::new(0.0, 0.0));
        assert_eq!(r1, r2)
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
    fn check_add_coord() {
        let mut r = Rect::new(Coordinate::new(0.0, 0.0), Coordinate::new(2.0, 0.0));
        r = r.add_coord(Coordinate::new(1.0, 1.0));
        let r2 = Rect::new(Coordinate::new(0.0, 0.0), Coordinate::new(2.0, 1.0));
        assert_eq!(r, r2)
    }

    #[test]
    fn check_add_coord_nan() {
        let mut r = Rect::new(
            Coordinate::new(0.0, f32::NAN),
            Coordinate::new(2.0, f32::NAN),
        );
        r = r.add_coord(Coordinate::new(1.0, 1.0));
        let r2 = Rect::new(Coordinate::new(0.0, 1.0), Coordinate::new(2.0, 1.0));
        assert_eq!(r, r2)
    }
}
