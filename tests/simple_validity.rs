use magog::planar::types::{LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon};

#[test]
fn validity_simple_point_valid() {
    Point::from((1., 2.)).validate().unwrap();
}

#[test]
fn validity_simple_multipoint_valid() {
    MultiPoint::from(vec![(1., 2.), (3., 4.)])
        .validate()
        .unwrap();
}

#[test]
fn validity_simple_linestring_valid() {
    LineString::from(vec![(0., 0.), (1., 2.), (3., 4.)])
        .validate()
        .unwrap();
}

#[test]
fn validity_simple_multilinestring_valid() {
    MultiLineString::from(vec![vec![(0., 0.), (1., 2.)], vec![(3., 4.), (4., 3.)]])
        .validate()
        .unwrap();
}

#[test]
fn validity_simple_polygon_valid() {
    Polygon::from(vec![(0., 0.), (0., 1.), (1., 1.), (1., 0.), (0., 0.)])
        .validate()
        .unwrap();
}

#[test]
fn validity_simple_multipolygon_valid() {
    MultiPolygon::from(vec![
        vec![(1., 1.), (1., 3.), (3., 3.), (3., 1.), (1., 1.)],
        vec![(2., 4.), (2., 6.), (6., 6.), (6., 4.), (2., 4.)],
    ])
    .validate()
    .unwrap();
}

#[test]
fn validity_simple_multipoint_invalid() {
    assert!(
        MultiPoint::from(vec![(0., 0.), (0., 1.), (1., 1.), (0., 1.)])
            .validate()
            .is_err()
    );
}

#[test]
fn validity_linestring_invalid_repeated_points() {
    assert!(LineString::from(vec![
        (0., 0.),
        (0., 1.),
        (0., 1.),
        (1., 1.),
        (1., 0.),
        (0., 0.)
    ])
    .validate()
    .is_err())
}

#[test]
fn validity_linestring_invalid_self_tangency() {
    assert!(LineString::from(vec![
        (0., 0.),
        (-1., 0.5),
        (0., 1.),
        (1., 1.),
        (1., 0.),
        (0., 1.),
        (0., 0.)
    ])
    .validate()
    .is_err())
}

#[test]
fn validity_polygon_invalid_self_intersection() {
    assert!(
        Polygon::from(vec![(0., 0.), (1., 1.), (0., 1.), (1., 0.), (0., 0.)])
            .validate()
            .is_err()
    );
}

#[test]
fn validity_polygon_invalid_degenerate_segment() {
    assert!(Polygon::from(vec![
        (0., 0.),
        (0., 1.),
        (0., 1.),
        (1., 1.),
        (1., 0.),
        (0., 0.)
    ],)
    .validate()
    .is_err());
}

#[test]
fn validity_polygon_invalid_degenerate_segment_inner_loop() {
    assert!(Polygon::new(
        vec![(0., 0.), (0., 1.), (0., 1.), (1., 1.), (1., 0.), (0., 0.)].into(),
        vec![vec![(2., 2.), (2., 3.), (3., 3.), (3., 2.), (2., 2.)].into()]
    )
    .validate()
    .is_err());
}

#[test]
fn validity_polygon_invalid_overlapping_segment() {
    assert!(Polygon::from(vec![
        (0., 0.),
        (0., 1.),
        (2., 1.),
        (1., 1.),
        (1., 0.),
        (0., 0.)
    ],)
    .validate()
    .is_err());
}

#[test]
fn validity_polygon_invalid_inner_loop_self_intersection() {
    assert!(Polygon::new(
        vec![(0., 0.), (0., 1.), (1., 1.), (1., 0.), (0., 0.)].into(),
        vec![vec![(0., 1.), (1., 1.), (0.5, 0.5), (0., 1.)].into()]
    )
    .validate()
    .is_err());
}

#[test]
fn validity_polygon_valid_disconnected_interior() {
    assert!(Polygon::new(
        vec![(0., 0.), (0., 1.), (1., 1.), (1., 0.), (0., 0.)].into(),
        vec![vec![(0., 0.), (0.5, 0.7), (1., 1.), (0.5, 0.4), (0., 0.)].into()]
    )
    .validate()
    .is_ok());
}

#[test]
fn validity_polygon_invalid_self_tangency() {
    assert!(Polygon::from(vec![
        (0., 0.),
        (-1., 0.5),
        (0., 1.),
        (1., 1.),
        (1., 0.),
        (0., 1.),
        (0., 0.)
    ],)
    .validate()
    .is_err());
}

// #[test]
// fn validity_multipolygon_invalid_self_intersection() {
//     assert!(MultiPolygon::from(vec![
//         vec![(0., 0.), (0., 1.), (1., 1.), (0., 1.), (0., 0.)],
//         vec![(0.5, 0.5), (0.5, 2.), (2., 2.), (2., 0.5), (0.5, 0.5)],
//     ])
//     .validate()
//     .is_err());
// }
