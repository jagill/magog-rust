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

pub fn intersection_candidates<'a, T: RTreeObject>(
    tree1: &'a RTree<T>,
    tree2: &'a RTree<T>,
) -> Vec<(&'a T, &'a T)> {
    // This is inefficient.  Replace with more efficient implementation if
    // https://github.com/Stoeoef/rstar/issues/6 is resolved.
    let probe;
    let build;
    let mut switched = false;
    if tree1.size() <= tree2.size() {
        probe = tree1;
        build = tree2;
    } else {
        probe = tree2;
        build = tree1;
        switched = true;
    }
    // We can now assume probe is smaller than/equal to build, so iterate through
    // probe, and check in build.
    let mut candidates = Vec::new();
    for p in probe.into_iter() {
        for b in build.locate_in_envelope_intersecting(&p.envelope()) {
            if switched {
                candidates.push((b, p))
            } else {
                candidates.push((p, b))
            }
        }
    }

    candidates
}
