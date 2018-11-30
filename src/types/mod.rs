use super::num_traits::{Float, Num};
use std::iter::Sum;

pub trait CoordinateType: Num + Float + Sum + Copy + Clone + PartialOrd + 'static {}
impl<T: Num + Float + Sum + Copy + 'static> CoordinateType for T {}


mod primitive;
mod geometry;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;

pub use crate::types::primitive::{Coordinate, Envelope, Rect, Segment, PointLocation, Triangle};

pub use crate::types::{
    line_string::LineString, multi_line_string::MultiLineString, multi_point::MultiPoint,
    multi_polygon::MultiPolygon, point::Point, polygon::Polygon, geometry::Geometry,
};

#[cfg(test)]
mod tests {

    #[test]
    fn check_trivial() {
        assert!(true);
    }

}
