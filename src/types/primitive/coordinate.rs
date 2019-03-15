use crate::types::CoordinateType;
use ordered_float::{FloatIsNan, NotNan};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coordinate<T>
where
    T: CoordinateType,
{
    pub x: T,
    pub y: T,
}

impl<T: CoordinateType> From<(T, T)> for Coordinate<T> {
    fn from(coords: (T, T)) -> Self {
        Coordinate {
            x: coords.0,
            y: coords.1,
        }
    }
}

impl<T: CoordinateType> From<(NotNan<T>, NotNan<T>)> for Coordinate<T> {
    fn from(coords: (NotNan<T>, NotNan<T>)) -> Self {
        Coordinate {
            x: coords.0.into_inner(),
            y: coords.1.into_inner(),
        }
    }
}

impl<T: CoordinateType> Coordinate<T> {
    pub fn new(x: T, y: T) -> Coordinate<T> {
        Coordinate { x: x, y: y }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_basic_coordinate_f32() {
        let x: f32 = 1.;
        let y: f32 = 2.;
        let c = Coordinate { x: x, y: y };
        assert_eq!(c.x, x);
        assert_eq!(c.y, y);
    }

    #[test]
    fn check_basic_coordinate_f64() {
        let x: f64 = 1.;
        let y: f64 = 2.;
        let c = Coordinate { x: x, y: y };
        assert_eq!(c.x, x);
        assert_eq!(c.y, y);
    }

    #[test]
    fn check_coordinate_equals() {
        let c1 = Coordinate { x: 1., y: 2. };
        let c2 = Coordinate { x: 1., y: 2. };
        assert_eq!(c1, c2);
    }

    #[test]
    fn check_coordinate_not_equals() {
        let c1 = Coordinate { x: 1., y: 2. };
        let c2 = Coordinate { x: 2., y: 1. };
        assert_ne!(c1, c2);
    }

    #[test]
    fn check_new_coordinate_f32() {
        let x: f32 = 1.;
        let y: f32 = 2.;
        let c = Coordinate::new(x, y);
        assert_eq!(c.x, x);
        assert_eq!(c.y, y);
    }

    #[test]
    fn check_new_coordinate_f64() {
        let x: f64 = 1.;
        let y: f64 = 2.;
        let c = Coordinate::new(x, y);
        assert_eq!(c.x, x);
        assert_eq!(c.y, y);
    }

    #[test]
    fn check_from_tuple() {
        let c = Coordinate::from((0.0, 1.0));
        assert_eq!(c.x, 0.0);
        assert_eq!(c.y, 1.0);
    }
}
