use {CoordinateType, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon};

pub trait Length<T> {
    fn length(&self) -> T;
}

impl<T> Length<T> for Point<T>
where
    T: CoordinateType,
{
    fn length(&self) -> T {
        T::zero()
    }
}

impl<T> Length<T> for MultiPoint<T>
where
    T: CoordinateType,
{
    fn length(&self) -> T {
        T::zero()
    }
}

/// Calculate the sum of the lengths of its LineStrings.
impl<T> Length<T> for MultiLineString<T>
where
    T: CoordinateType,
{
    fn length(&self) -> T {
        self.line_strings.iter().map(|ls| ls.length()).sum()
    }
}

/// Calculate the length of its exterior, plus the sum of that of the interiors.
impl<T> Length<T> for Polygon<T>
where
    T: CoordinateType,
{
    fn length(&self) -> T {
        self.exterior.length() + self.interiors.iter().map(|ls| ls.length()).sum()
    }
}

/// Calculate the sum of the lengths of its polygons.
impl<T> Length<T> for MultiPolygon<T>
where
    T: CoordinateType,
{
    fn length(&self) -> T {
        self.polygons.iter().map(|p| p.length()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use LineString;

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
            ]).length()
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
