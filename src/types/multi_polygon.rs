use crate::types::{CoordinateType, Envelope, Geometry, Point, Polygon};

#[derive(Debug, PartialEq)]
pub struct MultiPolygon<T>
where
    T: CoordinateType,
{
    pub polygons: Vec<Polygon<T>>,
    _envelope: Envelope<T>,
}

impl<T: CoordinateType> MultiPolygon<T> {
    pub fn new(polygons: Vec<Polygon<T>>) -> Self {
        let envs: Vec<Envelope<T>> = polygons.iter().map(|p| p.envelope()).collect();
        let _envelope = Envelope::from(&envs);
        MultiPolygon {
            polygons,
            _envelope,
        }
    }
}

// MultiPolygon implementation
impl<T: CoordinateType> MultiPolygon<T> {
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
impl<T: CoordinateType> MultiPolygon<T> {
    pub fn dimension(&self) -> u8 {
        2
    }

    pub fn geometry_type(&self) -> &'static str {
        "MultiPolygon"
    }

    pub fn envelope(&self) -> Envelope<T> {
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
    pub fn boundary(&self) -> Geometry<T> {
        // TODO: STUB  Should be a union of the boundaries of the component polygons.
        Geometry::Empty
    }
}
