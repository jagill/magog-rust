use crate::primitives::Coordinate;
use crate::types::MultiPolygon;

impl<C: Coordinate> MultiPolygon<C> {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.polygons.len() == 0 {
            return Err("MultiPolygon has no Polygon.");
        }

        for polygon in self.polygons.iter() {
            polygon.validate()?;
        }

        Ok(())
    }
}
