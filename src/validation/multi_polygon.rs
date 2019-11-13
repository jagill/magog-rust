use crate::algorithms::loop_relation::{find_loop_loop_relation, LoopLoopRelation};
use crate::flatbush::{Flatbush, FLATBUSH_DEFAULT_DEGREE};
use crate::types::MultiPolygon;
use crate::Coordinate;

impl<C: Coordinate> MultiPolygon<C> {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.polygons.is_empty() {
            // MultiPolygons with no Polygons are a valid empty geometry.
            return Ok(());
        }

        let intersection_err = Err("Two polygons intersect.");

        for polygon in self.polygons.iter() {
            polygon.validate()?;
        }
        let rtree_of_polygons = Flatbush::new(&self.polygons, FLATBUSH_DEFAULT_DEGREE);

        for (poly1_id, poly2_id) in rtree_of_polygons.find_self_intersection_candidates() {
            let polygon1 = &self.polygons[poly1_id];
            let polygon2 = &self.polygons[poly2_id];
            let inner_poly;
            let outer_poly;
            match find_loop_loop_relation(&polygon1.exterior, &polygon2.exterior) {
                LoopLoopRelation::Separate => continue,
                LoopLoopRelation::Crosses => return intersection_err,
                LoopLoopRelation::Contains => {
                    inner_poly = polygon2;
                    outer_poly = polygon1;
                }
                LoopLoopRelation::Within => {
                    inner_poly = polygon1;
                    outer_poly = polygon2;
                }
            }
            // If inner_poly.exterior is contained within outer_poly.exterior,
            // inner_poly must be inside of exactly one loop in outer_poly.interiors.
            // Validity ensures that there is at most one like this.
            // Crosses or Contains means this is invalid.  Separate means that
            // inner_poly might be in another interior loop.
            let mut inside_interior = false;
            for int_loop in &outer_poly.interiors {
                match find_loop_loop_relation(&inner_poly.exterior, &int_loop) {
                    LoopLoopRelation::Separate => continue,
                    LoopLoopRelation::Within => inside_interior = true,
                    LoopLoopRelation::Crosses | LoopLoopRelation::Contains => {
                        return intersection_err
                    }
                }
                if inside_interior {
                    break;
                }
            }
            if !inside_interior {
                // We didn't find any interior loop that inner_poly is contained in.
                return intersection_err;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{LineString, Polygon};

    #[test]
    fn test_valid_microsoft_examples() {
        assert!(MultiPolygon::<f32>::new(Vec::new()).validate().is_ok());
        assert!(MultiPolygon::from(vec![
            Polygon::from(vec![(1., 1.), (1., -1.), (-1., -1.), (-1., 1.), (1., 1.)]),
            Polygon::from(vec![(1., 1.), (3., 1.), (3., 3.), (1., 3.), (1., 1.)]),
        ])
        .validate()
        .is_ok());
        assert!(MultiPolygon::from(vec![
            Polygon::new(
                LineString::from(vec![(0., 0.), (5., 0.), (5., 5.), (0., 5.), (0., 0.)]),
                vec![LineString::from(vec![
                    (1., 1.),
                    (4., 1.),
                    (4., 4.),
                    (1., 4.),
                    (1., 1.)
                ])]
            ),
            Polygon::from(vec![(2., 2.), (3., 2.), (3., 3.), (2., 3.), (2., 2.)]),
        ])
        .validate()
        .is_ok());
    }

    #[test]
    fn test_invalid_microsoft_examples() {
        assert!(MultiPolygon::from(vec![
            Polygon::from(vec![(1., 1.), (1., -1.), (-1., -1.), (-1., 1.), (1., 1.)]),
            Polygon::from(vec![(1., 1.), (3., 1.), (3., 3.)]),
        ])
        .validate()
        .is_err());
        assert!(MultiPolygon::from(vec![
            Polygon::from(vec![(2., 2.), (2., -2.), (-2., -2.), (-2., 2.), (2., 2.)]),
            Polygon::from(vec![(1., 1.), (3., 1.), (3., 3.), (1., 3.), (1., 1.)]),
        ])
        .validate()
        .is_err());
    }
}
