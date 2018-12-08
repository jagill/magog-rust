use super::num_traits::{Float, Num};
use std::iter::Sum;

pub trait CoordinateType: Num + Float + Sum + Copy + Clone + PartialOrd + 'static {}
impl<T: Num + Float + Sum + Copy + 'static> CoordinateType for T {}

mod geometry;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;
mod primitive;

pub use crate::types::primitive::{Coordinate, Envelope, PointLocation, Rect, Segment, Triangle};

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
