use crate::primitives::{Coordinate, Envelope, HasEnvelope, SegmentIntersection};
use crate::types::{Geometry, LineString, Point};

#[derive(Debug, PartialEq)]
pub struct MultiLineString<C: Coordinate> {
    pub line_strings: Vec<LineString<C>>,
    _envelope: Envelope<C>,
}

impl<C: Coordinate> MultiLineString<C> {
    pub fn new(line_strings: Vec<LineString<C>>) -> Self {
        let envs: Vec<Envelope<C>> = line_strings.iter().map(|ls| ls.envelope()).collect();
        let _envelope = Envelope::from(&envs);
        MultiLineString {
            line_strings,
            _envelope,
        }
    }
}

/// Turn a `Vec` of `LineString`-ish objects into a `MultiLineString`.
impl<C: Coordinate, IL: Into<LineString<C>>> From<Vec<IL>> for MultiLineString<C> {
    fn from(v: Vec<IL>) -> Self {
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

// GEOMETRY implementation
impl<C: Coordinate> MultiLineString<C> {
    pub fn dimension(&self) -> u8 {
        1
    }

    pub fn geometry_type(&self) -> &'static str {
        "MultiLineString"
    }

    pub fn envelope(&self) -> Envelope<C> {
        self._envelope
    }

    pub fn is_empty(&self) -> bool {
        self.line_strings.iter().all(|ls| ls.is_empty())
    }

    /// The boundary of a MultiLineString is are the boundaries of
    /// the component LineStrings that don't touch any other LineString.
    pub fn boundary(&self) -> Geometry<C> {
        // TODO: STUB
        Geometry::empty()
    }

    /// A MultiLineString is simple if each LineString is simple, and none
    /// intersect each other.
    pub fn is_simple(&self) -> bool {
        match self.validate() {
            Err(_) => false,
            Ok(_) => true,
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.line_strings.len() == 0 {
            return Err("MultiLineString has no LineStrings.");
        }
        let intersection_err = Err("Intersection between LineStrings.");
        let mut rtrees = Vec::new();
        for (i1, line_string) in self.line_strings.iter().enumerate() {
            let rtree1 = line_string._validate_with_rtree()?;
            for i2 in 0..i1 {
                // If i2 isn't a key, something is deeply wrong.
                let line_string2 = &self.line_strings[i2];
                // If there's no chance of intersection, move along.
                // A more efficient RTree check would take care of this.
                if !(line_string.envelope().intersects(line_string2.envelope())) {
                    continue;
                }
                let rtree2 = &rtrees[i2];
                for (rtree_seg1, rtree_seg2) in
                    rtree1.intersection_candidates_with_other_tree(rtree2)
                {
                    match rtree_seg1.segment.intersect_segment(rtree_seg2.segment) {
                        SegmentIntersection::None => continue,
                        SegmentIntersection::Segment(_) => {
                            return intersection_err;
                        }
                        SegmentIntersection::Position(pos) => {
                            // Allow linestrings to intersect at their endpoints.
                            if let (Geometry::MultiPoint(mp1), Geometry::MultiPoint(mp2)) =
                                (line_string.boundary(), line_string2.boundary())
                            {
                                let point = Point(pos);
                                if !(mp1.contains_point(&point) && mp2.contains_point(&point)) {
                                    return intersection_err;
                                }
                            } else {
                                return intersection_err;
                            }
                        }
                    }
                }
            }
            rtrees.push(rtree1);
        }
        Ok(())
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
    fn check_empty_not_simple() {
        let ls: Vec<LineString<f32>> = Vec::new();
        let mls = MultiLineString::new(ls);
        assert!(!mls.is_simple());
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
