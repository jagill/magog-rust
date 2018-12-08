use crate::types::{Coordinate, CoordinateType, Envelope, Geometry, Point};
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct MultiPoint<T>
where
    T: CoordinateType,
{
    pub points: Vec<Point<T>>,
    _envelope: Envelope<T>,
}

/// Turn a `Vec` of `Coordinate`-ish objects into a `LineString`.
impl<T: CoordinateType, IC: Into<Coordinate<T>>> From<Vec<IC>> for MultiPoint<T> {
    fn from(v: Vec<IC>) -> Self {
        MultiPoint::new(v.into_iter().map(|c| Point(c.into())).collect())
    }
}

impl<T: CoordinateType> MultiPoint<T> {
    pub fn new(points: Vec<Point<T>>) -> Self {
        let coords: Vec<Coordinate<T>> = points.iter().map(|p| p.0).collect();
        let _envelope: Envelope<T> = Envelope::from(&coords);
        MultiPoint { points, _envelope }
    }
}

// GEOMETRY implementation
impl<T: CoordinateType> MultiPoint<T> {
    pub fn dimension(&self) -> u8 {
        0
    }

    pub fn geometry_type(&self) -> &'static str {
        "MultiPoint"
    }

    pub fn envelope(&self) -> Envelope<T> {
        self._envelope
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// A MultiPoint is simple if it has no duplicate points.
    pub fn is_simple(&self) -> bool {
        // TODO: Only sort once.
        // XXX TODO: Really, any NaN values in coordinates should be an error.
        let mut coords: Vec<Coordinate<T>> = self.points.iter().map(|p| p.0).collect();
        coords.sort_unstable_by(|a, b| utils::compare_coordinates(a, b));
        has_adjacent_duplicates(&coords)
    }

    pub fn boundary(&self) -> Geometry<T> {
        Geometry::Empty
    }
}

fn has_adjacent_duplicates<T: CoordinateType>(coords: &Vec<Coordinate<T>>) -> bool {
    let mut last_coord: Option<Coordinate<T>> = None;
    for c in coords.clone() {
        match last_coord {
            None => last_coord = Some(c),
            Some(c0) => {
                if c == c0 {
                    return false;
                } else {
                    last_coord = Some(c);
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_is_simple() {
        let mp = MultiPoint::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        assert!(mp.is_simple());
    }

    #[test]
    fn check_not_is_simple() {
        let mp = MultiPoint::from(vec![(0.0, 0.0), (1.0, 1.0), (0.0, 0.0)]);
        assert!(!mp.is_simple());
    }

}
