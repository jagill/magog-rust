use crate::primitives::{Envelope, HasEnvelope};
use crate::types::{Geometry, MultiLineString, Point, Polygon};
use crate::Coordinate;

#[derive(Debug, PartialEq)]
pub struct MultiPolygon<C: Coordinate> {
    pub polygons: Vec<Polygon<C>>,
    _envelope: Envelope<C>,
}

impl<C: Coordinate> MultiPolygon<C> {
    pub fn new(polygons: Vec<Polygon<C>>) -> Self {
        let envs: Vec<Envelope<C>> = polygons.iter().map(|p| p.envelope()).collect();
        let _envelope = Envelope::from(&envs);
        MultiPolygon {
            polygons,
            _envelope,
        }
    }
}

/// Turn a `Vec` of `Polygon`-ish objects into a `MultiPolygon`.
impl<C: Coordinate, P: Into<Polygon<C>>> From<Vec<P>> for MultiPolygon<C> {
    fn from(v: Vec<P>) -> Self {
        MultiPolygon::new(v.into_iter().map(|p| p.into()).collect())
    }
}

// MultiPolygon implementation
impl<C: Coordinate> MultiPolygon<C> {
    pub fn centroid(&self) -> Point<C> {
        // TODO: STUB
        Point::from((C::zero(), C::zero()))
    }

    pub fn point_on_surface(&self) -> Option<Point<C>> {
        let polys = &mut self.polygons.iter().filter(|p| !p.is_empty());
        polys.next()?.point_on_surface()
    }
}

impl<C: Coordinate> HasEnvelope<C> for MultiPolygon<C> {
    fn envelope(&self) -> Envelope<C> {
        self._envelope
    }
}

// GEOMETRY implementation
impl<C: Coordinate> MultiPolygon<C> {
    pub fn dimension(&self) -> u8 {
        2
    }

    pub fn geometry_type(&self) -> &'static str {
        "MultiPolygon"
    }

    pub fn is_empty(&self) -> bool {
        self.polygons.iter().all(|p| p.is_empty())
    }

    /// A MultiPolygon is simple if it has no self-intersections in or between the Polygons.
    pub fn is_simple(&self) -> bool {
        self.validate().is_ok()
    }

    /// The boundary of a MultiPolygon is the boundaries of the Polygons.
    pub fn boundary(&self) -> Geometry<C> {
        let line_strings = self
            .polygons
            .iter()
            .map(|p| p.boundary())
            .filter(|g| !g.is_empty())
            .filter_map(|g| g.as_multilinestring())
            .flat_map(|mls| mls.line_strings)
            .collect();
        MultiLineString::new(line_strings).into()
    }
}
