use crate::types::{Coordinate, CoordinateType};
use num_traits::Float;
use std::cmp::Ordering;

/// Cross product of the vector (c2 - c1) x c0
pub fn cross_product<T>(c0: Coordinate<T>, c1: Coordinate<T>, c2: Coordinate<T>) -> T
where
    T: CoordinateType,
{
    (c1.x - c0.x) * (c2.y - c0.y) - (c2.x - c0.x) * (c1.y - c0.y)
}

/// Compares two floats, putting NaNs at the end.
fn compare_coordinate_types<T: Float>(a: &T, b: &T) -> Ordering {
    match (a, b) {
        (x, y) if x.is_nan() && y.is_nan() => Ordering::Equal,
        (x, _) if x.is_nan() => Ordering::Greater,
        (_, y) if y.is_nan() => Ordering::Less,
        (_, _) => a.partial_cmp(b).unwrap(),
    }
}

/// Compare coordintes lexigraphically, putting NaNs at the end.
pub fn compare_coordinates<T: CoordinateType>(c1: &Coordinate<T>, c2: &Coordinate<T>) -> Ordering {
    let first_compare = compare_coordinate_types(&c1.x, &c2.x);
    match first_compare {
        Ordering::Equal => compare_coordinate_types(&c1.y, &c2.y),
        _ => first_compare,
    }
}
