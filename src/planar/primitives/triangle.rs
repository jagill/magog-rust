use crate::planar::primitives::Position;
use crate::Coordinate;

#[derive(Copy, Clone, Debug)]
pub struct Triangle<C: Coordinate>(pub Position<C>, pub Position<C>, pub Position<C>);

impl<C: Coordinate> Triangle<C> {
    pub fn to_array(&self) -> [Position<C>; 3] {
        [self.0, self.1, self.2]
    }
}

// (P, P, P) -> Triangle
impl<C: Coordinate, IC: Into<Position<C>> + Copy> From<(IC, IC, IC)> for Triangle<C> {
    fn from(positions: (IC, IC, IC)) -> Triangle<C> {
        Triangle(positions.0.into(), positions.1.into(), positions.2.into())
    }
}

impl<C: Coordinate> Triangle<C> {
    pub fn new(p0: Position<C>, p1: Position<C>, p2: Position<C>) -> Self {
        Triangle(p0, p1, p2)
    }

    pub fn signed_area(&self) -> C {
        ((self.1.x - self.0.x) * (self.2.y - self.0.y)
            - (self.2.x - self.0.x) * (self.1.y - self.0.y))
            / (C::one() + C::one())
    }

    pub fn area(&self) -> C {
        let signed_area = self.signed_area();
        if signed_area < C::zero() {
            -signed_area
        } else {
            signed_area
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
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
