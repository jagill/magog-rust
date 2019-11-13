use super::convex_hull::find_convex_hull_of_simple_loop;
use crate::planar::types::{LineString, Polygon};
use crate::Coordinate;

impl<C: Coordinate> Polygon<C> {
    pub fn convex_hull(&self) -> LineString<C> {
        find_convex_hull_of_simple_loop(&self.exterior)
    }
}
