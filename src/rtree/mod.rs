use crate::primitives::Rect;
use crate::types::LineString;
use rstar::AABB;
pub use rstar::{RTree, RTreeObject};

use crate::primitives::{Coordinate, Position, Segment};

fn position_to_point<C: Coordinate>(p: Position<C>) -> [C; 2] {
    [p.x, p.y]
}

#[derive(Debug)]
pub struct RTreeSegment<C: Coordinate> {
    pub id: usize,
    pub segment: Segment<C>,
}

impl<C: Coordinate> RTreeObject for RTreeSegment<C> {
    type Envelope = AABB<[C; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(
            position_to_point(self.segment.start),
            position_to_point(self.segment.end),
        )
    }
}

impl<C: Coordinate> Rect<C> {
    pub fn from_aabb(aabb: &AABB<[C; 2]>) -> Self {
        (aabb.lower(), aabb.upper()).into()
    }

    pub fn to_aabb(&self) -> AABB<[C; 2]> {
        AABB::from_corners(position_to_point(self.min), position_to_point(self.max))
    }
}

pub(crate) fn build_rtree<C: Coordinate>(line_string: &LineString<C>) -> RTree<RTreeSegment<C>> {
    RTree::bulk_load(
        line_string
            .segments_iter()
            .enumerate()
            .map(|(i, seg)| RTreeSegment {
                id: i,
                segment: seg,
            })
            .collect(),
    )
}
