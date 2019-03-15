use crate::types::{Coordinate, CoordinateType};

/// Cross product of the vector (c2 - c1) x c0
pub fn cross_product<T>(c0: Coordinate<T>, c1: Coordinate<T>, c2: Coordinate<T>) -> T
where
    T: CoordinateType,
{
    (c1.x - c0.x) * (c2.y - c0.y) - (c2.x - c0.x) * (c1.y - c0.y)
}
