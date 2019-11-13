use magog::flatbush::Flatbush;
use magog::planar::primitives::HasEnvelope;
use magog::planar::types::{Point, Polygon};

#[test]
fn test_octagons() {
    let octagon_a: Polygon<f32> = Polygon::from(vec![
        (0.0, 0.0),
        (0.5, 2.5),
        (0.0, 5.0),
        (2.5, 5.5),
        (5.0, 5.0),
        (5.5, 2.5),
        (5.0, 0.0),
        (2.5, 0.5),
        (0.0, 0.0),
    ]);

    let octagon_b: Polygon<f32> = Polygon::from(vec![
        (4.0, 4.0),
        (3.5, 7.0),
        (4.0, 10.0),
        (7.0, 10.5),
        (10.0, 10.0),
        (10.5, 7.0),
        (10.0, 4.0),
        (7.0, 3.5),
        (4.0, 4.0),
    ]);

    let point_x: Point<f32> = Point::from((1.0, 1.0));
    let point_y: Point<f32> = Point::from((4.5, 4.5));
    let point_w: Point<f32> = Point::from((20.0, 20.0));
    let point_z: Point<f32> = Point::from((6.0, 6.0));

    let point_rtree = Flatbush::new(&vec![point_x, point_y, point_z, point_w], 8);
    let mut candidates_a = point_rtree.find_intersection_candidates(octagon_a.envelope());
    candidates_a.sort();
    assert_eq!(vec![0, 1], candidates_a);
    let mut candidates_b = point_rtree.find_intersection_candidates(octagon_b.envelope());
    candidates_b.sort();
    assert_eq!(vec![1, 2], candidates_b);
}
