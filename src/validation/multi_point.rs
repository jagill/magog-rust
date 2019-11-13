use crate::types::MultiPoint;
use crate::Coordinate;
use ordered_float::FloatIsNan;
use std::collections::HashSet;

impl<C: Coordinate> MultiPoint<C> {
    /**
     * Validate the geometry.
     *
     * A MultiPoint is valid if it:
     * 1. is empty, or
     * 2. has no invalid points and no duplicate points.
     */
    pub fn validate(&self) -> Result<(), &'static str> {
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
