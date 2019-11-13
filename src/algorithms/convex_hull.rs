#![allow(dead_code)]
use crate::primitives::{Position, PositionLocation, Segment};
use crate::types::LineString;
use crate::Coordinate;
/**
 * Algorithms for calculating convex hulls.
 */
use std::collections::VecDeque;

/**
 * This finds the convex hull of a simple loop (eg, a polygon exterior).
 *
 * This algorithm is from
 * A. A. Melkman, in "On-line Construction of the Convex Hull of a Simple
 * Polyline", Information Processing Letters 25 (1987).
 *
 * If the loop is not simple, or not a loop, the convex hull will be wrong.
 * If the loop has fewer than 4 positions (which means it is not simple), the
 * algorithm will panic.
 */
pub(crate) fn find_convex_hull_of_simple_loop<C: Coordinate>(
    aloop: &LineString<C>,
) -> LineString<C> {
    let mut points = aloop.positions.iter().copied();
    let mut deque = VecDeque::new();

    points.next(); // Drop first point; it's also the last so we'll get to it.
    let a = points.next().unwrap();
    let b = points.next().unwrap();
    let c = points.next().unwrap();
    deque.push_front(c);
    match _triple_location(a, b, c) {
        PositionLocation::Right => {
            deque.push_back(a);
            deque.push_back(b);
        }
        _ => {
            deque.push_back(b);
            deque.push_back(a);
        }
    }
    deque.push_back(c);
    while let Some(pos) = points.next() {
        if !(_triple_location(pos, deque[0], deque[1]) == PositionLocation::Left
            || _triple_location(deque[deque.len() - 2], deque[deque.len() - 1], pos)
                == PositionLocation::Left)
        {
            continue;
        }
        while _triple_location(deque[deque.len() - 2], deque[deque.len() - 1], pos)
            != PositionLocation::Right
        {
            deque.pop_back();
        }
        deque.push_back(pos);
        while _triple_location(pos, deque[0], deque[1]) != PositionLocation::Right {
            deque.pop_front();
        }
        deque.push_front(pos);
    }

    // This algorithm builds the positions in the reverse order.
    LineString::collect_from(deque.into_iter().rev())
}

fn _triple_location<C: Coordinate>(
    a: Position<C>,
    b: Position<C>,
    c: Position<C>,
) -> PositionLocation {
    Segment::new(a, b).position_location(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_loops_equiv<C: Coordinate>(loop1: &mut LineString<C>, loop2: &mut LineString<C>) {
        assert!(loop1.is_closed());
        assert!(loop2.is_closed());

        let positions1 = &mut loop1.positions;
        positions1.pop();
        let positions2 = &mut loop2.positions;
        positions2.pop();
        let rot_index = positions2
            .iter()
            .position(|p| Some(p) == positions1.first());
        assert!(rot_index.is_some());
        let pos_slice1 = &positions1[..];
        let pos_slice2 = &mut positions2[..];
        pos_slice2.rotate_left(rot_index.unwrap());
        assert_eq!(pos_slice1, pos_slice2);
    }

    #[test]
    fn check_triangle_convex_hull() {
        let mut ls = LineString::from(vec![(0., 0.), (1., 0.), (0., 1.), (0., 0.)]);
        let mut hull = find_convex_hull_of_simple_loop(&ls);
        assert_loops_equiv(&mut hull, &mut ls);
    }

    #[test]
    fn check_star_convex_hull() {
        let ls = LineString::from(vec![
            (0., 0.),
            (0.5, 0.4),
            (1., 0.),
            (0.6, 0.5),
            (1., 1.),
            (0.5, 0.6),
            (0., 1.),
            (0.4, 0.5),
            (0., 0.),
        ]);
        let mut hull = find_convex_hull_of_simple_loop(&ls);
        let mut target = LineString::from(vec![(0., 0.), (1., 0.), (1., 1.), (0., 1.), (0., 0.)]);
        assert_loops_equiv(&mut hull, &mut target);
    }

    #[test]
    fn check_old_face_convex_hull() {
        let ls = LineString::from(vec![
            (0., 0.),
            (2., 0.),
            (2., 1.),
            (4., 2.),
            (3., 3.),
            (2., 3.),
            (4., 4.),
            (1., 4.),
            (0., 4.),
            (0., 0.),
        ]);
        let mut hull = find_convex_hull_of_simple_loop(&ls);
        let mut target = LineString::from(vec![
            (0., 0.),
            (2., 0.),
            (4., 2.),
            (4., 4.),
            (0., 4.),
            (0., 0.),
        ]);
        assert_loops_equiv(&mut hull, &mut target);
    }
}
