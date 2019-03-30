use crate::types::{
    Coordinate, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon,
};

fn get_signed_loop_area<C: Coordinate>(ls: &LineString<C>) -> C {
    if ls.num_points() < 4 {
        return C::zero();
    }
    let twice_area: C = ls.segments_iter().map(|s| s.determinant()).sum();
    twice_area / (C::one() + C::one())
}

fn get_loop_area<C: Coordinate>(ls: &LineString<C>) -> C {
    let signed_area = get_signed_loop_area(ls);
    if signed_area < C::zero() {
        -signed_area
    } else {
        signed_area
    }
}

pub trait Area<C: Coordinate> {
    fn area(&self) -> C;
}

impl<C: Coordinate> Area<C> for Point<C>
{
    fn area(&self) -> C {
        C::zero()
    }
}

impl<C: Coordinate> Area<C> for MultiPoint<C>
{
    fn area(&self) -> C {
        C::zero()
    }
}

impl<C: Coordinate> Area<C> for LineString<C>
{
    fn area(&self) -> C {
        C::zero()
    }
}

impl<C: Coordinate> Area<C> for MultiLineString<C>
{
    fn area(&self) -> C {
        C::zero()
    }
}

/// Calculate the area of its exterior, plus the sum of that of the interiors.
impl<C: Coordinate> Area<C> for Polygon<C>
{
    fn area(&self) -> C {
        get_loop_area(&self.exterior) - self.interiors.iter().map(|ls| get_loop_area(ls)).sum()
    }
}

/// Calculate the sum of the areas of its polygons.
impl<C: Coordinate> Area<C> for MultiPolygon<C>
{
    fn area(&self) -> C {
        self.polygons.iter().map(|p| p.area()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_point() {
        assert_eq!(0., Point::from((1.0, 2.0)).area());
    }

    #[test]
    fn check_multi_point() {
        assert_eq!(
            0.,
            MultiPoint::from(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]).area()
        );
    }

    #[test]
    fn check_line_string() {
        assert_eq!(
            0.0,
            LineString::from(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]).area()
        );
    }

    #[test]
    fn check_multi_line_string() {
        assert_eq!(
            0.0,
            MultiLineString::new(vec![
                LineString::from(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]),
                LineString::from(vec![(1.0, 0.0), (1.0, -1.0)]),
            ])
            .area()
        );
    }

    #[test]
    fn check_polygon() {
        let p = Polygon::from(vec![
            (0.0, 0.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (0.0, 0.0),
        ]);
        assert_eq!(1.0, p.area());
    }

    #[test]
    fn check_polygon_with_interior() {
        let p = Polygon::new(
            LineString::from(vec![
                (0.0, 0.0),
                (0.0, 1.0),
                (1.0, 1.0),
                (1.0, 0.0),
                (0.0, 0.0),
            ]),
            vec![LineString::from(vec![
                (0.25, 0.25),
                (0.25, 0.75),
                (0.75, 0.75),
                (0.75, 0.25),
                (0.25, 0.25),
            ])],
        );
        assert_eq!(0.75, p.area());
    }

    #[test]
    fn check_multi_polygon() {
        let p0 = Polygon::from(vec![
            (10.0, 10.0),
            (10.0, 11.0),
            (11.0, 11.0),
            (11.0, 10.0),
            (10.0, 10.0),
        ]);
        let p1 = Polygon::new(
            LineString::from(vec![
                (0.0, 0.0),
                (0.0, 1.0),
                (1.0, 1.0),
                (1.0, 0.0),
                (0.0, 0.0),
            ]),
            vec![LineString::from(vec![
                (0.25, 0.25),
                (0.25, 0.75),
                (0.75, 0.75),
                (0.75, 0.25),
                (0.25, 0.25),
            ])],
        );
        assert_eq!(1.75, MultiPolygon::new(vec![p0, p1]).area());
    }
}
