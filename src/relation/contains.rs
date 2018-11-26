use crate::types::CoordinateType;
use crate::types::Rect;
use crate::types::LineString;
use crate::types::Point;
use crate::types::Polygon;
use crate::types::PointLocation;
use Intersection;

pub fn intersection_linestring_point<T>(
    linestring: &LineString<T>,
    point: &Point<T>,
) -> Intersection
where T: CoordinateType,
{
    let coord = point.0;
    if !linestring.envelope.contains(coord) {
        return Intersection::Outside;
    }

    if !linestring.is_closed() {
        match linestring.first() {
            // Already checked empty case, but for syntactic completeness...
            None => return Intersection::Outside,
            Some(c) => if c == coord {
                return Intersection::Boundary;
            },
        }
        match linestring.last() {
            // Already checked empty case, but for syntactic completeness...
            None => return Intersection::Outside,
            Some(c) => if c == coord {
                return Intersection::Boundary;
            },
        }
    }

    if linestring.segments_iter()
            .filter(|&s| Rect::from(s).contains(coord))
            .any(|s| s.coord_position(coord) == PointLocation::On)
    {
        Intersection::Contains
    } else {
        Intersection::Outside
    }
}

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
                 && seg.coord_position(point.0.clone()) == PointLocation::Left
            {
                wn += 1; // have  a valid up intersect
            }
        } else {
            // seg.start.y > P.y (no test needed)
            if seg.end.y <= point.0.y  // a downward crossing
                 && seg.coord_position(point.0.clone()) == PointLocation::Right
            {
                wn -= 1; // have  a valid down intersect
            }
        }
    }
    Ok(wn)
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

    // Intersectino of LineString and Point
    #[test]
    fn check_linestring_point_far_outside() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        let p = Point::from((-1.0, -1.0));
        assert_eq!(intersection_linestring_point(&ls, &p), Intersection::Outside);
    }

    #[test]
    fn check_linestring_point_outside() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        let p = Point::from((0.0, 1.0));
        assert_eq!(intersection_linestring_point(&ls, &p), Intersection::Outside);
    }

    #[test]
    fn check_linestring_point_first_endpoint() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        let p = Point::from((0.0, 0.0));
        assert_eq!(intersection_linestring_point(&ls, &p), Intersection::Boundary);
    }

    #[test]
    fn check_linestring_point_last_endpoint() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        let p = Point::from((1.0, 1.0));
        assert_eq!(intersection_linestring_point(&ls, &p), Intersection::Boundary);
    }

    #[test]
    fn check_loop_point_first_endpoint() {
        let ls = LineString::from(vec![(0., 0.), (0., 1.), (1., 1.), (1., 0.), (0., 0.)]);
        let p = Point::from((0.0, 0.0));
        assert_eq!(intersection_linestring_point(&ls, &p), Intersection::Contains);
    }

    #[test]
    fn check_linestring_point_interior_vertex() {
        let ls = LineString::from(vec![(0., 0.), (0., 1.), (1., 1.)]);
        let p = Point::from((0.0, 1.0));
        assert_eq!(intersection_linestring_point(&ls, &p), Intersection::Contains);
    }

    #[test]
    fn check_linestring_point_interior_nonvertex() {
        let ls = LineString::from(vec![(0., 0.), (0., 1.), (1., 1.)]);
        let p = Point::from((0.0, 0.5));
        assert_eq!(intersection_linestring_point(&ls, &p), Intersection::Contains);
    }

    // This tests our condition which checks for colinearity of the infinite line.
    #[test]
    fn check_linestring_point_inside_crook() {
        let ls = LineString::from(vec![(0., 0.), (0., 1.), (1., 1.), (1., 2.), (0., 2.)]);
        let p = Point::from((0.0, 1.5));
        assert_eq!(intersection_linestring_point(&ls, &p), Intersection::Outside);
    }
}
