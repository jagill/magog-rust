use crate::flatbush::{Flatbush, FLATBUSH_DEFAULT_DEGREE};
use crate::planar::primitives::SegmentIntersection;
use crate::planar::types::{Geometry, MultiLineString, Point};
use crate::Coordinate;

impl<C: Coordinate> MultiLineString<C> {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.line_strings.len() == 0 {
            // Empty multilinestrings are valid empty geometries.
            return Ok(());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planar::types::LineString;

    #[test]
    fn test_valid_microsoft_examples() {
        assert!(MultiLineString::from(Vec::<LineString<f32>>::new())
            .validate()
            .is_ok());

        assert!(MultiLineString::from(
            vec![vec![(1., 1.), (3., 5.)], vec![(-5., 3.), (-8., -2.)],]
        )
        .validate()
        .is_ok());
        assert!(
            MultiLineString::from(vec![vec![(0., 2.), (1., 1.)], vec![(1., 0.), (1., 1.)],])
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn test_invalid_microsoft_examples() {
        assert!(
            MultiLineString::from(vec![vec![(1., 1.), (5., 5.)], vec![(3., 1.), (1., 3.)],])
                .validate()
                .is_err()
        );
        assert!(MultiLineString::from(vec![
            vec![(1., 1.), (3., 3.), (5., 5.)],
            vec![(3., 3.), (5., 5.), (7., 7.)],
        ])
        .validate()
        .is_err());
        assert!(
            MultiLineString::from(vec![vec![(1., 1.), (3., 5.)], vec![(-5., 3.)],])
                .validate()
                .is_err()
        );
    }
}
