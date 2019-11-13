use crate::planar::primitives::{Envelope, HasEnvelope};
use crate::planar::types::{
    Empty, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon,
};
use crate::Coordinate;

/// An enum representing any possible geometry type.
///
/// All `Geo` types can be converted to a `Geometry` member using `.into()` (as part of the
/// `std::convert::Into` pattern).
#[derive(PartialEq, Debug)]
pub enum Geometry<C: Coordinate> {
    Empty(Empty<C>),
    Point(Point<C>),
    LineString(LineString<C>),
    Polygon(Polygon<C>),
    MultiPoint(MultiPoint<C>),
    MultiLineString(MultiLineString<C>),
    MultiPolygon(MultiPolygon<C>),
    // GeometryCollection(GeometryCollection<C>),
}

// FROM constructors
impl<C: Coordinate> From<Empty<C>> for Geometry<C> {
    fn from(_x: Empty<C>) -> Geometry<C> {
        Geometry::Empty(Empty::new())
    }
}
impl<C: Coordinate> From<Point<C>> for Geometry<C> {
    fn from(x: Point<C>) -> Geometry<C> {
        Geometry::Point(x)
    }
}
impl<C: Coordinate> From<LineString<C>> for Geometry<C> {
    fn from(x: LineString<C>) -> Geometry<C> {
        Geometry::LineString(x)
    }
}
impl<C: Coordinate> From<Polygon<C>> for Geometry<C> {
    fn from(x: Polygon<C>) -> Geometry<C> {
        Geometry::Polygon(x)
    }
}
impl<C: Coordinate> From<MultiPoint<C>> for Geometry<C> {
    fn from(x: MultiPoint<C>) -> Geometry<C> {
        Geometry::MultiPoint(x)
    }
}
impl<C: Coordinate> From<MultiLineString<C>> for Geometry<C> {
    fn from(x: MultiLineString<C>) -> Geometry<C> {
        Geometry::MultiLineString(x)
    }
}
impl<C: Coordinate> From<MultiPolygon<C>> for Geometry<C> {
    fn from(x: MultiPolygon<C>) -> Geometry<C> {
        Geometry::MultiPolygon(x)
    }
}

impl<C: Coordinate> Geometry<C> {
    // Convenience constructor for empty geometry
    pub fn empty() -> Geometry<C> {
        Geometry::from(Empty::new())
    }

    /// Convert empty Geometries to an official Empty.
    pub fn as_empty(self) -> Option<Empty<C>> {
        if self.is_empty() {
            Some(Empty::new())
        } else {
            None
        }
    }

    /// If this Geometry is a Point, then return that, else None.
    pub fn as_point(self) -> Option<Point<C>> {
        if let Geometry::Point(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a LineString, then return that LineString, else None.
    pub fn as_linestring(self) -> Option<LineString<C>> {
        if let Geometry::LineString(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a Polygon, then return that, else None.
    pub fn as_polygon(self) -> Option<Polygon<C>> {
        if let Geometry::Polygon(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a MultiPoint, then return that, else None.
    pub fn as_multipoint(self) -> Option<MultiPoint<C>> {
        if let Geometry::MultiPoint(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a MultiLineString, then return that, else None.
    pub fn as_multilinestring(self) -> Option<MultiLineString<C>> {
        if let Geometry::MultiLineString(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a MultiPolygon, then return that, else None.
    pub fn as_multipolygon(self) -> Option<MultiPolygon<C>> {
        if let Geometry::MultiPolygon(x) = self {
            Some(x)
        } else {
            None
        }
    }
}

macro_rules! delegate_accessor {
    // This macro takes the name of an accessor function and delegates it
    // to each of the options of the Geometry Enum.
    ($func_name:ident, $ret_type:ty) => (
        pub fn $func_name(&self) -> $ret_type {
            match self {
                Geometry::Empty(x) => x.$func_name(),
                Geometry::Point(x) => x.$func_name(),
                Geometry::MultiPoint(x) => x.$func_name(),
                Geometry::LineString(x) => x.$func_name(),
                Geometry::MultiLineString(x) => x.$func_name(),
                Geometry::Polygon(x) => x.$func_name(),
                Geometry::MultiPolygon(x) => x.$func_name(),
            }
        }
    )
}

impl<C: Coordinate> Geometry<C> {
    // Basic accessors
    delegate_accessor!(dimension, u8);
    delegate_accessor!(geometry_type, &'static str);
    delegate_accessor!(envelope, Envelope<C>);
    delegate_accessor!(is_empty, bool);
    delegate_accessor!(is_simple, bool);
    delegate_accessor!(boundary, Geometry<C>);

    // Intersection Relations
    // fn equals(&self, other: &Geometry<C>) -> bool;
    // fn disjoint(&self, other: &Geometry<C>) -> bool;
    // fn intersects(&self, other: &Geometry<C>) -> bool;
    // fn touches(&self, other: &Geometry<C>) -> bool;
    // fn crosses(&self, other: &Geometry<C>) -> bool;
    // fn within(&self, other: &Geometry<C>) -> bool;
    // fn contains(&self, other: &Geometry<C>) -> bool;
    // fn overlaps(&self, other: &Geometry<C>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planar::primitives::Position;

    #[test]
    fn check_dim_point() {
        assert_eq!(Geometry::from(Point::from((0.0, 1.0))).dimension(), 0);
    }

    #[test]
    fn check_dim_linestring() {
        assert_eq!(
            Geometry::from(LineString::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)])).dimension(),
            1
        );
    }

    #[test]
    fn check_type_point() {
        assert_eq!(
            Geometry::from(Point::from((0.0, 1.0))).geometry_type(),
            "Point"
        );
    }

    #[test]
    fn check_type_linestring() {
        assert_eq!(
            Geometry::from(LineString::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)]))
                .geometry_type(),
            "LineString"
        );
    }

    #[test]
    fn check_envelope_point() {
        let p = (0.0, 1.0);
        assert_eq!(
            Geometry::from(Point::from(p)).envelope(),
            Envelope::from((p, p))
        );
    }

    #[test]
    fn check_envelope_linestring() {
        let positions = vec![
            Position::new(0.0, 0.0),
            Position::new(0.0, 1.0),
            Position::new(1.0, 1.0),
        ];
        assert_eq!(
            Geometry::from(LineString::from(positions.clone())).envelope(),
            Envelope::from(&positions)
        );
    }
}
