use crate::relation::Intersection;
use crate::types::{Coordinate, Geometry, LineString, Point, PositionLocation, Polygon, Rect};

pub fn intersection_linestring_point<C: Coordinate>(
    linestring: &LineString<C>,
    point: &Point<C>,
) -> Intersection
{
    let position = point.0;
    if !linestring.envelope().contains(position) {
        return Intersection::Outside;
    }

    if !linestring.is_closed() {
        match linestring.start_point() {
            // Already checked empty case, but for syntactic completeness...
            None => return Intersection::Outside,
            Some(c) => {
                if c == position {
                    return Intersection::Boundary;
                }
            }
        }
        match linestring.end_point() {
            // Already checked empty case, but for syntactic completeness...
            None => return Intersection::Outside,
            Some(c) => {
                if c == position {
                    return Intersection::Boundary;
                }
            }
        }
    }

    if linestring
        .segments_iter()
        .filter(|&s| Rect::from(s).contains(position))
        .any(|s| s.coord_position(position) == PositionLocation::On)
    {
        Intersection::Contains
    } else {
        Intersection::Outside
    }
}

pub fn intersection_polygon_point<C>(
    polygon: &Polygon<C>,
    point: &Point<C>,
) -> Result<Intersection, &'static str>
where
    T: Coordinate,
{
    // If it's not in the envelope, it's not in the polygon.
    if !polygon.envelope.contains(point.0) {
        return Ok(Intersection::Outside);
    }

    match _intersection_simple_polygon_point(&polygon.exterior, point)? {
        // If it's outside the exterior ring, it's not in the polygon.
        Intersection::Outside => return Ok(Intersection::Outside),
        // If it's on the exterior ring, it's on the boundarty.
        Intersection::Boundary => return Ok(Intersection::Boundary),
        // If it's inside the exterior ring, it may be in the polygon.
        Intersection::Contains => (),
    }
    for int_ring in &polygon.interiors {
        match _intersection_simple_polygon_point(&int_ring, point)? {
            // If it's inside an interior ring, it's not in the polygon.
            Intersection::Contains => return Ok(Intersection::Outside),
            // If it's on an interior ring, it's on the boundarty.
            Intersection::Boundary => return Ok(Intersection::Boundary),
            // If it's outside an interior ring, it may be in the polygon.
            Intersection::Outside => (),
        }
    }
    // If it's inside the exerior ring, but not inside an interior ring, it's contained.
    Ok(Intersection::Contains)
}

/// Check the intersection of a simple polygon (defined by a loop) and a point.
fn _intersection_simple_polygon_point<C>(
    ls: &LineString<C>,
    point: &Point<C>,
) -> Result<Intersection, &'static str>
where
    T: Coordinate,
{
    if !ls.is_closed() {
        return Err("Simple polygons must be defined by a closed LineString.");
    }

    let mut wn: i32 = 0; // the winding number counter
    let position = point.0;
    // loop through all edges of the polygon
    let right_segments = ls.segments_iter().filter(|&s| {
        // We only care about segments we are on, or intersect a ray in the positive x dir.
        let rect = Rect::from(s);
        position.y <= rect.max.y && position.y >= rect.min.y && position.x <= rect.max.x
    });
    for seg in right_segments {
        if seg.contains(position) {
            return Ok(Intersection::Boundary);
        }

        if seg.start.y <= position.y {
            if seg.end.y > position.y  // an upward crossing
                 && seg.position_location(position) == PositionLocation::Left
            {
                wn += 1; // have a valid up intersect
            }
        } else {
            // seg.start.y > P.y (no test needed)
            if seg.end.y <= position.y  // a downward crossing
                 && seg.position_location(position) == PositionLocation::Right
            {
                wn -= 1; // have  a valid down intersect
            }
        }
    }

    if wn == 0 {
        Ok(Intersection::Outside)
    } else {
        Ok(Intersection::Contains)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_basic_containment() {
        let poly = Polygon::from(vec![(0., 0.), (0., 1.), (1., 1.), (1., 0.), (0., 0.)]);
        let point = Point::from((0.5, 0.5));
        // let position = intersection_polygon_point(&poly, &point)?;
        let position = intersection_polygon_point(&poly, &point).expect("Shouldn't have error");
        assert_eq!(position, Intersection::Contains);
    }

    #[test]
    fn check_envelope_non_containment() {
        let poly = Polygon::from(vec![(0., 0.), (0., 1.), (1., 1.), (1., 0.), (0., 0.)]);
        let point = Point::from((1.5, 0.5));
        let position = intersection_polygon_point(&poly, &point).expect("Shouldn't have error");
        assert_eq!(position, Intersection::Outside);
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
        let position = intersection_polygon_point(&poly, &point).expect("Shouldn't have error");
        assert_eq!(position, Intersection::Outside);
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
        let position = intersection_polygon_point(&poly, &point).expect("Shouldn't have error");
        assert_eq!(position, Intersection::Outside);
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
        let position = intersection_polygon_point(&poly, &point).expect("Shouldn't have error");
        assert_eq!(position, Intersection::Outside);
    }

    // Intersection of LineString and Point
    #[test]
    fn check_linestring_point_far_outside() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        let p = Point::from((-1.0, -1.0));
        assert_eq!(
            intersection_linestring_point(&ls, &p),
            Intersection::Outside
        );
    }

    #[test]
    fn check_linestring_point_outside() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        let p = Point::from((0.0, 1.0));
        assert_eq!(
            intersection_linestring_point(&ls, &p),
            Intersection::Outside
        );
    }

    #[test]
    fn check_linestring_point_first_endpoint() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        let p = Point::from((0.0, 0.0));
        assert_eq!(
            intersection_linestring_point(&ls, &p),
            Intersection::Boundary
        );
    }

    #[test]
    fn check_linestring_point_last_endpoint() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        let p = Point::from((1.0, 1.0));
        assert_eq!(
            intersection_linestring_point(&ls, &p),
            Intersection::Boundary
        );
    }

    #[test]
    fn check_loop_point_first_endpoint() {
        let ls = LineString::from(vec![(0., 0.), (0., 1.), (1., 1.), (1., 0.), (0., 0.)]);
        let p = Point::from((0.0, 0.0));
        assert_eq!(
            intersection_linestring_point(&ls, &p),
            Intersection::Contains
        );
    }

    #[test]
    fn check_linestring_point_interior_vertex() {
        let ls = LineString::from(vec![(0., 0.), (0., 1.), (1., 1.)]);
        let p = Point::from((0.0, 1.0));
        assert_eq!(
            intersection_linestring_point(&ls, &p),
            Intersection::Contains
        );
    }

    #[test]
    fn check_linestring_point_interior_nonvertex() {
        let ls = LineString::from(vec![(0., 0.), (0., 1.), (1., 1.)]);
        let p = Point::from((0.0, 0.5));
        assert_eq!(
            intersection_linestring_point(&ls, &p),
            Intersection::Contains
        );
    }

    // This tests our condition which checks for colinearity of the infinite line.
    #[test]
    fn check_linestring_point_inside_crook() {
        let ls = LineString::from(vec![(0., 0.), (0., 1.), (1., 1.), (1., 2.), (0., 2.)]);
        let p = Point::from((0.0, 1.5));
        assert_eq!(
            intersection_linestring_point(&ls, &p),
            Intersection::Outside
        );
    }
}
