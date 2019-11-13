mod empty;
mod geometry;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;

pub use crate::planar::types::{
    empty::Empty, geometry::Geometry, line_string::LineString, multi_line_string::MultiLineString,
    multi_point::MultiPoint, multi_polygon::MultiPolygon, point::Point, polygon::Polygon,
};

#[cfg(test)]
mod tests {

    #[test]
    fn check_trivial() {
        assert!(true);
    }
}
