use crate::types::primitive::{Coordinate, Position};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect<C: Coordinate> {
    pub min: Position<C>,
    pub max: Position<C>,
}

//// From Conversions

// (Position, Position) -> Rect
impl<C: Coordinate, IP: Into<Position<C>>> From<(IP, IP)> for Rect<C> {
    fn from(positions: (IP, IP)) -> Self {
        Rect::new(positions.0.into(), positions.1.into())
    }
}

impl<C: Coordinate> Rect<C> {
    pub fn new(p1: Position<C>, p2: Position<C>) -> Rect<C> {
        let (min_x, max_x) = Position::min_max(p1.x, p2.x);
        let (min_y, max_y) = Position::min_max(p1.y, p2.y);
        Rect {
            min: Position::from((min_x, min_y)),
            max: Position::from((max_x, max_y)),
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

    pub fn contains(&self, p: Position<C>) -> bool {
        self.min.x <= p.x && self.max.x >= p.x && self.min.y <= p.y && self.max.y >= p.y
    }

    /// Do these two rects intersect?
    pub fn intersects(&self, r: Rect<C>) -> bool {
        self.min.x <= r.max.x
            && self.max.x >= r.min.x
            && self.min.y <= r.max.y
            && self.max.y >= r.min.y
    }

    /// Return a rect expanded by position.  Nans absorbed when possible.
    pub fn add_position(&self, p: Position<C>) -> Rect<C> {
        Rect::new(
            Position::new(self.min.x.min(p.x), self.min.y.min(p.y)),
            Position::new(self.max.x.max(p.x), self.max.y.max(p.y)),
        )
    }

    /// Return a rect expanded by an other rect.  Nans absorbed when possible.
    pub fn merge(&self, other: Rect<C>) -> Rect<C> {
        Rect {
            min: Position {
                x: self.min.x.min(other.min.x),
                y: self.min.y.min(other.min.y),
            },
            max: Position {
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
            min: Position { x: min_x, y: min_y },
            max: Position { x: max_x, y: max_y },
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
            min: Position { x: min_x, y: min_y },
            max: Position { x: max_x, y: max_y },
        };
        assert_eq!(e.min.x, min_x);
        assert_eq!(e.min.y, min_y);
        assert_eq!(e.max.x, max_x);
        assert_eq!(e.max.y, max_y);
    }

    #[test]
    fn check_rect_equals() {
        let e1 = Rect {
            min: Position { x: 1., y: 2. },
            max: Position { x: 3., y: 4. },
        };
        let e2 = Rect {
            min: Position { x: 1., y: 2. },
            max: Position { x: 3., y: 4. },
        };
        assert_eq!(e1, e2);
    }

    #[test]
    fn check_rect_not_equals() {
        let e1 = Rect {
            min: Position { x: 1., y: 2.1 },
            max: Position { x: 3., y: 4. },
        };
        let e2 = Rect {
            min: Position { x: 1., y: 2.2 },
            max: Position { x: 3., y: 4. },
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
            Position { x: min_x, y: min_y },
            Position { x: max_x, y: max_y },
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
            Position { x: min_x, y: min_y },
            Position { x: max_x, y: max_y },
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
            min: Position { x: min_x, y: min_y },
            max: Position { x: max_x, y: max_y },
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
            min: Position { x: min_x, y: min_y },
            max: Position { x: max_x, y: max_y },
        };
        assert!(r.validate().is_err(), "Min_y > max_y");
    }

    #[test]
    fn check_new_absorb_nans() {
        let r1 = Rect::new(Position::new(0.0, f32::NAN), Position::new(f32::NAN, 0.0));
        let r2 = Rect::new(Position::new(0.0, 0.0), Position::new(0.0, 0.0));
        assert_eq!(r1, r2)
    }

    #[test]
    fn check_contains_position() {
        let r = Rect {
            min: Position { x: 0., y: 0. },
            max: Position { x: 1., y: 1. },
        };
        let p = Position { x: 0.5, y: 0.5 };
        assert!(r.contains(p));
    }

    #[test]
    fn check_not_contains_position() {
        let e = Rect {
            min: Position { x: 0., y: 0. },
            max: Position { x: 1., y: 1. },
        };
        let p = Position { x: 1.5, y: 0.5 };
        assert!(!e.contains(p));
    }

    #[test]
    fn check_from_position_tuple() {
        let p0 = Position { x: 0.0, y: 3.0 };
        let p1 = Position { x: 1.0, y: 2.0 };
        let r = Rect::from((p0, p1));
        assert_eq!(r.min.x, 0.0);
        assert_eq!(r.min.y, 2.0);
        assert_eq!(r.max.x, 1.0);
        assert_eq!(r.max.y, 3.0);
    }

    #[test]
    fn check_from_tuple_tuple() {
        let r = Rect::from(((0.0, 3.0), (1.0, 2.0)));
        assert_eq!(r.min.x, 0.0);
        assert_eq!(r.min.y, 2.0);
        assert_eq!(r.max.x, 1.0);
        assert_eq!(r.max.y, 3.0);
    }

    #[test]
    fn check_add_position() {
        let mut r = Rect::new(Position::new(0.0, 0.0), Position::new(2.0, 0.0));
        r = r.add_position(Position::new(1.0, 1.0));
        let r2 = Rect::new(Position::new(0.0, 0.0), Position::new(2.0, 1.0));
        assert_eq!(r, r2)
    }

    #[test]
    fn check_add_position_nan() {
        let mut r = Rect::new(Position::new(0.0, f32::NAN), Position::new(2.0, f32::NAN));
        r = r.add_position(Position::new(1.0, 1.0));
        let r2 = Rect::new(Position::new(0.0, 1.0), Position::new(2.0, 1.0));
        assert_eq!(r, r2)
    }
}
