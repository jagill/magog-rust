use crate::planar::primitives::{Envelope, HasEnvelope, Position};
use crate::planar::types::{Geometry, LineString, MultiPoint};
use crate::Coordinate;
use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
pub struct MultiLineString<C: Coordinate> {
    pub line_strings: Vec<LineString<C>>,
    _envelope: Envelope<C>,
}

impl<C: Coordinate> MultiLineString<C> {
    pub fn new(line_strings: Vec<LineString<C>>) -> Self {
        let _envelope = Envelope::of(line_strings.iter());
        MultiLineString {
            line_strings,
            _envelope,
        }
    }
}

/// Turn a `Vec` of `LineString`-ish objects into a `MultiLineString`.
impl<C: Coordinate, L: Into<LineString<C>>> From<Vec<L>> for MultiLineString<C> {
    fn from(v: Vec<L>) -> Self {
        MultiLineString::new(v.into_iter().map(|l| l.into()).collect())
    }
}

// MultiLineString implementation
impl<C: Coordinate> MultiLineString<C> {
    pub fn is_closed(&self) -> bool {
        self.line_strings.iter().all(|ls| ls.is_closed())
    }

    pub fn length(&self) -> C {
        self.line_strings.iter().map(|ls| ls.length()).sum()
    }
}

impl<C: Coordinate> HasEnvelope<C> for MultiLineString<C> {
    fn envelope(&self) -> Envelope<C> {
        self._envelope
    }
}

// GEOMETRY implementation
impl<C: Coordinate> MultiLineString<C> {
    pub fn dimension(&self) -> u8 {
        1
    }

    pub fn geometry_type(&self) -> &'static str {
        "MultiLineString"
    }

    pub fn is_empty(&self) -> bool {
        self.line_strings.iter().all(|ls| ls.is_empty())
    }

    /**
     * A point is in the MultiLineString boundary if it is in an odd-number of
     * boundaries of the compoinent LineStrings.
     */
    pub fn boundary(&self) -> Geometry<C> {
        // Use BTreeSet to ensure consistent output ordering.
        let mut boundary = BTreeSet::new();
        self.line_strings
            .iter()
            .filter_map(|ls| ls.boundary().as_multipoint())
            .flat_map(|mp| mp.points)
            .filter_map(|point| point.0.to_hashable().ok())
            .for_each(|pos| {
                if boundary.contains(&pos) {
                    boundary.remove(&pos);
                } else {
                    boundary.insert(pos);
                }
            });
        if boundary.is_empty() {
            Geometry::empty()
        } else {
            let positions: Vec<Position<C>> = boundary.iter().map(|p| Position::from(*p)).collect();
            MultiPoint::from(positions).into()
        }
    }

    /// A MultiLineString is simple if each LineString is simple, and none
    /// intersect each other.
    pub fn is_simple(&self) -> bool {
        self.validate().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_basic_simple() {
        let mls = MultiLineString::new(vec![
            LineString::from(vec![(0., 0.), (0.4, 0.4)]),
            LineString::from(vec![(0.5, 0.5), (1., 1.), (0., 1.), (0.5, 0.5)]),
        ]);
        assert!(mls.is_simple());
    }

    #[test]
    fn check_empty_simple() {
        let ls: Vec<LineString<f32>> = Vec::new();
        let mls = MultiLineString::new(ls);
        assert!(mls.is_simple());
    }

    #[test]
    fn check_non_simple_linestring_not_simple() {
        let mls = MultiLineString::new(vec![LineString::from(vec![(0.0, 0.0)])]);
        assert!(!mls.is_simple());
    }

    #[test]
    fn check_ribbon_not_simple() {
        let mls = MultiLineString::new(vec![
            LineString::from(vec![(0., 0.), (0.5, 0.5)]),
            LineString::from(vec![(0.5, 0.5), (1., 1.), (0., 1.), (0.5, 0.5)]),
            LineString::from(vec![(0.5, 0.5), (1., 0.)]),
        ]);
        // Second LS is a loop and has no boundary, so the isxn is invalid.
        assert!(!mls.is_simple());
    }

    #[test]
    fn check_cross_not_simple() {
        let mls = MultiLineString::new(vec![
            LineString::from(vec![(0., 0.), (1., 1.)]),
            LineString::from(vec![(0., 1.), (0., 1.)]),
        ]);
        assert!(!mls.is_simple());
    }

    #[test]
    fn check_long_line_simple() {
        // Since their intersection is the boundary of each, this is simple.
        let mls = MultiLineString::new(vec![
            LineString::from(vec![(0., 0.), (1., 0.)]),
            LineString::from(vec![(1., 0.), (1., 1.)]),
        ]);
        assert!(mls.is_simple());
    }

    #[test]
    fn check_box_simple() {
        // Since their intersection is the boundary of each, this is simple.
        let mls = MultiLineString::new(vec![
            LineString::from(vec![(0., 0.), (1., 0.)]),
            LineString::from(vec![(1., 0.), (1., 1.), (0., 0.)]),
        ]);
        assert!(mls.is_simple());
    }
}
