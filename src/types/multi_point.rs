use {Coordinate, CoordinateType, Envelope, Point};

#[derive(Debug, PartialEq)]
pub struct MultiPoint<T>
where
    T: CoordinateType,
{
    pub points: Vec<Point<T>>,
    pub envelope: Envelope<T>,
}

/// Turn a `Vec` of `Coordinate`-ish objects into a `LineString`.
impl<T: CoordinateType, IC: Into<Coordinate<T>>> From<Vec<IC>> for MultiPoint<T> {
    fn from(v: Vec<IC>) -> Self {
        MultiPoint::new(v.into_iter().map(|c| Point(c.into())).collect())
    }
}

impl<T: CoordinateType> MultiPoint<T> {
    pub fn new(points: Vec<Point<T>>) -> Self {
        let coords: Vec<Coordinate<T>> = points.iter().map(|p| p.0).collect();
        let envelope: Envelope<T> = Envelope::from(&coords);
        MultiPoint { points, envelope }
    }
}
