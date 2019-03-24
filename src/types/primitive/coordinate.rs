use num_traits::{Bounded, Float, Signed};
use ordered_float::{FloatIsNan, NotNan};
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, Mul, Sub};

pub trait Coordinate: Float + Sum + Bounded + Signed + Debug + 'static {}
impl<T: Float + Sum + Bounded + Signed + Debug + 'static> Coordinate for T {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord2<T>
where
    T: Coordinate,
{
    pub x: T,
    pub y: T,
}

impl<T: Coordinate> From<(T, T)> for Coord2<T> {
    fn from(coords: (T, T)) -> Self {
        Coord2 {
            x: coords.0,
            y: coords.1,
        }
    }
}

impl<T: Coordinate> From<[T; 2]> for Coord2<T> {
    fn from(coords: [T; 2]) -> Self {
        Coord2 {
            x: coords[0],
            y: coords[1],
        }
    }
}

impl<T: Coordinate> From<(NotNan<T>, NotNan<T>)> for Coord2<T> {
    fn from(coords: (NotNan<T>, NotNan<T>)) -> Self {
        Coord2 {
            x: coords.0.into_inner(),
            y: coords.1.into_inner(),
        }
    }
}

impl<T: Coordinate> Coord2<T> {
    pub fn new(x: T, y: T) -> Coord2<T> {
        Coord2 { x: x, y: y }
    }

    /// Cross product of the vector c1 x c2
    pub fn cross(c1: Coord2<T>, c2: Coord2<T>) -> T
    where
        T: Coordinate,
    {
        c1.x * c2.y - c1.y * c2.x
    }

    /// Dot product of the vector c1 . c2
    pub fn dot(c1: Coord2<T>, c2: Coord2<T>) -> T
    where
        T: Coordinate,
    {
        c1.x * c2.x + c1.y * c2.y
    }

    /**
     * Order z1, z2 into (min, max).
     *
     * If z1 or z2 is NAN, set min/max to be the other.
     * If both are NAN, return (NAN, NAN).
     */
    pub fn min_max(z1: T, z2: T) -> (T, T) {
        (z1.min(z2), z1.max(z2))
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if !&self.x.is_finite() {
            return Err("x is not finite");
        };
        if !&self.y.is_finite() {
            return Err("y is not finite");
        };
        Ok(())
    }

    pub fn to_hashable(&self) -> Result<(NotNan<T>, NotNan<T>), FloatIsNan> {
        let x = NotNan::new(self.x)?;
        let y = NotNan::new(self.y)?;
        Ok((x, y))
    }
}

impl<T: Coordinate> Sub for Coord2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Coordinate> Add for Coord2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Coordinate> Mul<T> for Coord2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Coord2::new(self.x * rhs, self.y * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_basic_coord2_f32() {
        let x: f32 = 1.;
        let y: f32 = 2.;
        let c = Coord2 { x: x, y: y };
        assert_eq!(c.x, x);
        assert_eq!(c.y, y);
    }

    #[test]
    fn check_basic_coord2_f64() {
        let x: f64 = 1.;
        let y: f64 = 2.;
        let c = Coord2 { x: x, y: y };
        assert_eq!(c.x, x);
        assert_eq!(c.y, y);
    }

    #[test]
    fn check_coord2_equals() {
        let c1 = Coord2 { x: 1., y: 2. };
        let c2 = Coord2 { x: 1., y: 2. };
        assert_eq!(c1, c2);
    }

    #[test]
    fn check_coord2_not_equals() {
        let c1 = Coord2 { x: 1., y: 2. };
        let c2 = Coord2 { x: 2., y: 1. };
        assert_ne!(c1, c2);
    }

    #[test]
    fn check_new_coord2_f32() {
        let x: f32 = 1.;
        let y: f32 = 2.;
        let c = Coord2::new(x, y);
        assert_eq!(c.x, x);
        assert_eq!(c.y, y);
    }

    #[test]
    fn check_new_coord2_f64() {
        let x: f64 = 1.;
        let y: f64 = 2.;
        let c = Coord2::new(x, y);
        assert_eq!(c.x, x);
        assert_eq!(c.y, y);
    }

    #[test]
    fn check_from_tuple() {
        let c = Coord2::from((0.0, 1.0));
        assert_eq!(c.x, 0.0);
        assert_eq!(c.y, 1.0);
    }

    #[test]
    fn check_from_array() {
        let c = Coord2::from([0.0, 1.0]);
        assert_eq!(c.x, 0.0);
        assert_eq!(c.y, 1.0);
    }
}
