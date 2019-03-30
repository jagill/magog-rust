use crate::types::{Coordinate, Envelope, Geometry, Point, Position};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct MultiPoint<T>
where
    T: Coordinate,
{
    pub points: Vec<Point<T>>,
    _envelope: Envelope<T>,
}

/// Turn a `Vec` of `Position`-ish objects into a `LineString`.
impl<T: Coordinate, IC: Into<Position<T>>> From<Vec<IC>> for MultiPoint<T> {
    fn from(v: Vec<IC>) -> Self {
        MultiPoint::new(v.into_iter().map(|c| Point(c.into())).collect())
    }
}

impl<T: Coordinate> MultiPoint<T> {
    pub fn new(points: Vec<Point<T>>) -> Self {
        let coords: Vec<Position<T>> = points.iter().map(|p| p.0).collect();
        let _envelope: Envelope<T> = Envelope::from(&coords);
        MultiPoint { points, _envelope }
    }

    pub fn num_points(&self) -> usize {
        self.points.len()
    }
}

// GEOMETRY implementation
impl<T: Coordinate> MultiPoint<T> {
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

    /**
     * Check if the geometry is simple.
     *
     * A MultiPoint is simple if it is not empty, has no invalid points, and has
     * no duplicate points.
     */
    pub fn is_simple(&self) -> bool {
        if self.points.is_empty() {
            return false;
        }
        let mut coord_set = HashSet::new();
        for point in &self.points {
            if point.validate().is_err() {
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

    /**
     * Make this a simple Geometry.
     *
     * First, remove bad or duplicate points.
     * Then, if there are no remaining points, return Geometry::Empty.
     * Else, return MultiPoint with the remaining points.
     */
    pub fn make_simple(&self) -> Geometry<T> {
        let mut coord_set = HashSet::new();
        for point in &self.points {
            if point.validate().is_err() {
                continue;
            }
            match point.0.to_hashable() {
                Err(_) => continue,
                Ok(hashable) => {
                    coord_set.insert(hashable);
                }
            }
        }

        if coord_set.is_empty() {
            Geometry::Empty
        } else {
            Geometry::from(MultiPoint::new(
                coord_set.iter().map(|&h| Point::from(h)).collect(),
            ))
        }
    }

    pub fn boundary(&self) -> Geometry<T> {
        Geometry::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Rect;
    use core::f32;

    #[test]
    fn check_is_simple() {
        let mp = MultiPoint::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        assert!(mp.is_simple());
    }

    #[test]
    fn check_not_is_simple_duplicate() {
        let mp = MultiPoint::from(vec![(0.0, 0.0), (1.0, 1.0), (0.0, 0.0)]);
        assert!(!mp.is_simple());
    }

    #[test]
    fn check_is_not_simple_nan() {
        let mp = MultiPoint::from(vec![(0.0, 0.0), (1.0, f32::NAN)]);
        assert!(!mp.is_simple());
    }

    #[test]
    fn check_envelope_nan() {
        let mp = MultiPoint::from(vec![(0.0, 0.0), (1.0, f32::NAN)]);
        let env = Envelope::from(Rect::from(((0.0, 0.0), (1.0, 0.0))));
        assert_eq!(mp.envelope(), env);
    }

    #[test]
    fn check_make_simple_empty() {
        let mp: MultiPoint<f32> = MultiPoint::new(Vec::new());
        assert_eq!(mp.make_simple(), Geometry::Empty);
    }

    // #[test]
    // fn check_make_simple_noop() {
    //     let mp = MultiPoint::from(vec![(0.0, 0.0), (1.0, 1.0)]);
    //     assert_eq!(mp.make_simple(), Geometry::from(mp));
    // }

    // #[test]
    // fn check_make_simple_dedup() {
    //     let mp1 = MultiPoint::from(vec![(0.0, 0.0), (1.0, 1.0), (0.0, 0.0)]);
    //     let mp2 = MultiPoint::from(vec![(0.0, 0.0), (1.0, 1.0)]);
    //     assert_eq!(mp1.make_simple(), Geometry::from(mp2));
    // }

}
