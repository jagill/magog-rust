use crate::types::{Coordinate, CoordinateType, Envelope, Geometry, MultiPoint, Point, Segment};

#[derive(Debug, PartialEq)]
pub struct LineString<T>
where
    T: CoordinateType,
{
    pub coords: Vec<Coordinate<T>>,
    _envelope: Envelope<T>,
}

/// Turn a `Vec` of `Coordinate`-ish objects into a `LineString`.
impl<T: CoordinateType, IC: Into<Coordinate<T>>> From<Vec<IC>> for LineString<T> {
    fn from(v: Vec<IC>) -> Self {
        LineString::new(v.into_iter().map(|c| c.into()).collect())
    }
}

impl<T: CoordinateType> LineString<T> {
    pub fn new(coords: Vec<Coordinate<T>>) -> LineString<T> {
        let _envelope = Envelope::from(&coords);
        LineString { coords, _envelope }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        let mut last_coord: Option<Coordinate<T>> = None;
        for &coord in &self.coords {
            coord.validate()?;
            // According to the spec this function must fail if any two consecutive points are the same.
            match last_coord {
                None => last_coord = Some(coord),
                Some(c) => {
                    if c == coord {
                        return Err("LineString coordinates have repeated points.");
                    }
                    last_coord = Some(coord);
                }
            }
        }
        self._envelope.validate()?;
        Ok(())
    }

    pub fn segments_iter<'a>(&'a self) -> impl Iterator<Item = Segment<T>> + 'a {
        self.coords
            .iter()
            .zip(self.coords.iter().skip(1))
            .map(|(start, end)| Segment {
                start: start.clone(),
                end: end.clone(),
            })
    }
}

// LineString Implementation
impl<T: CoordinateType> LineString<T> {
    pub fn num_points(&self) -> usize {
        self.coords.len()
    }

    /// Get the point at coordinate `n` of the LineString.
    /// If `n > self.num_points`, return None.
    pub fn get_point(&self, n: usize) -> Option<Point<T>> {
        match self.coords.get(n) {
            None => None,
            Some(c) => Some(Point::new(*c)),
        }
    }

    pub fn is_closed(&self) -> bool {
        if self.coords.len() < 4 {
            return false;
        }
        return self.coords[0] == self.coords[self.coords.len() - 1];
    }

    pub fn is_ring(&self) -> bool {
        self.is_closed() && self.is_simple()
    }

    pub fn length(&self) -> T {
        self.segments_iter().map(|s| s.length()).sum()
    }

    /// Return the first coordinate of the linestring
    pub fn start_point(&self) -> Option<Coordinate<T>> {
        if self.coords.len() == 0 {
            return None;
        }
        Some(self.coords[0])
    }

    /// Return the last coordinate of the linestring
    pub fn end_point(&self) -> Option<Coordinate<T>> {
        if self.coords.len() == 0 {
            return None;
        }
        Some(self.coords[self.coords.len() - 1])
    }
}

// GEOMETRY implementation
impl<T: CoordinateType> LineString<T> {
    pub fn dimension(&self) -> u8 {
        1
    }

    pub fn geometry_type(&self) -> &'static str {
        "LineString"
    }

    pub fn envelope(&self) -> Envelope<T> {
        self._envelope
    }

    pub fn is_empty(&self) -> bool {
        self.coords.is_empty()
    }

    /// A LineString is simple if it has no self-intersections.
    pub fn is_simple(&self) -> bool {
        // TODO STUB
        true
    }

    pub fn boundary(&self) -> Geometry<T> {
        if self.is_closed() {
            Geometry::Empty
        } else {
            match (self.start_point(), self.end_point()) {
                (None, _) | (_, None) => Geometry::Empty,
                (Some(s), Some(e)) => Geometry::from(MultiPoint::from(vec![s, e])),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Rect;

    #[test]
    fn check_basic_linestring() {
        let c0: Coordinate<f64> = Coordinate { x: 0.0, y: 0.1 };
        let c1: Coordinate<f64> = Coordinate { x: 1.0, y: 1.1 };
        let ls = LineString::new(vec![c0, c1]);
        let results: Vec<Coordinate<f64>> = ls.coords.into_iter().collect();
        assert_eq!(results, vec![c0, c1])
    }

    #[test]
    fn check_linestring_segments_iter() {
        let c0: Coordinate<f64> = Coordinate { x: 0.0, y: 0.1 };
        let c1: Coordinate<f64> = Coordinate { x: 1.0, y: 1.1 };
        let c2: Coordinate<f64> = Coordinate { x: 2.0, y: 2.1 };
        let ls = LineString::new(vec![c0, c1, c2]);
        let results: Vec<Segment<f64>> = ls.segments_iter().collect();
        assert_eq!(
            results,
            vec![
                Segment { start: c0, end: c1 },
                Segment { start: c1, end: c2 },
            ]
        )
    }

    #[test]
    fn check_is_empty() {
        let ls: LineString<f64> = LineString::new(vec![]);
        assert!(ls.is_empty())
    }

    #[test]
    fn check_empty_is_not_loop() {
        let ls: LineString<f64> = LineString::new(vec![]);
        assert!(!ls.is_closed());
    }

    #[test]
    fn check_single_is_not_loop() {
        let ls = LineString::from(vec![(0.0, 0.0)]);
        assert!(!ls.is_closed());
    }

    #[test]
    fn check_double_is_not_loop() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        assert!(!ls.is_closed());
    }

    #[test]
    fn check_triple_is_not_loop() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)]);
        assert!(!ls.is_closed());
    }

    #[test]
    fn check_is_not_loop() {
        let ls = LineString::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]);
        assert!(!ls.is_closed());
    }

    #[test]
    fn check_is_loop() {
        let ls = LineString::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, 0.0)]);
        assert!(ls.is_closed());
    }

    #[test]
    fn check_envelope() {
        let ls = LineString::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, 0.0)]);
        match ls.envelope().rect {
            None => assert!(false, "Envelope should not be empty."),
            Some(r) => assert_eq!(r, Rect::from(((0.0, 0.0), (1.0, 1.0)))),
        }
    }

    #[test]
    fn check_num_points() {
        let ls = LineString::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, 0.0)]);
        assert_eq!(4, ls.num_points());
    }

    #[test]
    fn check_get_point() {
        let ls = LineString::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, 0.0)]);
        assert_eq!(Some(Point::from((0.0, 1.0))), ls.get_point(1));
    }
}
