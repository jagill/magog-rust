use crate::types::{Coordinate, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon};

pub trait Length<C: Coordinate> {
    fn length(&self) -> C;
}

impl<C: Coordinate> Length<C> for Point<C>
{
    fn length(&self) -> C {
        C::zero()
    }
}

impl<C: Coordinate> Length<C> for MultiPoint<C>
{
    fn length(&self) -> C {
        C::zero()
    }
}

/// Calculate the sum of the lengths of its LineStrings.
impl<C: Coordinate> Length<C> for MultiLineString<C>
{
    fn length(&self) -> C {
        self.line_strings.iter().map(|ls| ls.length()).sum()
    }
}

/// Calculate the length of its exterior, plus the sum of that of the interiors.
impl<C: Coordinate> Length<C> for Polygon<C>
{
    fn length(&self) -> C {
        self.exterior.length() + self.interiors.iter().map(|ls| ls.length()).sum()
    }
}

/// Calculate the sum of the lengths of its polygons.
impl<C: Coordinate> Length<C> for MultiPolygon<C>
{
    fn length(&self) -> C {
        self.polygons.iter().map(|p| p.length()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LineString;

    #[test]
    fn check_point() {
        assert_eq!(0., Point::from((1.0, 2.0)).length());
    }

    #[test]
    fn check_multi_point() {
        assert_eq!(
            0.,
            MultiPoint::from(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]).length()
        );
    }

    #[test]
    fn check_line_string() {
        assert_eq!(
            2.0,
            LineString::from(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]).length()
        );
    }

    #[test]
    fn check_multi_line_string() {
        assert_eq!(
            3.0,
            MultiLineString::new(vec![
                LineString::from(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]),
                LineString::from(vec![(1.0, 0.0), (1.0, -1.0)]),
            ])
            .length()
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
        assert_eq!(4.0, p.length());
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
        assert_eq!(6.0, p.length());
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
        assert_eq!(10.0, MultiPolygon::new(vec![p0, p1]).length());
    }
}
