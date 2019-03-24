use num_traits::{Bounded, Float, Signed};
use std::fmt::Debug;
use std::iter::Sum;

pub trait Coordinate: Float + Sum + Bounded + Signed + Debug + 'static {}
impl<T: Float + Sum + Bounded + Signed + Debug + 'static> Coordinate for T {}

mod geometry;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;
mod primitive;

pub use crate::types::primitive::{Coord2, Envelope, PointLocation, Rect, Segment, Triangle};

pub use crate::types::{
    geometry::Geometry, line_string::LineString, multi_line_string::MultiLineString,
    multi_point::MultiPoint, multi_polygon::MultiPolygon, point::Point, polygon::Polygon,
};

#[cfg(test)]
mod tests {

    #[test]
    fn check_trivial() {
        assert!(true);
    }

}
