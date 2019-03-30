use crate::types::{
    Coordinate, Envelope, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon,
};

/// An enum representing any possible geometry type.
///
/// All `Geo` types can be converted to a `Geometry` member using `.into()` (as part of the
/// `std::convert::Into` pattern).
#[derive(PartialEq, Debug)]
pub enum Geometry<C: Coordinate> {
    Empty,
    Point(Point<C>),
    LineString(LineString<C>),
    Polygon(Polygon<C>),
    MultiPoint(MultiPoint<C>),
    MultiLineString(MultiLineString<C>),
    MultiPolygon(MultiPolygon<C>),
    // GeometryCollection(GeometryCollection<C>),
}

// FROM constructors
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
    /// Convert empty Geometries to an official Empty.
    pub fn maybe_to_empty(self) -> Geometry<C> {
        if self.is_empty() {
            Geometry::Empty
        } else {
            self
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

impl<C: Coordinate> Geometry<C> {
    // Basic accessors
    pub fn dimension(&self) -> u8 {
        match self {
            Geometry::Empty => 0,
            Geometry::Point(x) => x.dimension(),
            Geometry::MultiPoint(x) => x.dimension(),
            Geometry::LineString(x) => x.dimension(),
            Geometry::MultiLineString(x) => x.dimension(),
            Geometry::Polygon(x) => x.dimension(),
            Geometry::MultiPolygon(x) => x.dimension(),
        }
    }

    pub fn geometry_type(&self) -> &'static str {
        match self {
            Geometry::Empty => "Empty",
            Geometry::Point(x) => x.geometry_type(),
            Geometry::MultiPoint(x) => x.geometry_type(),
            Geometry::LineString(x) => x.geometry_type(),
            Geometry::MultiLineString(x) => x.geometry_type(),
            Geometry::Polygon(x) => x.geometry_type(),
            Geometry::MultiPolygon(x) => x.geometry_type(),
        }
    }

    pub fn envelope(&self) -> Envelope<C> {
        match self {
            Geometry::Empty => Envelope::empty(),
            Geometry::Point(x) => x.envelope(),
            Geometry::MultiPoint(x) => x.envelope(),
            Geometry::LineString(x) => x.envelope(),
            Geometry::MultiLineString(x) => x.envelope(),
            Geometry::Polygon(x) => x.envelope(),
            Geometry::MultiPolygon(x) => x.envelope(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Geometry::Empty => true,
            Geometry::Point(x) => x.is_empty(),
            Geometry::MultiPoint(x) => x.is_empty(),
            Geometry::LineString(x) => x.is_empty(),
            Geometry::MultiLineString(x) => x.is_empty(),
            Geometry::Polygon(x) => x.is_empty(),
            Geometry::MultiPolygon(x) => x.is_empty(),
        }
    }

    pub fn is_simple(&self) -> bool {
        match self {
            Geometry::Empty => true,
            Geometry::Point(x) => x.is_simple(),
            Geometry::MultiPoint(x) => x.is_simple(),
            Geometry::LineString(x) => x.is_simple(),
            Geometry::MultiLineString(x) => x.is_simple(),
            Geometry::Polygon(x) => x.is_simple(),
            Geometry::MultiPolygon(x) => x.is_simple(),
        }
    }

    pub fn boundary(&self) -> Geometry<C> {
        match self {
            Geometry::Empty => Geometry::Empty,
            Geometry::Point(x) => x.boundary(),
            Geometry::MultiPoint(x) => x.boundary(),
            Geometry::LineString(x) => x.boundary(),
            Geometry::MultiLineString(x) => x.boundary(),
            Geometry::Polygon(x) => x.boundary(),
            Geometry::MultiPolygon(x) => x.boundary(),
        }
    }

    //     // Intersection Relations
    //     // fn equals(&self, other: &Geometry<C>) -> bool;
    //     // fn disjoint(&self, other: &Geometry<C>) -> bool;
    //     // fn intersects(&self, other: &Geometry<C>) -> bool;
    //     // fn touches(&self, other: &Geometry<C>) -> bool;
    //     // fn crosses(&self, other: &Geometry<C>) -> bool;
    //     // fn within(&self, other: &Geometry<C>) -> bool;
    //     // fn contains(&self, other: &Geometry<C>) -> bool;
    //     // fn overlaps(&self, other: &Geometry<C>) -> bool;
}
