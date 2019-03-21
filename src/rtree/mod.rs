use rstar::AABB;
pub use rstar::{RTree, RTreeObject};

use crate::types::{Coordinate, CoordinateType, Segment};

fn coord_to_point<T: CoordinateType>(c: Coordinate<T>) -> [T; 2] {
    [c.x, c.y]
}

pub struct RTreeSegment<T>
where
    T: CoordinateType,
{
    pub index: usize,
    pub segment: Segment<T>,
}

impl<T: CoordinateType> RTreeObject for RTreeSegment<T> {
    type Envelope = AABB<[T; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(
            coord_to_point(self.segment.start),
            coord_to_point(self.segment.end),
        )
    }
}
