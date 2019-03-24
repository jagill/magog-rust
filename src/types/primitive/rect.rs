use crate::types::primitive::{Coord2, Coordinate};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect<T>
where
    T: Coordinate,
{
    pub min: Coord2<T>,
    pub max: Coord2<T>,
}

//// From Conversions

// (Coord2, Coord2) -> Rect
impl<T: Coordinate, IC: Into<Coord2<T>>> From<(IC, IC)> for Rect<T> {
    fn from(coords: (IC, IC)) -> Self {
        Rect::new(coords.0.into(), coords.1.into())
    }
}

impl<T: Coordinate> Rect<T> {
    pub fn new(c1: Coord2<T>, c2: Coord2<T>) -> Rect<T> {
        let (min_x, max_x) = Coord2::min_max(c1.x, c2.x);
        let (min_y, max_y) = Coord2::min_max(c1.y, c2.y);
        Rect {
            min: Coord2::from((min_x, min_y)),
            max: Coord2::from((max_x, max_y)),
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

    pub fn contains(&self, c: Coord2<T>) -> bool {
        self.min.x <= c.x && self.max.x >= c.x && self.min.y <= c.y && self.max.y >= c.y
    }

    /// Return a rect expanded by c.  Nans absorbed when possible.
    pub fn add_coord(&self, c: Coord2<T>) -> Rect<T> {
        Rect::new(
            Coord2::new(self.min.x.min(c.x), self.min.y.min(c.y)),
            Coord2::new(self.max.x.max(c.x), self.max.y.max(c.y)),
        )
    }

    /// Return a rect expanded by an other rect.  Nans absorbed when possible.
    pub fn merge(&self, other: Rect<T>) -> Rect<T> {
        Rect {
            min: Coord2 {
                x: self.min.x.min(other.min.x),
                y: self.min.y.min(other.min.y),
            },
            max: Coord2 {
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
            min: Coord2 { x: min_x, y: min_y },
            max: Coord2 { x: max_x, y: max_y },
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
            min: Coord2 { x: min_x, y: min_y },
            max: Coord2 { x: max_x, y: max_y },
        };
        assert_eq!(e.min.x, min_x);
        assert_eq!(e.min.y, min_y);
        assert_eq!(e.max.x, max_x);
        assert_eq!(e.max.y, max_y);
    }

    #[test]
    fn check_rect_equals() {
        let e1 = Rect {
            min: Coord2 { x: 1., y: 2. },
            max: Coord2 { x: 3., y: 4. },
        };
        let e2 = Rect {
            min: Coord2 { x: 1., y: 2. },
            max: Coord2 { x: 3., y: 4. },
        };
        assert_eq!(e1, e2);
    }

    #[test]
    fn check_rect_not_equals() {
        let e1 = Rect {
            min: Coord2 { x: 1., y: 2.1 },
            max: Coord2 { x: 3., y: 4. },
        };
        let e2 = Rect {
            min: Coord2 { x: 1., y: 2.2 },
            max: Coord2 { x: 3., y: 4. },
        };
        assert_ne!(e1, e2);
    }

    #[test]
    fn check_new_rect_f32() {
        let min_x: f32 = 1.;
        let min_y: f32 = 2.;
        let max_x: f32 = 3.;
        let max_y: f32 = 4.;
        let e = Rect::new(Coord2 { x: min_x, y: min_y }, Coord2 { x: max_x, y: max_y });
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
        let e = Rect::new(Coord2 { x: min_x, y: min_y }, Coord2 { x: max_x, y: max_y });
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
            min: Coord2 { x: min_x, y: min_y },
            max: Coord2 { x: max_x, y: max_y },
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
            min: Coord2 { x: min_x, y: min_y },
            max: Coord2 { x: max_x, y: max_y },
        };
        assert!(r.validate().is_err(), "Min_y > max_y");
    }

    #[test]
    fn check_new_absorb_nans() {
        let r1 = Rect::new(Coord2::new(0.0, f32::NAN), Coord2::new(f32::NAN, 0.0));
        let r2 = Rect::new(Coord2::new(0.0, 0.0), Coord2::new(0.0, 0.0));
        assert_eq!(r1, r2)
    }

    #[test]
    fn check_contains_coord() {
        let e = Rect {
            min: Coord2 { x: 0., y: 0. },
            max: Coord2 { x: 1., y: 1. },
        };
        let p = Coord2 { x: 0.5, y: 0.5 };
        assert!(e.contains(p));
    }

    #[test]
    fn check_not_contains_coord() {
        let e = Rect {
            min: Coord2 { x: 0., y: 0. },
            max: Coord2 { x: 1., y: 1. },
        };
        let p = Coord2 { x: 1.5, y: 0.5 };
        assert!(!e.contains(p));
    }

    #[test]
    fn check_from_coord_tuple() {
        let c0 = Coord2 { x: 0.0, y: 3.0 };
        let c1 = Coord2 { x: 1.0, y: 2.0 };
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
        let mut r = Rect::new(Coord2::new(0.0, 0.0), Coord2::new(2.0, 0.0));
        r = r.add_coord(Coord2::new(1.0, 1.0));
        let r2 = Rect::new(Coord2::new(0.0, 0.0), Coord2::new(2.0, 1.0));
        assert_eq!(r, r2)
    }

    #[test]
    fn check_add_coord_nan() {
        let mut r = Rect::new(Coord2::new(0.0, f32::NAN), Coord2::new(2.0, f32::NAN));
        r = r.add_coord(Coord2::new(1.0, 1.0));
        let r2 = Rect::new(Coord2::new(0.0, 1.0), Coord2::new(2.0, 1.0));
        assert_eq!(r, r2)
    }
}
