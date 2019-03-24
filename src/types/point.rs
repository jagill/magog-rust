use crate::types::{Coord2, Coordinate, Envelope, Geometry};

#[derive(Debug, PartialEq)]
pub struct Point<T>(pub Coord2<T>)
where
    T: Coordinate;

/// Turn a `Coord2`-ish object into a `Point`.
impl<T: Coordinate, IC: Into<Coord2<T>>> From<IC> for Point<T> {
    fn from(c: IC) -> Self {
        Point(c.into())
    }
}

impl<T: Coordinate> Point<T> {
    pub fn new(coord: Coord2<T>) -> Point<T> {
        Point(coord)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        self.0.validate()?;
        Ok(())
    }

    pub fn x(&self) -> T {
        self.0.x
    }

    pub fn y(&self) -> T {
        self.0.y
    }
}

// GEOMETRY implementation
impl<T: Coordinate> Point<T> {
    pub fn dimension(&self) -> u8 {
        0
    }

    pub fn geometry_type(&self) -> &'static str {
        "Point"
    }

    pub fn envelope(&self) -> Envelope<T> {
        Envelope::from((self.0, self.0))
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn is_simple(&self) -> bool {
        match self.validate() {
            Err(_) => false,
            Ok(_) => true,
        }
    }

    pub fn boundary(&self) -> Geometry<T> {
        Geometry::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32;

    #[test]
    fn check_constructor() {
        let p = Point(Coord2 { x: 0.1, y: 1.0 });
        assert_eq!(p.x(), 0.1);
        assert_eq!(p.y(), 1.0);
    }

    #[test]
    fn check_is_simple() {
        let p = Point(Coord2 { x: 0.1, y: 1.0 });
        assert!(p.is_simple());
    }

    #[test]
    fn check_is_not_simple() {
        let p = Point(Coord2 {
            x: 0.1,
            y: f32::NAN,
        });
        assert!(!p.is_simple());
    }

}
