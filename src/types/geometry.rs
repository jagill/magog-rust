use crate::types::{
    CoordinateType, Envelope, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon,
};

/// An enum representing any possible geometry type.
///
/// All `Geo` types can be converted to a `Geometry` member using `.into()` (as part of the
/// `std::convert::Into` pattern).
#[derive(PartialEq, Debug)]
pub enum Geometry<T>
where
    T: CoordinateType,
{
    Empty,
    Point(Point<T>),
    LineString(LineString<T>),
    Polygon(Polygon<T>),
    MultiPoint(MultiPoint<T>),
    MultiLineString(MultiLineString<T>),
    MultiPolygon(MultiPolygon<T>),
    // GeometryCollection(GeometryCollection<T>),
}

// FROM constructors
impl<T: CoordinateType> From<Point<T>> for Geometry<T> {
    fn from(x: Point<T>) -> Geometry<T> {
        Geometry::Point(x)
    }
}
impl<T: CoordinateType> From<LineString<T>> for Geometry<T> {
    fn from(x: LineString<T>) -> Geometry<T> {
        Geometry::LineString(x)
    }
}
impl<T: CoordinateType> From<Polygon<T>> for Geometry<T> {
    fn from(x: Polygon<T>) -> Geometry<T> {
        Geometry::Polygon(x)
    }
}
impl<T: CoordinateType> From<MultiPoint<T>> for Geometry<T> {
    fn from(x: MultiPoint<T>) -> Geometry<T> {
        Geometry::MultiPoint(x)
    }
}
impl<T: CoordinateType> From<MultiLineString<T>> for Geometry<T> {
    fn from(x: MultiLineString<T>) -> Geometry<T> {
        Geometry::MultiLineString(x)
    }
}
impl<T: CoordinateType> From<MultiPolygon<T>> for Geometry<T> {
    fn from(x: MultiPolygon<T>) -> Geometry<T> {
        Geometry::MultiPolygon(x)
    }
}

impl<T: CoordinateType> Geometry<T> {
    /// Convert empty Geometries to an official Empty.
    pub fn maybe_to_empty(self) -> Geometry<T> {
        if self.is_empty() {
            Geometry::Empty
        } else {
            self
        }
    }

    /// If this Geometry is a Point, then return that, else None.
    pub fn as_point(self) -> Option<Point<T>> {
        if let Geometry::Point(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a LineString, then return that LineString, else None.
    pub fn as_linestring(self) -> Option<LineString<T>> {
        if let Geometry::LineString(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a Polygon, then return that, else None.
    pub fn as_polygon(self) -> Option<Polygon<T>> {
        if let Geometry::Polygon(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a MultiPoint, then return that, else None.
    pub fn as_multipoint(self) -> Option<MultiPoint<T>> {
        if let Geometry::MultiPoint(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a MultiLineString, then return that, else None.
    pub fn as_multilinestring(self) -> Option<MultiLineString<T>> {
        if let Geometry::MultiLineString(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a MultiPolygon, then return that, else None.
    pub fn as_multipolygon(self) -> Option<MultiPolygon<T>> {
        if let Geometry::MultiPolygon(x) = self {
            Some(x)
        } else {
            None
        }
    }
}

impl<T: CoordinateType> Geometry<T> {
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

    pub fn envelope(&self) -> Envelope<T> {
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

    pub fn boundary(&self) -> Geometry<T> {
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
    //     // fn equals(&self, other: &Geometry<T>) -> bool;
    //     // fn disjoint(&self, other: &Geometry<T>) -> bool;
    //     // fn intersects(&self, other: &Geometry<T>) -> bool;
    //     // fn touches(&self, other: &Geometry<T>) -> bool;
    //     // fn crosses(&self, other: &Geometry<T>) -> bool;
    //     // fn within(&self, other: &Geometry<T>) -> bool;
    //     // fn contains(&self, other: &Geometry<T>) -> bool;
    //     // fn overlaps(&self, other: &Geometry<T>) -> bool;
}
