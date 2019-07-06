use crate::primitives::Coordinate;
use crate::types::MultiPoint;
use ordered_float::FloatIsNan;
use std::collections::HashSet;

impl<C: Coordinate> MultiPoint<C> {
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
}
