use crate::types::Coordinate;
use crate::types::CoordinateType;
use crate::types::LineString;
use crate::types::Point;
use crate::types::Polygon;
use crate::types::Triangle;

pub fn polygon_contains_point<T>(
    polygon: &Polygon<T>,
    point: &Point<T>,
) -> Result<bool, &'static str>
where
    T: CoordinateType,
{
    // If it's not in the envelope, it's not in the polygon.
    if !polygon.envelope.contains(point.0) {
        return Ok(false);
    }
    let ext_wn: i32 = find_winding_number(point, &polygon.exterior)?;
    // If it's not in exterior ring, it's not in the polygon.
    if ext_wn == 0 {
        return Ok(false);
    }
    // If it's inside the exerior ring, but inside an interior ring, it's not contained.
    let not_contained_in_interiors = polygon
        .interiors
        .iter()
        .map(|ls| find_winding_number(point, ls).unwrap())
        .all(|wn| wn == 0);
    Ok(not_contained_in_interiors)
}

fn find_winding_number<T>(point: &Point<T>, ls: &LineString<T>) -> Result<i32, &'static str>
where
    T: CoordinateType,
{
    let mut wn: i32 = 0; // the winding number counter
    if !ls.is_closed() {
        return Err("Cannot find winding number of a non-loop LineString.");
    }

    // loop through all edges of the polygon
    for seg in ls.segments_iter() {
        if seg.start.y <= point.0.y {
            if seg.end.y > point.0.y  // an upward crossing
                 && line_point_position(&seg.start, &seg.end, &point.0) == PointLocation::Left
            {
                wn += 1; // have  a valid up intersect
            }
        } else {
            // seg.start.y > P.y (no test needed)
            if seg.end.y <= point.0.y  // a downward crossing
                 && line_point_position(&seg.start, &seg.end, &point.0) == PointLocation::Right
            {
                wn -= 1; // have  a valid down intersect
            }
        }
    }
    Ok(wn)
}

/// Location of a point in relation to a line
#[derive(PartialEq, Clone, Debug)]
enum PointLocation {
    Left,
    On,
    Right,
}

/// Tests if a coordinate is Left|On|Right of an infinite line.
///    Input:  three points P0, P1, and P2
///    Return: PointLocation for location of P2 relative to [P0, P1]
fn line_point_position<T>(
    c0: &Coordinate<T>,
    c1: &Coordinate<T>,
    c2: &Coordinate<T>,
) -> PointLocation
where
    T: CoordinateType,
{
    let test = Triangle(c0.clone(), c1.clone(), c2.clone()).signed_area();
    if test > T::zero() {
        PointLocation::Left
    } else if test == T::zero() {
        PointLocation::On
    } else {
        PointLocation::Right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_basic_containment() {
        let poly = Polygon::from(vec![(0., 0.), (0., 1.), (1., 1.), (1., 0.), (0., 0.)]);
        let point = Point::from((0.5, 0.5));
        assert!(polygon_contains_point(&poly, &point).expect("Shouldn't have error"));
    }

    #[test]
    fn check_envelope_non_containment() {
        let poly = Polygon::from(vec![(0., 0.), (0., 1.), (1., 1.), (1., 0.), (0., 0.)]);
        let point = Point::from((1.5, 0.5));
        assert!(!polygon_contains_point(&poly, &point).expect("Shouldn't have error"));
    }

    #[test]
    fn check_in_envelope_non_containment() {
        let poly = Polygon::from(vec![
            (0., 0.),
            (0., 1.),
            (1., 1.),
            (0.5, 0.5),
            (1., 0.),
            (0., 0.),
        ]);
        let point = Point::from((1.0, 0.5));
        assert!(!polygon_contains_point(&poly, &point).unwrap());
    }

    #[test]
    fn check_interior_non_containment() {
        let poly = Polygon::new(
            LineString::from(vec![(-1., -1.), (-1., 1.), (1., 1.), (1., -1.), (-1., -1.)]),
            vec![LineString::from(vec![
                (-0.5, -0.5),
                (-0.5, 0.5),
                (0.5, 0.5),
                (0.5, -0.5),
                (-0.5, -0.5),
            ])],
        );
        let point = Point::from((0.0, 0.0));
        assert!(!polygon_contains_point(&poly, &point).unwrap());
    }

    #[test]
    fn check_interior_non_containment_ccw() {
        let poly = Polygon::new(
            LineString::from(vec![(-1., -1.), (-1., 1.), (1., 1.), (1., -1.), (-1., -1.)]),
            vec![LineString::from(vec![
                (-0.5, -0.5),
                (0.5, -0.5),
                (0.5, 0.5),
                (-0.5, 0.5),
                (-0.5, -0.5),
            ])],
        );
        let point = Point::from((0.0, 0.0));
        assert!(!polygon_contains_point(&poly, &point).unwrap());
    }
}
