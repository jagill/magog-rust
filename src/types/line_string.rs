use crate::flatbush::{Flatbush, FLATBUSH_DEFAULT_DEGREE};
use crate::primitives::{Coordinate, Envelope, HasEnvelope, Position, Segment};
use crate::types::{Geometry, MultiPoint, Point};

#[derive(Debug, PartialEq, Clone)]
pub struct LineString<C: Coordinate> {
    pub positions: Vec<Position<C>>,
    _envelope: Envelope<C>,
}

/// Turn a `Vec` of `Position`-ish objects into a `LineString`.
impl<C: Coordinate, IP: Into<Position<C>>> From<Vec<IP>> for LineString<C> {
    fn from(v: Vec<IP>) -> Self {
        LineString::new(v.into_iter().map(|p| p.into()).collect())
    }
}

impl<C: Coordinate> LineString<C> {
    pub fn new(positions: Vec<Position<C>>) -> LineString<C> {
        let _envelope = Envelope::from(&positions);
        LineString {
            positions,
            _envelope,
        }
    }

    pub fn segments_iter<'a>(&'a self) -> impl Iterator<Item = Segment<C>> + 'a {
        self.positions
            .iter()
            .zip(self.positions.iter().skip(1))
            .map(|(start, end)| Segment {
                start: start.clone(),
                end: end.clone(),
            })
    }

    /**
     * Return the `n`th segment.
     *
     * This does not check that `n` is < positions.len() - 1;
     * it will just panic if an invalid n is given.
     */
    pub fn get_segment(&self, n: usize) -> Segment<C> {
        Segment::new(self.positions[n], self.positions[n + 1])
    }

    pub fn build_rtree(&self) -> Flatbush<C> {
        let segments = self.segments_iter().collect();
        Flatbush::new_unsorted(&segments, FLATBUSH_DEFAULT_DEGREE)
    }
}

// LineString Implementation
impl<C: Coordinate> LineString<C> {
    pub fn num_points(&self) -> usize {
        self.positions.len()
    }

    /// Get the point at coordinate `n` of the LineString.
    /// If `n > self.num_points`, return None.
    pub fn get_point(&self, n: usize) -> Option<Point<C>> {
        let position = self.positions.get(n)?;
        Some(Point::new(*position))
    }

    pub fn is_closed(&self) -> bool {
        if self.positions.len() < 4 {
            return false;
        }
        return self.positions[0] == self.positions[self.positions.len() - 1];
    }

    pub fn is_ring(&self) -> bool {
        self.is_closed() && self.is_simple()
    }

    pub fn length(&self) -> C {
        self.segments_iter().map(|s| s.length()).sum()
    }

    /// Return the first coordinate of the linestring
    pub fn start_point(&self) -> Option<Point<C>> {
        let position = self.positions.get(0)?;
        Some(Point(*position))
    }

    /// Return the last coordinate of the linestring
    pub fn end_point(&self) -> Option<Point<C>> {
        if self.positions.len() == 0 {
            return None;
        }
        Some(Point(self.positions[self.positions.len() - 1]))
    }
}

// GEOMETRY implementation
impl<C: Coordinate> LineString<C> {
    pub fn dimension(&self) -> u8 {
        1
    }

    pub fn geometry_type(&self) -> &'static str {
        "LineString"
    }

    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    pub fn boundary(&self) -> Geometry<C> {
        if self.is_closed() {
            Geometry::empty()
        } else {
            match (self.start_point(), self.end_point()) {
                (None, _) | (_, None) => Geometry::empty(),
                (Some(s), Some(e)) => Geometry::from(MultiPoint::new(vec![s, e])),
            }
        }
    }

    /// A LineString is simple if it has no self-intersections.
    pub fn is_simple(&self) -> bool {
        self.validate().is_ok()
    }
}

impl<C: Coordinate> HasEnvelope<C> for LineString<C> {
    fn envelope(&self) -> Envelope<C> {
        self._envelope
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Rect;

    use core::f32;

    #[test]
    fn check_basic_linestring() {
        let p0: Position<f64> = Position { x: 0.0, y: 0.1 };
        let p1: Position<f64> = Position { x: 1.0, y: 1.1 };
        let ls = LineString::new(vec![p0, p1]);
        let results: Vec<Position<f64>> = ls.positions.into_iter().collect();
        assert_eq!(results, vec![p0, p1])
    }

    #[test]
    fn check_linestring_segments_iter() {
        let p0: Position<f64> = Position { x: 0.0, y: 0.1 };
        let p1: Position<f64> = Position { x: 1.0, y: 1.1 };
        let p2: Position<f64> = Position { x: 2.0, y: 2.1 };
        let ls = LineString::new(vec![p0, p1, p2]);
        let results: Vec<Segment<f64>> = ls.segments_iter().collect();
        assert_eq!(
            results,
            vec![
                Segment { start: p0, end: p1 },
                Segment { start: p1, end: p2 },
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

    // is_simple checks
    #[test]
    fn check_empty_simple() {
        let empty_vec: Vec<Position<f32>> = Vec::new();
        let ls = LineString::new(empty_vec);
        assert!(ls.is_simple());
    }

    #[test]
    fn check_single_not_simple() {
        let ls = LineString::from(vec![(0.0, 0.0)]);
        assert!(!ls.is_simple());
    }

    #[test]
    fn check_open_simple() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        assert!(ls.is_simple());
    }

    #[test]
    fn check_repeated_point_not_simple() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0), (1.0, 1.0), (1.0, 0.0)]);
        assert!(!ls.is_simple());
    }

    #[test]
    fn check_loop_simple() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0)]);
        assert!(ls.is_simple());
    }

    #[test]
    fn check_self_isxn_not_simple() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 1.0)]);
        assert!(!ls.is_simple());
    }

    #[test]
    fn check_backtrack_not_simple() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0), (0.5, 0.5)]);
        assert!(!ls.is_simple());
    }

    #[test]
    fn check_backtrack2_not_simple() {
        let ls = LineString::from(vec![(-1.0, 49.0), (-1.0, 50.0), (-1.0, 49.0)]);
        assert!(!ls.is_simple());
    }

    #[test]
    fn check_tail_not_simple() {
        let ls = LineString::from(vec![
            (0.0, 0.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (0.0, 0.0),
            (0.0, 1.0),
        ]);
        assert!(!ls.is_simple());
    }

    #[test]
    fn check_loop_overlap_not_simple() {
        let ls = LineString::from(vec![
            (0.0, 0.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (-1.0, -1.0),
            (0.2, 0.2),
        ]);
        assert!(!ls.is_simple());
    }

    #[test]
    fn check_internal_tangent_loop_not_simple() {
        let ls = LineString::from(vec![
            (0.0, 0.0),
            (1.0, 0.0),
            (0.5, 0.5),
            (1.0, 1.0),
            (1.0, 0.0),
            (0.5, 0.5),
            (0.0, 0.0),
        ]);
        assert!(!ls.is_simple());
    }

    #[test]
    fn check_tadpole_not_simple() {
        let ls = LineString::from(vec![
            (0.0, 0.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (1.0, 1.0),
            (0.0, 0.0),
        ]);
        assert!(!ls.is_simple());
    }

    #[test]
    fn check_nan_is_not_simple() {
        let ls = LineString::from(vec![(0., 0.), (0.1, f32::NAN)]);
        assert!(!ls.is_simple());
    }

    #[test]
    fn check_angle_simple() {
        let ls = LineString::from(vec![(1., 0.), (1., 1.), (0., 0.)]);
        assert!(ls.is_simple());
    }

}
