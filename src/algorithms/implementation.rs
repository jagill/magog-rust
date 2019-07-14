use crate::algorithms::convex_hull::find_convex_hull_of_simple_loop;
use crate::primitives::Coordinate;
use crate::types::{LineString, Polygon};

impl<C: Coordinate> Polygon<C> {
    pub fn convex_hull(&self) -> LineString<C> {
        find_convex_hull_of_simple_loop(&self.exterior)
    }
}
