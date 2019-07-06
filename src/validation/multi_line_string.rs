use crate::flatbush::{Flatbush, FLATBUSH_DEFAULT_DEGREE};
use crate::primitives::{Coordinate, SegmentIntersection};
use crate::types::{Geometry, MultiLineString, Point};

impl<C: Coordinate> MultiLineString<C> {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.line_strings.len() == 0 {
            return Err("MultiLineString has no LineStrings.");
        }
        let intersection_err = Err("Intersection between LineStrings.");

        for linestring in &self.line_strings {
            linestring.validate()?;
        }
        let rtree_of_linestrings = Flatbush::new(&self.line_strings, FLATBUSH_DEFAULT_DEGREE);

        for (ls1_id, ls2_id) in rtree_of_linestrings.find_self_intersection_candidates() {
            let linestring1 = &self.line_strings[ls1_id];
            let linestring2 = &self.line_strings[ls2_id];
            let rtree1 = linestring1.build_rtree();
            let rtree2 = linestring2.build_rtree();
            for (seg1_id, seg2_id) in rtree1.find_other_rtree_intersection_candidates(&rtree2) {
                let seg1 = linestring1.get_segment(seg1_id);
                let seg2 = linestring2.get_segment(seg2_id);
                match seg1.intersect_segment(seg2) {
                    SegmentIntersection::None => continue,
                    SegmentIntersection::Segment(_) => {
                        return intersection_err;
                    }
                    SegmentIntersection::Position(pos) => {
                        // Allow linestrings to intersect at their endpoints.
                        if let (Geometry::MultiPoint(mp1), Geometry::MultiPoint(mp2)) =
                            (linestring1.boundary(), linestring2.boundary())
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
        Ok(())
    }
}
