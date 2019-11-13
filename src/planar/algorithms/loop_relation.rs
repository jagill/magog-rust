use crate::Coordinate;
use std::collections::{HashMap, HashSet};

use crate::flatbush::{Flatbush, FlatbushNode, FLATBUSH_DEFAULT_DEGREE};
use crate::planar::primitives::{
    Envelope, HasEnvelope, Position, SafePosition, Segment, SegmentIntersection,
};
use crate::planar::types::LineString;

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

/**
 * Check how loop_1 relates to loop_2.
 *
 * Separate: The two linestrings are outside each other, except for a finite
 *   number of points.
 * Contains: loop_2 is inside of loop_1, except for a finite number of points.
 * Within: loop_1 is inside of loop_2, except for a finite number of points.
 * Crosses: Either
 *   1. loop_1 has points outside and inside of loop_2, and vice versa, or
 *   2. they intersect at an infinite number of points (necessarily dimension 1).
 *
 * Note if loop_1 or loop_2 are not actually loops, the result will be meaningless.
 * The caller is responsible for checking that loop_1 and loop_2 are actually loops.
 * This assumes the loops are valid linestrings.  If any positions contain an NAN,
 * this may panic.
 *
 */
#[allow(dead_code)]
pub fn find_loop_loop_relation<C: Coordinate>(
    loop_1: &LineString<C>,
    loop_2: &LineString<C>,
) -> LoopLoopRelation {
    if !loop_1.envelope().intersects(loop_2.envelope()) {
        return LoopLoopRelation::Separate;
    }

    let segments_1 = loop_1.segments_iter().collect();
    let segments_2 = loop_2.segments_iter().collect();
    let rtree_1 = Flatbush::new_unsorted(&segments_1, FLATBUSH_DEFAULT_DEGREE);
    let rtree_2 = Flatbush::new_unsorted(&segments_2, FLATBUSH_DEFAULT_DEGREE);

    let mut stack = Vec::with_capacity(rtree_1.degree + rtree_2.degree);
    stack.push((rtree_1.root_node(), rtree_2.root_node()));

    let mut winding_numbers_1: HashMap<SafePosition<C>, i32> = HashMap::new();
    let mut winding_numbers_2: HashMap<SafePosition<C>, i32> = HashMap::new();
    let mut boundary_positions: HashSet<SafePosition<C>> = HashSet::new();

    while let Some((node1, node2)) = stack.pop() {
        if node1.level == 0 && node2.level == 0 {
            let seg1 = segments_1[node1.sibling_index];
            let seg2 = segments_2[node2.sibling_index];
            let mut intersecting_position: Option<Position<C>> = None;
            match seg1.intersect_segment(seg2) {
                SegmentIntersection::Segment(_s) => {
                    return LoopLoopRelation::Crosses;
                }
                SegmentIntersection::Position(p) => {
                    if ![seg1.start, seg1.end, seg2.start, seg2.end].contains(&p) {
                        return LoopLoopRelation::Crosses;
                    } else {
                        intersecting_position = Some(p);
                        boundary_positions.insert(p.to_hashable().unwrap());
                    }
                }
                SegmentIntersection::None => (),
            }

            if Some(seg1.start) != intersecting_position {
                let wn1 = winding_numbers_1
                    .entry(seg1.start.to_hashable().unwrap())
                    .or_insert(0);
                *wn1 += Segment::find_winding_number(seg1.start, seg2);
            }

            if Some(seg2.start) != intersecting_position {
                let wn2 = winding_numbers_2
                    .entry(seg2.start.to_hashable().unwrap())
                    .or_insert(0);
                *wn2 += Segment::find_winding_number(seg2.start, seg1);
            }
        } else if node1.level >= node2.level {
            for child1 in rtree_1.get_children(node1) {
                _maybe_push_other_isxn(child1, node2, &mut stack);
            }
        } else {
            // node2.level > node1.level
            for child2 in rtree_2.get_children(node2) {
                _maybe_push_other_isxn(node1, child2, &mut stack);
            }
        }
    }

    let loop_1_relation = _get_loop_relation(&winding_numbers_1, &boundary_positions);
    if loop_1_relation == LoopRelation::Crosses {
        return LoopLoopRelation::Crosses;
    }
    let loop_2_relation = _get_loop_relation(&winding_numbers_2, &boundary_positions);
    if loop_2_relation == LoopRelation::Crosses {
        return LoopLoopRelation::Crosses;
    }

    match (loop_1_relation, loop_2_relation) {
        (LoopRelation::Outside, LoopRelation::Outside) => LoopLoopRelation::Separate,
        (LoopRelation::Inside, LoopRelation::Outside) => LoopLoopRelation::Within,
        (LoopRelation::Outside, LoopRelation::Inside) => LoopLoopRelation::Contains,
        (LoopRelation::Inside, LoopRelation::Inside) => {
            panic!("BUG: Two loops found inside each other.")
        }
        (LoopRelation::Crosses, LoopRelation::Crosses) => LoopLoopRelation::Crosses,
        (LoopRelation::Crosses, _) | (_, LoopRelation::Crosses) => {
            panic!("BUG: One loop found Crossing but not the other")
        }
    }
}

fn _maybe_push_other_isxn<C: Coordinate>(
    node1: FlatbushNode<C>,
    node2: FlatbushNode<C>,
    stack: &mut Vec<(FlatbushNode<C>, FlatbushNode<C>)>,
) {
    if let (Envelope::Bounds(r1), Envelope::Bounds(r2)) = (node1.envelope, node2.envelope) {
        // right is a position that will extend r1 to the right edge of r2.
        let right = Position::new(r2.max.x, r1.max.y);
        if r1.add_position(right).intersects(r2) {
            stack.push((node1, node2));
        }
    }
}

fn _get_loop_relation<C: Coordinate>(
    winding_numbers: &HashMap<SafePosition<C>, i32>,
    boundary_positions: &HashSet<SafePosition<C>>,
) -> LoopRelation {
    let mut outside = false;
    let mut inside = false;
    for pos in winding_numbers.keys() {
        if boundary_positions.contains(pos) {
            continue;
        }
        let wn = winding_numbers[pos];
        let is_even = wn % 2 == 0;
        outside = outside || is_even;
        inside = inside || !is_even;
        if outside && inside {
            return LoopRelation::Crosses;
        }
    }
    if outside {
        LoopRelation::Outside
    } else {
        LoopRelation::Inside
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_loop_separate() {
        let loop_a = LineString::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, 0.0)]);
        let loop_b = LineString::from(vec![(2.0, 0.0), (2.0, 1.0), (3.0, 0.0), (2.0, 0.0)]);
        assert_eq!(
            find_loop_loop_relation(&loop_a, &loop_b),
            LoopLoopRelation::Separate
        )
    }

    #[test]
    fn check_loop_equal_crossing() {
        let loop_a = LineString::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, 0.0)]);
        let loop_b = LineString::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, 0.0)]);
        assert_eq!(
            find_loop_loop_relation(&loop_a, &loop_b),
            LoopLoopRelation::Crosses
        )
    }

    #[test]
    fn check_loop_contains() {
        let loop_a = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 3.0),
            (3.0, 3.0),
            (3.0, 0.0),
            (0.0, 0.0),
        ]);
        let loop_b = LineString::from(vec![
            (1.0, 1.0),
            (1.0, 2.0),
            (2.0, 2.0),
            (2.0, 1.0),
            (1.0, 1.0),
        ]);
        assert_eq!(
            find_loop_loop_relation(&loop_a, &loop_b),
            LoopLoopRelation::Contains
        )
    }

    #[test]
    fn check_loop_within() {
        let loop_a = LineString::from(vec![
            (1.0, 1.0),
            (1.0, 2.0),
            (2.0, 2.0),
            (2.0, 1.0),
            (1.0, 1.0),
        ]);
        let loop_b = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 3.0),
            (3.0, 3.0),
            (3.0, 0.0),
            (0.0, 0.0),
        ]);
        assert_eq!(
            find_loop_loop_relation(&loop_a, &loop_b),
            LoopLoopRelation::Within
        )
    }

    #[test]
    fn check_loop_crosses() {
        let loop_a = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 2.0),
            (2.0, 2.0),
            (2.0, 0.0),
            (0.0, 0.0),
        ]);
        let loop_b = LineString::from(vec![
            (1.0, 1.0),
            (1.0, 3.0),
            (3.0, 3.0),
            (3.0, 1.0),
            (1.0, 1.0),
        ]);
        assert_eq!(
            find_loop_loop_relation(&loop_a, &loop_b),
            LoopLoopRelation::Crosses
        )
    }

    #[test]
    fn check_loop_crosses_at_vertex_1() {
        let loop_a = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 2.0),
            (1.0, 2.0),
            (2.0, 2.0),
            (2.0, 1.0),
            (2.0, 0.0),
            (0.0, 0.0),
        ]);
        let loop_b = LineString::from(vec![
            (1.0, 1.0),
            (1.0, 3.0),
            (3.0, 3.0),
            (3.0, 1.0),
            (1.0, 1.0),
        ]);
        assert_eq!(
            find_loop_loop_relation(&loop_a, &loop_b),
            LoopLoopRelation::Crosses
        )
    }

    #[test]
    fn check_loop_crosses_at_vertex_2() {
        let loop_a = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 2.0),
            (2.0, 2.0),
            (2.0, 0.0),
            (0.0, 0.0),
        ]);
        let loop_b = LineString::from(vec![
            (1.0, 1.0),
            (1.0, 2.0),
            (1.0, 3.0),
            (3.0, 3.0),
            (3.0, 1.0),
            (2.0, 1.0),
            (1.0, 1.0),
        ]);
        assert_eq!(
            find_loop_loop_relation(&loop_a, &loop_b),
            LoopLoopRelation::Crosses
        )
    }

    #[test]
    fn check_loop_crosses_at_vertex_1_and_2() {
        let loop_a = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 2.0),
            (1.0, 2.0),
            (2.0, 2.0),
            (2.0, 1.0),
            (2.0, 0.0),
            (0.0, 0.0),
        ]);
        let loop_b = LineString::from(vec![
            (1.0, 1.0),
            (1.0, 2.0),
            (1.0, 3.0),
            (3.0, 3.0),
            (3.0, 1.0),
            (2.0, 1.0),
            (1.0, 1.0),
        ]);
        assert_eq!(
            find_loop_loop_relation(&loop_a, &loop_b),
            LoopLoopRelation::Crosses
        )
    }

    #[test]
    fn check_loop_crosses_a() {
        let loop_a = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 2.0),
            (2.0, 2.0),
            (2.0, 0.0),
            (0.0, 0.0),
        ]);
        let loop_b = LineString::from(vec![
            (1.0, 1.0),
            (2.0, 2.0),
            (3.0, 1.0),
            (2.0, 0.0),
            (1.0, 1.0),
        ]);
        assert_eq!(
            find_loop_loop_relation(&loop_a, &loop_b),
            LoopLoopRelation::Crosses
        )
    }

    #[test]
    fn check_loop_crosses_b() {
        let loop_a = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 2.0),
            (1.0, 2.0),
            (2.0, 1.0),
            (1.0, 0.0),
            (0.0, 0.0),
        ]);
        let loop_b = LineString::from(vec![
            (1.0, 0.0),
            (1.0, 2.0),
            (3.0, 2.0),
            (3.0, 0.0),
            (1.0, 0.0),
        ]);
        assert_eq!(
            find_loop_loop_relation(&loop_a, &loop_b),
            LoopLoopRelation::Crosses
        )
    }

    #[test]
    fn check_loop_contains_with_vertex_1() {
        let loop_a = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 3.0),
            (2.0, 3.0),
            (2.0, 0.0),
            (0.0, 0.0),
        ]);
        let loop_b = LineString::from(vec![(1.0, 1.0), (1.0, 2.0), (2.0, 2.0), (1.0, 1.0)]);
        assert_eq!(
            find_loop_loop_relation(&loop_a, &loop_b),
            LoopLoopRelation::Contains
        )
    }

    #[test]
    fn check_loop_contains_with_vertex_2() {
        let loop_a = LineString::from(vec![
            (0.0, 0.0),
            (0.0, 3.0),
            (2.0, 3.0),
            (2.0, 2.0),
            (2.0, 0.0),
            (0.0, 0.0),
        ]);
        let loop_b = LineString::from(vec![(1.0, 1.0), (1.0, 2.0), (2.0, 2.0), (1.0, 1.0)]);
        assert_eq!(
            find_loop_loop_relation(&loop_a, &loop_b),
            LoopLoopRelation::Contains
        )
    }

    #[test]
    fn check_loops_touching_at_corner() {
        let loop_a = LineString::from(vec![(1., 1.), (1., -1.), (-1., -1.), (-1., 1.), (1., 1.)]);
        let loop_b = LineString::from(vec![(1., 1.), (3., 1.), (3., 3.), (1., 3.), (1., 1.)]);
        assert_eq!(
            find_loop_loop_relation(&loop_a, &loop_b),
            LoopLoopRelation::Separate
        )
    }
}
