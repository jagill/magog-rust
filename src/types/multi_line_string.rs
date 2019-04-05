use crate::rtree::intersection_candidates;
use crate::types::primitive::SegmentIntersection;
use crate::types::{Coordinate, Envelope, Geometry, LineString};

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
        let mut rtrees = Vec::new();
        for line_string in self.line_strings.iter() {
            let rtree1 = line_string._validate_with_rtree()?;
            for rtree2 in &rtrees {
                for (rtree_seg1, rtree_seg2) in intersection_candidates(&rtree1, rtree2) {
                    // TODO: Allow linestrings to intersect at their endpoints.
                    match rtree_seg1.segment.intersect_segment(rtree_seg2.segment) {
                        SegmentIntersection::None => continue,
                        _ => return Err("Intersection between LineStrings."),
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
    fn check_ribbon_not_simple() {
        let mls = MultiLineString::new(vec![
            LineString::from(vec![(0., 0.), (0.5, 0.5)]),
            LineString::from(vec![(0.5, 0.5), (1., 1.), (0., 1.), (0.5, 0.5)]),
            LineString::from(vec![(0.5, 0.5), (1., 0.)]),
        ]);
        // Second LS is a loop and has no boundary, so the isxn is invalid.
        assert!(!mls.is_simple());
    }
}
