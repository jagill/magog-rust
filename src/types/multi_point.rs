use crate::types::{Coordinate, CoordinateType, Envelope, Geometry, Point};
use std::collections::HashSet;

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
        let mut coord_set = HashSet::new();
        for point in &self.points {
            if let Err(_) = point.validate() {
                return false;
            }
            match point.0.to_hashable() {
                Err(_) => return false,
                Ok(hashable) => {
                    if coord_set.contains(&hashable) {
                        return false;
                    } else {
                        coord_set.insert(hashable);
                    }
                }
            }
        }
        true
    }

    /// Remove bad or duplicate points.
    pub fn make_simple(&self) -> Self {
        let mut coord_set = HashSet::new();
        for point in &self.points {
            if let Err(_) = point.validate() {
                continue;
            }
            match point.0.to_hashable() {
                Err(_) => continue,
                Ok(hashable) => {
                    coord_set.insert(hashable);
                }
            }
        }
        return MultiPoint::new(coord_set.iter().map(|&h| Point::from(h)).collect());
    }

    pub fn boundary(&self) -> Geometry<T> {
        Geometry::Empty
    }
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
