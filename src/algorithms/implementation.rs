use crate::algorithms::convex_hull::find_convex_hull_of_simple_loop;
use crate::types::{LineString, Polygon};
use crate::Coordinate;

impl<C: Coordinate> Polygon<C> {
    pub fn convex_hull(&self) -> LineString<C> {
        find_convex_hull_of_simple_loop(&self.exterior)
    }
}
