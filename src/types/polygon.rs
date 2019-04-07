use crate::primitives::{Coordinate, Envelope};
use crate::types::{Geometry, LineString, Point};

#[derive(Debug, PartialEq)]
pub struct Polygon<C: Coordinate> {
    pub exterior: LineString<C>,
    pub interiors: Vec<LineString<C>>,
    _envelope: Envelope<C>,
}

/// Turn a `Vec` of `Position`-ish objects into a `Polygon`.
impl<C: Coordinate, ILS: Into<LineString<C>>> From<ILS> for Polygon<C> {
    fn from(ext: ILS) -> Self {
        let exterior: LineString<C> = ext.into();
        let _envelope = exterior.envelope().clone();
        Polygon {
            exterior,
            interiors: vec![],
            _envelope,
        }
    }
}

impl<C: Coordinate> Polygon<C> {
    pub fn new(exterior: LineString<C>, interiors: Vec<LineString<C>>) -> Polygon<C> {
        let _envelope = exterior.envelope().clone();
        Polygon {
            exterior,
            interiors,
            _envelope,
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        &self.exterior.validate()?;
        if !&self.exterior.is_closed() {
            return Err("Exterior is not a loop.");
        };
        for interior in &self.interiors {
            interior.validate()?;
            if !interior.is_closed() {
                return Err("Interior linestring is not a loop.");
            };
        }
        Ok(())
    }
}

// Polygon implementation
impl<C: Coordinate> Polygon<C> {
    pub fn centroid(&self) -> Point<C> {
        // TODO: STUB
        Point::from((C::zero(), C::zero()))
    }

    pub fn point_on_surface(&self) -> Point<C> {
        // TODO: STUB
        Point::from((C::zero(), C::zero()))
    }
}

// GEOMETRY implementation
impl<C: Coordinate> Polygon<C> {
    pub fn dimension(&self) -> u8 {
        2
    }

    pub fn geometry_type(&self) -> &'static str {
        "Polygon"
    }

    pub fn envelope(&self) -> Envelope<C> {
        self._envelope
    }

    pub fn is_empty(&self) -> bool {
        // TODO: STUB
        false
    }

    /// A Polygon is simple if it has no self-intersections in its envelopes.
    pub fn is_simple(&self) -> bool {
        self.exterior.is_simple()
            && self.interiors.iter().all(|ls| ls.is_simple())
            // TODO: STUB  Should be that none of the rings intersect.
            && true
    }

    /// The boundary of a Polygon are the component LineStrings.
    pub fn boundary(&self) -> Geometry<C> {
        // TODO: STUB
        Geometry::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_basic_polygon() {
        let p = Polygon::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, 0.0)]);
        assert_eq!(p.exterior.num_points(), 4);
        assert_eq!(p.interiors.len(), 0);
    }
}
