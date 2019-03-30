use rstar::AABB;
pub use rstar::{RTree, RTreeObject};

use crate::types::{Coordinate, Position, Segment};

fn coord_to_point<C: Coordinate>(p: Position<C>) -> [C; 2] {
    [p.x, p.y]
}

pub struct RTreeSegment<C: Coordinate> {
    pub index: usize,
    pub segment: Segment<C>,
}

impl<C: Coordinate> RTreeObject for RTreeSegment<C> {
    type Envelope = AABB<[C; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(
            coord_to_point(self.segment.start),
            coord_to_point(self.segment.end),
        )
    }
}
