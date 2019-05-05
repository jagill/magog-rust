use crate::primitives::{Coordinate, Position, Rect, Segment, SegmentIntersection};
use crate::rtree::{RTree, RTreeSegment};
use rstar::Envelope;
use rstar::RTreeNode::{Leaf, Parent};

#[derive(PartialEq, Debug)]
pub enum LoopLoopRelation {
    Separate,
    Contains,
    Within,
    Crosses,
}

#[derive(PartialEq, Debug)]
pub(crate) enum LoopRelation {
    Outside,
    Inside,
    Crosses,
}

pub fn find_loop_loop_relation<C: Coordinate>(
    rtree_a: &RTree<RTreeSegment<C>>,
    rtree_b: &RTree<RTreeSegment<C>>,
) -> LoopLoopRelation {
    let env_a = rtree_a.root().envelope();
    let env_b = rtree_b.root().envelope();
    if !env_a.intersects(&env_b) {
        return LoopLoopRelation::Separate;
    }

    match _find_loop_loop_relation(&rtree_a, &rtree_b) {
        // None means one of the rtrees is empty, so the loop is empty.
        // We define a relation with an empty loop as Separate.
        None => LoopLoopRelation::Separate,
        Some(LoopRelation::Crosses) => LoopLoopRelation::Crosses,
        // b is inside a
        Some(LoopRelation::Inside) => LoopLoopRelation::Contains,
        Some(LoopRelation::Outside) => match _find_loop_loop_relation(&rtree_b, &rtree_a) {
            None => LoopLoopRelation::Separate,
            // If b crosses a, a should have crossed b
            Some(LoopRelation::Crosses) => panic!("Bug: Loop A outside B, but B Crosses A."),
            // a is inside b
            Some(LoopRelation::Inside) => LoopLoopRelation::Within,
            Some(LoopRelation::Outside) => LoopLoopRelation::Separate,
        },
    }
}

/**
 * Check for basic relation of other to base.
 *
 * This method assumes
 *   1. !other.envelope().contains_envelope(base.envelope())
 *   2. other.envelope().intersects(base.envelope())
 *
 * The returned relation is other to base, so Inside means all of other's segments are
 * inside base.
 * It will return None if other is empty.
 */
fn _find_loop_loop_relation<C: Coordinate>(
    base: &RTree<RTreeSegment<C>>,
    other: &RTree<RTreeSegment<C>>,
) -> Option<LoopRelation> {
    let mut inside = false;
    let mut outside = false;
    let mut stack = Vec::new();
    stack.extend(other.root().children());

    while let Some(node) = stack.pop() {
        match node {
            Parent(p) => match loop_rect_relation(base, Rect::from_aabb(&p.envelope())) {
                LoopRelation::Inside => inside = true,
                LoopRelation::Outside => outside = true,
                LoopRelation::Crosses => stack.extend(p.children()),
            },
            Leaf(t) => match loop_segment_relation(base, t.segment) {
                LoopRelation::Inside => inside = true,
                LoopRelation::Outside => outside = true,
                LoopRelation::Crosses => return Some(LoopRelation::Crosses),
            },
        }
        if inside && outside {
            return Some(LoopRelation::Crosses);
        }
    }
    // It should be impossible to have inside && outside, but let's check it anyway
    if inside && outside {
        Some(LoopRelation::Crosses)
    } else if inside {
        Some(LoopRelation::Inside)
    } else if outside {
        Some(LoopRelation::Outside)
    } else {
        // This means we started with an empty base or other
        None
    }
}

/**
 * Check if the rect is inside a loop.
 *
 * The possible outcomes:
 *   1. The rect is entirely outside the loop, not touching at any points.
 *   2. The rect is entirely inside the loop, not touching at any points.
 *   3. The rect touches the loop at one or more points.
 *
 * For case 1, this returns Outside.
 * For case 2, this returns Inside.
 * For case 3, this returns Crosses.
 *
 * This assumes the LineString is closed and valid; if not the answer is meaningless.
 */
pub(crate) fn loop_rect_relation<C: Coordinate>(
    loop_rtree: &RTree<RTreeSegment<C>>,
    rect: Rect<C>,
) -> LoopRelation {
    let root_rect = Rect::from_aabb(&loop_rtree.root().envelope());
    if !root_rect.intersects(rect) {
        return LoopRelation::Outside;
    }
    // We'll find all segments that intersect a ray to x=+inf
    let right_pos = Position::new(root_rect.max.x, rect.max.y);
    let query_env = rect.add_position(right_pos).to_aabb();
    // loop through all edges of the loop that intersect our ray.
    let right_segments = loop_rtree
        .locate_in_envelope_intersecting(&query_env)
        .map(|x| x.segment);

    // Decompose rect into segments and positions to check individually
    let tr = rect.max;
    let bl = rect.min;
    let tl = Position::new(rect.min.x, rect.max.y);
    let br = Position::new(rect.max.x, rect.min.y);

    // The winding numbers for each corner
    let mut tl_wn: i32 = 0;
    let mut tr_wn: i32 = 0;
    let mut bl_wn: i32 = 0;
    let mut br_wn: i32 = 0;
    for other_seg in right_segments {
        if rect_intersect_segment(rect, other_seg) {
            return LoopRelation::Crosses;
        }
        tl_wn += Segment::find_winding_number(tl, other_seg);
        tr_wn += Segment::find_winding_number(tr, other_seg);
        bl_wn += Segment::find_winding_number(bl, other_seg);
        br_wn += Segment::find_winding_number(br, other_seg);
    }

    let wns = vec![tl_wn, tr_wn, bl_wn, br_wn];
    if wns.iter().all(|wn| wn % 2 == 0) {
        // If all corners are outside, the rect is outside
        LoopRelation::Outside
    } else if wns.iter().all(|wn| wn % 2 == 1) {
        // If all corners are inside, the rect is inside
        LoopRelation::Inside
    } else {
        // If some are inside and some are outside, the rect crosses
        LoopRelation::Crosses
    }
}

fn rect_intersect_segment<C: Coordinate>(rect: Rect<C>, seg: Segment<C>) -> bool {
    if !rect.intersects(Rect::from(seg)) {
        return false;
    }
    let tl = Position::new(rect.min.x, rect.max.y);
    let tr = Position::new(rect.max.x, rect.max.y);
    let bl = Position::new(rect.min.x, rect.min.y);
    let br = Position::new(rect.max.x, rect.min.y);
    !vec![
        Segment::new(tl, bl),
        Segment::new(tr, br),
        Segment::new(tl, tr),
        Segment::new(bl, br),
    ]
    .iter()
    .all(|s| s.intersect_segment(seg) == SegmentIntersection::None)
}

/**
 * Check if the segment is inside a loop.
 *
 * The possible outcomes:
 *   1. The segment is entirely outside the loop, possibly one or both endpoints are on the loop.
 *   2. The segment is entirely inside the loop, possibly one or both endpoints are on the loop.
 *   3. The segment intersects the loop, partly in and partly out.
 *   4. The segment overlaps one or more segments of the loop.
 *
 * For case 1, this returns Outside.
 * For case 2, this returns Inside.
 * For cases 3 & 4, this returns Crosses.
 *
 * This assumes the LineString is closed and valid; if not the answer is meaningless.
 */
pub(crate) fn loop_segment_relation<C: Coordinate>(
    loop_rtree: &RTree<RTreeSegment<C>>,
    segment: Segment<C>,
) -> LoopRelation {
    let rect = Rect::from(segment);

    let root_rect = Rect::from_aabb(&loop_rtree.root().envelope());
    if !root_rect.intersects(rect) {
        return LoopRelation::Outside;
    }
    // We'll find all segments that intersect a ray to x=+inf
    let right_pos = Position::new(root_rect.max.x, rect.max.y);
    let query_env = rect.add_position(right_pos).to_aabb();
    // loop through all edges of the loop that intersect our ray.
    let right_segments = loop_rtree
        .locate_in_envelope_intersecting(&query_env)
        .map(|x| x.segment);

    // The winding numbers for each end of the segment
    let mut start_wn: i32 = 0;
    let mut end_wn: i32 = 0;
    for other_seg in right_segments {
        if segment_crosses_segment(segment, other_seg) {
            // FIXME: This will trigger if other_seg touches segment at segment's
            // interior, but other_seg's boundary.  This should be ok!
            // Consider:
            //  Loop \| Segment
            //       /|
            return LoopRelation::Crosses;
        }
        start_wn += Segment::find_winding_number(segment.start, other_seg);
        end_wn += Segment::find_winding_number(segment.end, other_seg);
    }

    if (start_wn % 2 == 0) && (end_wn % 2 == 0) {
        // If both ends are outside, the segment is outside
        LoopRelation::Outside
    } else if (start_wn % 2 == 1) && (end_wn % 2 == 1) {
        // If both ends are inside, the segment is inside
        LoopRelation::Inside
    } else {
        // If one is inside and one is outside, the segment crosses
        LoopRelation::Crosses
    }
}

fn segment_crosses_segment<C: Coordinate>(segment: Segment<C>, other_seg: Segment<C>) -> bool {
    Rect::from(segment).intersects(Rect::from(other_seg))
        && match segment.intersect_segment(other_seg) {
            SegmentIntersection::None => false,
            SegmentIntersection::Segment(_) => true,
            SegmentIntersection::Position(p) => !(p == segment.start || p == segment.end),
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rtree::build_rtree;
    use crate::types::LineString;

    fn _get_relation<C: Coordinate>(
        loop_a: LineString<C>,
        loop_b: LineString<C>,
    ) -> LoopLoopRelation {
        find_loop_loop_relation(&build_rtree(&loop_a), &build_rtree(&loop_b))
    }

    #[test]
    fn check_loops_separate() {
        let loop_a = LineString::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, 0.0)]);
        let loop_b = LineString::from(vec![(10.0, 0.0), (10.0, 1.0), (11.0, 0.0), (10.0, 0.0)]);
        assert_eq!(_get_relation(loop_a, loop_b), LoopLoopRelation::Separate)
    }

    #[test]
    fn check_loop_contains() {
        let loop_a = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (0.0, 0.0),
        ]);
        let loop_b = LineString::from(vec![
            (0.25, 0.25),
            (0.25, 0.75),
            (0.75, 0.75),
            (0.75, 0.25),
            (0.25, 0.25),
        ]);
        assert_eq!(_get_relation(loop_a, loop_b), LoopLoopRelation::Contains)
    }

    #[test]
    fn check_loop_within() {
        let loop_a = LineString::from(vec![
            (0.25, 0.25),
            (0.25, 0.75),
            (0.75, 0.75),
            (0.75, 0.25),
            (0.25, 0.25),
        ]);
        let loop_b = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (0.0, 0.0),
        ]);
        assert_eq!(_get_relation(loop_a, loop_b), LoopLoopRelation::Within)
    }

    #[test]
    fn check_loop_crosses() {
        let loop_a = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (0.0, 0.0),
        ]);
        let loop_b = LineString::from(vec![
            (0.5, 0.0),
            (0.5, 1.0),
            (1.5, 1.0),
            (1.5, 0.0),
            (0.5, 0.0),
        ]);
        assert_eq!(_get_relation(loop_a, loop_b), LoopLoopRelation::Crosses)
    }

    #[test]
    fn check_loop_crosses_at_vertex() {
        let loop_a = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 1.0),
            (0.5, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (0.5, 0.0),
            (0.0, 0.0),
        ]);
        let loop_b = LineString::from(vec![
            (2.0, 2.0),
            (0.5, 2.0),
            (0.5, 1.0),
            (0.5, 0.0),
            (0.5, -1.0),
            (2.0, -1.0),
            (2.0, 2.0),
        ]);
        assert_eq!(_get_relation(loop_a, loop_b), LoopLoopRelation::Crosses)
    }

}
