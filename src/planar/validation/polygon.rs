use crate::flatbush::{Flatbush, FLATBUSH_DEFAULT_DEGREE};
use crate::planar::algorithms::loop_relation::{find_loop_loop_relation, LoopLoopRelation};
use crate::planar::types::Polygon;
use crate::Coordinate;

impl<C: Coordinate> Polygon<C> {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.exterior.is_empty() && self.interiors.is_empty() {
            // Empty polygons are a valid empty geometry.
            return Ok(());
        }
        if !self.exterior.is_closed() {
            return Err("Exterior is not a loop.");
        };
        self.exterior.validate()?;
        for interior in &self.interiors {
            if !interior.is_closed() {
                return Err("Interior linestring is not a loop.");
            };
            interior.validate()?;
            if find_loop_loop_relation(&self.exterior, &interior) != LoopLoopRelation::Contains {
                return Err("Interior loop not contained in exterior loop.");
            }
        }

        let rtree_of_interiors = Flatbush::new(&self.interiors, FLATBUSH_DEFAULT_DEGREE);
        for (ls1_id, ls2_id) in rtree_of_interiors.find_self_intersection_candidates() {
            let linestring_1 = &self.interiors[ls1_id];
            let linestring_2 = &self.interiors[ls2_id];
            if find_loop_loop_relation(linestring_1, linestring_2) != LoopLoopRelation::Separate {
                return Err("Two Interior rings intersect.");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planar::types::LineString;

    #[test]
    fn test_valid_microsoft_examples() {
        assert!(Polygon::<f32>::from(LineString::new(Vec::new()))
            .validate()
            .is_ok());
        assert!(Polygon::from(LineString::from(vec![
            (1., 1.),
            (3., 3.),
            (3., 1.),
            (1., 1.)
        ]))
        .validate()
        .is_ok());

        assert!(Polygon::new(
            LineString::from(vec![(-5., -5.), (-5., 5.), (5., 5.), (5., -5.), (-5., -5.)]),
            vec![LineString::from(vec![
                (0., 0.),
                (3., 0.),
                (3., 3.),
                (0., 3.),
                (0., 0.)
            ])],
        )
        .validate()
        .is_ok());

        assert!(Polygon::new(
            LineString::from(vec![
                (-20., -20.),
                (-20., 20.),
                (20., 20.),
                (20., -20.),
                (-20., -20.)
            ]),
            vec![],
        )
        .validate()
        .is_ok());

        assert!(Polygon::new(
            LineString::from(vec![
                (-20., -20.),
                (-20., 20.),
                (20., 20.),
                (20., -20.),
                (-20., -20.)
            ]),
            vec![LineString::from(vec![
                (10., 0.),
                (0., 10.),
                (0., -10.),
                (10., 0.),
            ])],
        )
        .validate()
        .is_ok());

        assert!(Polygon::new(
            LineString::from(vec![
                (-20., -20.),
                (-20., 20.),
                (20., 20.),
                (20., -20.),
                (-20., -20.)
            ]),
            vec![
                LineString::from(vec![(10., 0.), (0., 10.), (0., -10.), (10., 0.),]),
                LineString::from(vec![(-10., 0.), (0., 10.), (-5., -10.), (-10., 0.),]),
            ],
        )
        .validate()
        .is_ok());
    }

    #[test]
    fn test_invalid_microsoft_examples() {
        assert!(Polygon::new(
            LineString::from(vec![(-5., -5.), (-5., 5.), (5., 5.), (5., -5.), (-5., -5.)]),
            vec![LineString::from(vec![
                (3., 0.),
                (6., 0.),
                (6., 3.),
                (3., 3.),
                (3., 0.)
            ])],
        )
        .validate()
        .is_err());

        assert!(Polygon::from(LineString::from(vec![
            (1., 1.),
            (1., 1.),
            (1., 1.),
            (1., 1.)
        ]))
        .validate()
        .is_err());

        assert!(
            Polygon::from(LineString::from(vec![(1., 1.), (3., 3.), (1., 1.),]))
                .validate()
                .is_err()
        );

        assert!(Polygon::from(LineString::from(vec![
            (1., 1.),
            (3., 3.),
            (3., 1.),
            (1., 5.)
        ]))
        .validate()
        .is_err());

        assert!(Polygon::new(
            LineString::from(vec![(-5., -5.), (-5., 5.), (5., 5.), (5., -5.), (-5., -5.)]),
            vec![LineString::from(vec![(0., 0.), (3., 0.), (0., 0.),])],
        )
        .validate()
        .is_err());

        // XXX: According to microsoft validity, this is not valid because the
        // inner loop touches the outer loop in two places.  We don't test
        // for that currently.
        assert!(Polygon::new(
            LineString::from(vec![
                (-20., -20.),
                (-20., 20.),
                (20., 20.),
                (20., -20.),
                (-20., -20.)
            ]),
            vec![LineString::from(vec![
                (20., 0.),
                (0., 10.),
                (0., -20.),
                (20., 0.),
            ])],
        )
        .validate()
        .is_ok());

        assert!(Polygon::new(
            LineString::from(vec![
                (-20., -20.),
                (-20., 20.),
                (20., 20.),
                (20., -20.),
                (-20., -20.)
            ]),
            vec![
                LineString::from(vec![(10., 0.), (0., 10.), (0., -10.), (10., 0.),]),
                LineString::from(vec![(-10., 0.), (0., 10.), (0., -10.), (-10., 0.),])
            ],
        )
        .validate()
        .is_err());

        assert!(Polygon::new(
            LineString::from(vec![
                (-20., -20.),
                (-20., 20.),
                (20., 20.),
                (20., -20.),
                (-20., -20.)
            ]),
            vec![
                LineString::from(vec![(10., 0.), (0., 10.), (0., -10.), (10., 0.),]),
                LineString::from(vec![(-10., 0.), (1., 5.), (0., -10.), (-10., 0.),])
            ],
        )
        .validate()
        .is_err());

        assert!(Polygon::new(
            LineString::from(vec![(10., 0.), (0., 10.), (0., -10.), (10., 0.),]),
            vec![LineString::from(vec![
                (-20., -20.),
                (-20., 20.),
                (20., 20.),
                (20., -20.),
                (-20., -20.)
            ])],
        )
        .validate()
        .is_err());
    }
}
