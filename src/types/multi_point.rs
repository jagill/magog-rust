use crate::types::{Coordinate, Envelope, Geometry, Point, Position};
use ordered_float::FloatIsNan;
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, PartialEq)]
pub struct MultiPoint<C: Coordinate> {
    pub points: Vec<Point<C>>,
    _envelope: Envelope<C>,
}

/// Turn a `Vec` of `Position`-ish objects into a `LineString`.
impl<C: Coordinate, IP: Into<Position<C>>> From<Vec<IP>> for MultiPoint<C> {
    fn from(v: Vec<IP>) -> Self {
        MultiPoint::new(v.into_iter().map(|p| Point(p.into())).collect())
    }
}

impl<C: Coordinate> MultiPoint<C> {
    pub fn new(points: Vec<Point<C>>) -> Self {
        let _envelope: Envelope<C> = Envelope::from(&points);
        MultiPoint { points, _envelope }
    }

    pub fn num_points(&self) -> usize {
        self.points.len()
    }
}

// GEOMETRY implementation
impl<C: Coordinate> MultiPoint<C> {
    pub fn dimension(&self) -> u8 {
        0
    }

    pub fn geometry_type(&self) -> &'static str {
        "MultiPoint"
    }

    pub fn envelope(&self) -> Envelope<C> {
        self._envelope
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub fn is_simple(&self) -> bool {
        match self.validate() {
            Err(_) => false,
            Ok(_) => true,
        }
    }

    /**
     * Validate the geometry.
     *
     * A MultiPoint is valid if it is not empty, has no invalid points, and has
     * no duplicate points.
     */
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.points.is_empty() {
            return Err("MultiPoint has no points.");
        }
        let mut position_set = HashSet::new();
        for point in &self.points {
            point.validate()?;
            match point.0.to_hashable() {
                Err(FloatIsNan) => return Err("Point contains NaN."),
                Ok(hashable) => {
                    if position_set.contains(&hashable) {
                        return Err("Duplicate point");
                    } else {
                        position_set.insert(hashable);
                    }
                }
            }
        }
        Ok(())
    }

    /**
     * Make this a simple Geometry.
     *
     * First, remove bad or duplicate points.
     * Then, if there are no remaining points, return Geometry::Empty.
     * Else, return MultiPoint with the remaining points.
     */
    pub fn make_simple(&self) -> Geometry<C> {
        // Use BTreeSet so that output is ordered and deterministic.
        let mut position_set = BTreeSet::new();
        for point in &self.points {
            if point.validate().is_err() {
                continue;
            }
            match point.0.to_hashable() {
                Err(_) => continue,
                Ok(hashable) => {
                    position_set.insert(hashable);
                }
            }
        }

        if position_set.is_empty() {
            Geometry::empty()
        } else {
            Geometry::from(MultiPoint::new(
                position_set.iter().map(|&h| Point::from(h)).collect(),
            ))
        }
    }

    pub fn boundary(&self) -> Geometry<C> {
        Geometry::empty()
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
        assert_eq!(mp.make_simple(), Geometry::empty());
    }

    #[test]
    fn check_make_simple_noop() {
        let mp = MultiPoint::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        assert_eq!(mp.make_simple(), Geometry::from(mp));
    }

    #[test]
    fn check_make_simple_dedup() {
        let mp1 = MultiPoint::from(vec![(0.0, 0.0), (1.0, 1.0), (0.0, 0.0)]);
        let mp2 = MultiPoint::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        assert_eq!(mp1.make_simple(), Geometry::from(mp2));
    }

}
