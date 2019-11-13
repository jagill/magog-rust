use crate::Coordinate;
use ordered_float::{FloatIsNan, NotNan};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Position<C: Coordinate> {
    pub x: C,
    pub y: C,
}

pub type SafePosition<C> = (NotNan<C>, NotNan<C>);

impl<C: Coordinate> From<(C, C)> for Position<C> {
    fn from(coords: (C, C)) -> Self {
        Position {
            x: coords.0,
            y: coords.1,
        }
    }
}

impl<C: Coordinate> From<[C; 2]> for Position<C> {
    fn from(coords: [C; 2]) -> Self {
        Position {
            x: coords[0],
            y: coords[1],
        }
    }
}

impl<C: Coordinate> From<SafePosition<C>> for Position<C> {
    fn from(coords: SafePosition<C>) -> Self {
        Position {
            x: coords.0.into_inner(),
            y: coords.1.into_inner(),
        }
    }
}

impl<C: Coordinate, P: Into<Position<C>>> From<&P> for Position<C> {
    fn from(p: &P) -> Self {
        p.into()
    }
}

impl<C: Coordinate> Position<C> {
    pub fn new(x: C, y: C) -> Position<C> {
        Position { x: x, y: y }
    }

    /// Cross product of the vector c1 x c2
    pub fn cross(c1: Position<C>, c2: Position<C>) -> C {
        c1.x * c2.y - c1.y * c2.x
    }

    /// Dot product of the vector c1 . c2
    pub fn dot(c1: Position<C>, c2: Position<C>) -> C {
        c1.x * c2.x + c1.y * c2.y
    }

    /**
     * Given p1, p2, return p_min and p_max.
     *
     * p_min has the minimum values for both x and y.
     * p_max has the maximum values for both x and y.
     * If any coordinate is NAN, set the min/max to be the other.
     * If both are NAN, return (NAN, NAN).
     */
    pub fn min_max(p1: Position<C>, p2: Position<C>) -> (Position<C>, Position<C>) {
        (
            Position::new(p1.x.min(p2.x), p1.y.min(p2.y)),
            Position::new(p1.x.max(p2.x), p1.y.max(p2.y)),
        )
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.x.is_finite() {
            return Err("x is not finite");
        };
        if !self.y.is_finite() {
            return Err("y is not finite");
        };
        Ok(())
    }

    /**
     * Return a position guaranteed not to have NaNs.
     *
     * If one of the coordinates is NaN, return FloatIsNan error.
     */
    pub fn to_hashable(&self) -> Result<SafePosition<C>, FloatIsNan> {
        let x = NotNan::new(self.x)?;
        let y = NotNan::new(self.y)?;
        Ok((x, y))
    }
}

impl<C: Coordinate> Sub for Position<C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<C: Coordinate> Add for Position<C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<C: Coordinate> Mul<C> for Position<C> {
    type Output = Self;

    fn mul(self, rhs: C) -> Self::Output {
        Position::new(self.x * rhs, self.y * rhs)
    }
}

impl<C: Coordinate> Div<C> for Position<C> {
    type Output = Self;

    fn div(self, rhs: C) -> Self::Output {
        Position::new(self.x / rhs, self.y / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_basic_pos_f32() {
        let x: f32 = 1.;
        let y: f32 = 2.;
        let p = Position { x: x, y: y };
        assert_eq!(p.x, x);
        assert_eq!(p.y, y);
    }

    #[test]
    fn check_basic_pos_f64() {
        let x: f64 = 1.;
        let y: f64 = 2.;
        let p = Position { x: x, y: y };
        assert_eq!(p.x, x);
        assert_eq!(p.y, y);
    }

    #[test]
    fn check_pos_equals() {
        let p1 = Position { x: 1., y: 2. };
        let p2 = Position { x: 1., y: 2. };
        assert_eq!(p1, p2);
    }

    #[test]
    fn check_pos_not_equals() {
        let c1 = Position { x: 1., y: 2. };
        let c2 = Position { x: 2., y: 1. };
        assert_ne!(c1, c2);
    }

    #[test]
    fn check_new_pos_f32() {
        let x: f32 = 1.;
        let y: f32 = 2.;
        let c = Position::new(x, y);
        assert_eq!(c.x, x);
        assert_eq!(c.y, y);
    }

    #[test]
    fn check_new_pos_f64() {
        let x: f64 = 1.;
        let y: f64 = 2.;
        let p = Position::new(x, y);
        assert_eq!(p.x, x);
        assert_eq!(p.y, y);
    }

    #[test]
    fn check_from_tuple() {
        let p = Position::from((0.0, 1.0));
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 1.0);
    }

    #[test]
    fn check_from_array() {
        let p = Position::from([0.0, 1.0]);
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 1.0);
    }
}
