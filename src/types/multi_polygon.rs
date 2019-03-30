use crate::types::{Coordinate, Envelope, Geometry, Point, Polygon};

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

// MultiPolygon implementation
impl<C: Coordinate> MultiPolygon<C> {
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
impl<C: Coordinate> MultiPolygon<C> {
    pub fn dimension(&self) -> u8 {
        2
    }

    pub fn geometry_type(&self) -> &'static str {
        "MultiPolygon"
    }

    pub fn envelope(&self) -> Envelope<C> {
        self._envelope
    }

    pub fn is_empty(&self) -> bool {
        self.polygons.iter().all(|p| p.is_empty())
    }

    /// A MultiPolygon is simple if it has no self-intersections in or between the Polygons.
    pub fn is_simple(&self) -> bool {
        self.polygons.iter().all(|p| p.is_simple())
            // TODO: STUB  Should be a pair-wise check for disjoint
            && true
    }

    /// The boundary of a MultiPolygon is the boundaries of the Polygons.
    pub fn boundary(&self) -> Geometry<C> {
        // TODO: STUB  Should be a union of the boundaries of the component polygons.
        Geometry::Empty
    }
}
