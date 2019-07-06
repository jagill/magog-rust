use crate::algorithms::loop_relation::{find_loop_loop_relation, LoopLoopRelation};
use crate::flatbush::{Flatbush, FLATBUSH_DEFAULT_DEGREE};
use crate::primitives::Coordinate;
use crate::types::Polygon;

impl<C: Coordinate> Polygon<C> {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.exterior.is_closed() {
            return Err("Exterior is not a loop.");
        };
        self.exterior.validate()?;
        for interior in &self.interiors {
            if !interior.is_closed() {
                return Err("Interior linestring is not a loop.");
            };
            interior.validate()?;
            if find_loop_loop_relation(&self.exterior, &interior) != LoopLoopRelation::Contains {
                return Err("Interior loop not contained in exterior loop.");
            }
        }

        let rtree_of_interiors = Flatbush::new(&self.interiors, FLATBUSH_DEFAULT_DEGREE);
        for (ls1_id, ls2_id) in rtree_of_interiors.find_self_intersection_candidates() {
            let linestring_1 = &self.interiors[ls1_id];
            let linestring_2 = &self.interiors[ls2_id];
            if find_loop_loop_relation(linestring_1, linestring_2) != LoopLoopRelation::Separate {
                return Err("Two Interior rings intersect.");
            }
        }

        Ok(())
    }
}
