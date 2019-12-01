use crate::Coordinate;
use ordered_float::{FloatIsNan, NotNan};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Position<C: Coordinate> {
    pub x: C,
}

pub type SafePosition<C> = NotNan<C>;

impl<C: Coordinate> From<C> for Position<C> {
    fn from(coord: C) -> Self {
        Position { x: coord }
    }
}
impl<C: Coordinate> From<SafePosition<C>> for Position<C> {
    fn from(coord: SafePosition<C>) -> Self {
        Position {
            x: coord.into_inner(),
        }
    }
}

impl<C: Coordinate> Position<C> {
    pub fn new(x: C) -> Self {
        Position { x }
    }

    pub fn min(&self, other: Self) -> Self {
        Position::new(self.x.min(other.x))
    }

    pub fn max(&self, other: Self) -> Self {
        Position::new(self.x.max(other.x))
    }

    /**
     * Given p1, p2, return p_min and p_max.
     *
     * If any coordinate is NAN, set the min/max to be the other.
     * If both are NAN, return (NAN, NAN).
     */
    pub fn min_max(p1: Self, p2: Self) -> (Self, Self) {
        (p1.min(p2), p1.max(p2))
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.x.is_finite() {
            return Err("x is not finite");
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
        Ok(x)
    }
}

impl<C: Coordinate> Sub for Position<C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position::new(self.x - rhs.x)
    }
}

impl<C: Coordinate> Add for Position<C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x)
    }
}

impl<C: Coordinate> Mul<C> for Position<C> {
    type Output = Self;

    fn mul(self, rhs: C) -> Self::Output {
        Position::new(self.x * rhs)
    }
}

impl<C: Coordinate> Div<C> for Position<C> {
    type Output = Self;

    fn div(self, rhs: C) -> Self::Output {
        Position::new(self.x / rhs)
    }
}
