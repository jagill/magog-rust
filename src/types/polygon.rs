use crate::types::{CoordinateType, Envelope, Geometry, LineString, Point};

#[derive(Debug, PartialEq)]
pub struct Polygon<T>
where
    T: CoordinateType,
{
    pub exterior: LineString<T>,
    pub interiors: Vec<LineString<T>>,
    _envelope: Envelope<T>,
}

/// Turn a `Vec` of `Coord2`-ish objects into a `Polygon`.
impl<T: CoordinateType, ILS: Into<LineString<T>>> From<ILS> for Polygon<T> {
    fn from(ext: ILS) -> Self {
        let exterior: LineString<T> = ext.into();
        let _envelope = exterior.envelope().clone();
        Polygon {
            exterior,
            interiors: vec![],
            _envelope,
        }
    }
}

impl<T> Polygon<T>
where
    T: CoordinateType,
{
    pub fn new(exterior: LineString<T>, interiors: Vec<LineString<T>>) -> Polygon<T> {
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
impl<T: CoordinateType> Polygon<T> {
    pub fn centroid(&self) -> Point<T> {
        // TODO: STUB
        Point::from((T::zero(), T::zero()))
    }

    pub fn point_on_surface(&self) -> Point<T> {
        // TODO: STUB
        Point::from((T::zero(), T::zero()))
    }
}

// GEOMETRY implementation
impl<T: CoordinateType> Polygon<T>
where
    T: CoordinateType,
{
    pub fn dimension(&self) -> u8 {
        2
    }

    pub fn geometry_type(&self) -> &'static str {
        "Polygon"
    }

    pub fn envelope(&self) -> Envelope<T> {
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
    pub fn boundary(&self) -> Geometry<T> {
        // TODO: STUB
        Geometry::Empty
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
