use crate::types::primitive::{Coord2, CoordinateType};

#[derive(Copy, Clone, Debug)]
pub struct Triangle<T: CoordinateType>(pub Coord2<T>, pub Coord2<T>, pub Coord2<T>);

impl<T: CoordinateType> Triangle<T> {
    pub fn to_array(&self) -> [Coord2<T>; 3] {
        [self.0, self.1, self.2]
    }
}

impl<IC: Into<Coord2<T>> + Copy, T: CoordinateType> From<(IC, IC, IC)> for Triangle<T> {
    fn from(coords: (IC, IC, IC)) -> Triangle<T> {
        Triangle(coords.0.into(), coords.1.into(), coords.2.into())
    }
}

impl<T: CoordinateType> Triangle<T> {
    pub fn new(c0: Coord2<T>, c1: Coord2<T>, c2: Coord2<T>) -> Self {
        Triangle(c0, c1, c2)
    }

    pub fn signed_area(&self) -> T {
        ((self.1.x - self.0.x) * (self.2.y - self.0.y)
            - (self.2.x - self.0.x) * (self.1.y - self.0.y))
            / (T::one() + T::one())
    }

    pub fn area(&self) -> T {
        let signed_area = self.signed_area();
        if signed_area < T::zero() {
            -signed_area
        } else {
            signed_area
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_signed_area() {
        let t = Triangle::from(((0., 0.), (1., 0.), (0., 1.)));
        assert_eq!(t.signed_area(), 0.5);
    }

    #[test]
    fn check_signed_area_negative() {
        let t = Triangle::from(((0., 0.), (0., 1.), (1., 0.)));
        assert_eq!(t.signed_area(), -0.5);
    }

    #[test]
    fn check_area() {
        let t = Triangle::from(((0., 0.), (1., 0.), (0., 1.)));
        assert_eq!(t.area(), 0.5);
    }

    #[test]
    fn check_area_not_negative() {
        let t = Triangle::from(((0., 0.), (0., 1.), (1., 0.)));
        assert_eq!(t.area(), 0.5);
    }
}
