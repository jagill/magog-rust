use rstar::AABB;
pub use rstar::{RTree, RTreeObject};

use crate::types::{Coord2, Coordinate, Segment};

fn coord_to_point<T: Coordinate>(c: Coord2<T>) -> [T; 2] {
    [c.x, c.y]
}

pub struct RTreeSegment<T>
where
    T: Coordinate,
{
    pub index: usize,
    pub segment: Segment<T>,
}

impl<T: Coordinate> RTreeObject for RTreeSegment<T> {
    type Envelope = AABB<[T; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(
            coord_to_point(self.segment.start),
            coord_to_point(self.segment.end),
        )
    }
}
