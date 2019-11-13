use crate::flatbush::Flatbush;
use crate::planar::primitives::SegmentIntersection;
use crate::planar::types::LineString;
use crate::Coordinate;

impl<C: Coordinate> LineString<C> {
    /**
     * Validate the geometry.
     *
     * A LineString is valid if it has 2 or more positions, has no repeated
     * positions, and has no self-intersections, except possibly last_point
     * and first_point being the same.
     */
    pub fn validate(&self) -> Result<(), &'static str> {
        match self._validate_with_rtree() {
            Err(s) => Err(s),
            _ => Ok(()),
        }
    }

    /*
     * The workhouse fn for validation.
     * It does the work, but also returns the constructed rtree, which can be
     * used for additional validation checks, eg for MultiLineString.
     */
    pub(crate) fn _validate_with_rtree(&self) -> Result<Flatbush<C>, &'static str> {
        if self.is_empty() {
            // LineStrings with no points are valid empty geometries
            return Ok(Flatbush::new_empty());
        } else if self.num_points() == 1 {
            // Must have at least 2 points to be 1-dimensional.
            return Err("LineString must have at least 2 points.");
        }

        // Declare the errors here
        let repeated_err = Err("LineString has repeated points.");
        let intersection_err = Err("LineString has self-intersection.");

        for seg in self.segments_iter() {
            // First check: should have finite coordinates.
            seg.start.validate()?;
            seg.end.validate()?;
            // Second check: should not have two same adjacent points.
            if seg.start == seg.end {
                return repeated_err;
            }
        }

        let rtree = self.build_rtree();
        let intersections = rtree.find_self_intersection_candidates();

        let num_segments = self.num_points() - 1;
        for (low_id, high_id) in intersections {
            let first_segment = self.get_segment(low_id);
            let second_segment = self.get_segment(high_id);
            match first_segment.intersect_segment(second_segment) {
                SegmentIntersection::None => continue,
                SegmentIntersection::Position(p) => {
                    // Point intersections are fine at the shared point between
                    // adjacent segments.  In loops this includes the wraparound.
                    if ((high_id == low_id + 1) || (low_id == 0 && high_id == num_segments - 1))
                        && (p == first_segment.end || p == first_segment.start)
                    {
                        continue;
                    } else {
                        return intersection_err;
                    }
                }
                SegmentIntersection::Segment(_) => {
                    // Segment intersxns are always bad
                    return intersection_err;
                }
            }
        }
        Ok(rtree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planar::primitives::Position;

    #[test]
    fn test_valid_microsoft_examples() {
        assert!(LineString::from(Vec::<Position<f32>>::new())
            .validate()
            .is_ok());
        assert!(
            LineString::from(vec![(1., 1.), (2., 3.), (4., 8.), (-6., 3.)])
                .validate()
                .is_ok()
        );
        assert!(LineString::from(vec![(1., 1.), (3., 3.)])
            .validate()
            .is_ok());
    }

    #[test]
    fn test_invalid_microsoft_examples() {
        assert!(LineString::from(vec![(1., 1.)]).validate().is_err());
        assert!(LineString::from(vec![(1., 1.), (1., 1.)])
            .validate()
            .is_err());
        assert!(
            LineString::from(vec![(1., 4.), (3., 4.), (2., 4.), (2., 0.)])
                .validate()
                .is_err()
        );
        // The following two are "valid" by Microsoft's criteria, but non-simple.
        // This library equates valid and simple, as per the SQL3 spec.
        assert!(
            LineString::from(vec![(1., 1.), (3., 3.), (2., 4.), (2., 0.)])
                .validate()
                .is_err()
        );
        assert!(
            LineString::from(vec![(1., 1.), (3., 3.), (2., 4.), (2., 0.), (1., 1.)])
                .validate()
                .is_err()
        );
    }
}
