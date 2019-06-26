use crate::primitives::{Coordinate, Envelope, HasEnvelope};
use crate::rtree::utils::{find_loop_loop_relation, LoopLoopRelation};
use crate::types::{Geometry, LineString, MultiLineString, Point};

#[derive(Debug, PartialEq)]
pub struct Polygon<C: Coordinate> {
    pub exterior: LineString<C>,
    pub interiors: Vec<LineString<C>>,
    _envelope: Envelope<C>,
}

/// Turn a `Vec` of `Position`-ish objects into a `Polygon` with no interior loops.
impl<C: Coordinate, ILS: Into<LineString<C>>> From<ILS> for Polygon<C> {
    fn from(ext: ILS) -> Self {
        let exterior: LineString<C> = ext.into();
        let _envelope = exterior.envelope();
        Polygon {
            exterior,
            interiors: vec![],
            _envelope,
        }
    }
}

impl<C: Coordinate> Polygon<C> {
    pub fn new(exterior: LineString<C>, interiors: Vec<LineString<C>>) -> Polygon<C> {
        let _envelope = exterior.envelope();
        Polygon {
            exterior,
            interiors,
            _envelope,
        }
    }

    /// A Polygon is simple if it has no self-intersections in its envelopes.
    pub fn is_simple(&self) -> bool {
        !self.validate().is_err()
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.exterior.is_closed() {
            return Err("Exterior is not a loop.");
        };
        let ext_rtree = self.exterior._validate_with_rtree()?;
        let mut rtrees = Vec::new(); // store interior ring rtrees
        for interior in &self.interiors {
            if !interior.is_closed() {
                return Err("Interior linestring is not a loop.");
            };
            let int_rtree = interior._validate_with_rtree()?;
            if find_loop_loop_relation(&ext_rtree, &int_rtree) != LoopLoopRelation::Contains {
                return Err("Interior loop not contained in exterior loop.");
            }
            for other_rtree in rtrees.iter() {
                if find_loop_loop_relation(&int_rtree, &other_rtree) != LoopLoopRelation::Separate {
                    return Err("Two Interior rings intersect.");
                }
            }
            rtrees.push(int_rtree);
        }
        Ok(())
    }
}

// Polygon implementation
impl<C: Coordinate> Polygon<C> {
    pub fn centroid(&self) -> Point<C> {
        // TODO: STUB
        Point::from((C::zero(), C::zero()))
    }

    /**
     * Find an abitrary point on the surface.
     * If empty, return None.
     */
    pub fn point_on_surface(&self) -> Option<Point<C>> {
        self.exterior.start_point()
    }
}

// GEOMETRY implementation
impl<C: Coordinate> Polygon<C> {
    pub fn dimension(&self) -> u8 {
        2
    }

    pub fn geometry_type(&self) -> &'static str {
        "Polygon"
    }

    pub fn envelope(&self) -> Envelope<C> {
        self._envelope
    }

    pub fn is_empty(&self) -> bool {
        self.exterior.is_empty()
    }

    /// The boundary of a Polygon are the component LineStrings.
    pub fn boundary(&self) -> Geometry<C> {
        let mut line_strings = Vec::with_capacity(1 + self.interiors.len());
        line_strings.push(self.exterior.clone());
        line_strings.extend(self.interiors.clone());
        Geometry::from(MultiLineString::new(line_strings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_basic_polygon() {
        let p = Polygon::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, 0.0)]);
        assert_eq!(p.exterior.num_points(), 4);
        assert_eq!(p.interiors.len(), 0);
    }

    // Validity checks
    #[test]
    fn check_basic_square() {
        let basic_square = Polygon::from(vec![
            (0.0, 0.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (0.0, 0.0),
        ]);
        assert!(basic_square.is_simple());
    }

    #[test]
    fn check_non_loop() {
        let basic_square = Polygon::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)]);
        assert!(!basic_square.is_simple());
    }

    #[test]
    fn check_interior_loop() {
        let poly = Polygon::new(
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

        assert!(poly.is_simple());
    }

    #[test]
    fn check_interior_non_loop() {
        let poly = Polygon::new(
            LineString::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)]),
            vec![LineString::from(vec![
                (0.25, 0.25),
                (0.25, 0.75),
                (0.75, 0.75),
            ])],
        );

        assert!(!poly.is_simple());
    }

}
