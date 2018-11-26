use {CoordinateType, Envelope, Polygon};

#[derive(Debug, PartialEq)]
pub struct MultiPolygon<T>
where
    T: CoordinateType,
{
    pub polygons: Vec<Polygon<T>>,
    pub envelope: Envelope<T>,
}

impl<T: CoordinateType> MultiPolygon<T> {
    pub fn new(polygons: Vec<Polygon<T>>) -> Self {
        let envs: Vec<Envelope<T>> = polygons.iter().map(|p| p.envelope).collect();
        let envelope = Envelope::from(&envs);
        MultiPolygon { polygons, envelope }
    }
}
