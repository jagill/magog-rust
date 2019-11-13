use crate::planar::primitives::Position;
use crate::planar::types::{
    Geometry, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon,
};
use wkt;

pub fn parse_wkt(wkt_str: &str) -> Result<Vec<Geometry<f64>>, &str> {
    let wkt_geoms = wkt::Wkt::from_str(wkt_str)?;
    let geoms = wkt_geoms.items.into_iter().map(from_wkt_geometry).collect();
    Ok(geoms)
}

fn from_wkt_geometry(geom: wkt::Geometry) -> Geometry<f64> {
    match geom {
        wkt::Geometry::Point(p) => match from_wkt_point(p) {
            None => Geometry::empty(),
            Some(point) => Geometry::from(point),
        },
        wkt::Geometry::LineString(ls) => Geometry::from(from_wkt_linestring(ls)),
        wkt::Geometry::Polygon(p) => match from_wkt_polygon(p) {
            None => Geometry::empty(),
            Some(poly) => Geometry::from(poly),
        },
        wkt::Geometry::MultiPoint(mp) => Geometry::from(from_wkt_multi_point(mp)),
        wkt::Geometry::MultiLineString(mls) => Geometry::from(from_wkt_multi_linestring(mls)),
        wkt::Geometry::MultiPolygon(mpoly) => Geometry::from(from_wkt_multi_polygon(mpoly)),
        _ => unimplemented!(),
    }
}

fn from_wkt_point(pt: wkt::types::Point) -> Option<Point<f64>> {
    let coord = pt.0?;
    Some(Point::from((coord.x, coord.y)))
}

fn from_wkt_linestring(ls: wkt::types::LineString) -> LineString<f64> {
    let positions =
        ls.0.into_iter()
            .map(|coord| Position::new(coord.x, coord.y))
            .collect();
    LineString::new(positions)
}

fn from_wkt_polygon(poly: wkt::types::Polygon) -> Option<Polygon<f64>> {
    let mut linestrings: Vec<LineString<f64>> =
        poly.0.into_iter().map(from_wkt_linestring).collect();
    if linestrings.is_empty() {
        return None;
    }
    let exterior = linestrings.remove(0);
    Some(Polygon::new(exterior, linestrings))
}

fn from_wkt_multi_point(mp: wkt::types::MultiPoint) -> MultiPoint<f64> {
    MultiPoint::new(mp.0.into_iter().filter_map(from_wkt_point).collect())
}

fn from_wkt_multi_linestring(mls: wkt::types::MultiLineString) -> MultiLineString<f64> {
    MultiLineString::new(mls.0.into_iter().map(from_wkt_linestring).collect())
}

fn from_wkt_multi_polygon(mpoly: wkt::types::MultiPolygon) -> MultiPolygon<f64> {
    MultiPolygon::new(mpoly.0.into_iter().filter_map(from_wkt_polygon).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_single_geom(wkt_str: &str) -> Geometry<f64> {
        let mut geoms = parse_wkt(wkt_str).unwrap();
        assert_eq!(geoms.len(), 1);
        geoms.remove(0)
    }

    fn assert_equals_point(wkt_str: &str, x: f64, y: f64) {
        let geom = get_single_geom(wkt_str);
        let point = geom.as_point().unwrap();
        assert_eq!(point, Point::from((x, y)));
    }

    fn assert_equals_linestring(wkt_str: &str, positions: Vec<(f64, f64)>) {
        let geom = get_single_geom(wkt_str);
        let linestring = geom.as_linestring().unwrap();
        assert_eq!(linestring, LineString::from(positions));
    }

    fn assert_equals_polygon(
        wkt_str: &str,
        exterior: Vec<(f64, f64)>,
        interiors: Vec<Vec<(f64, f64)>>,
    ) {
        let geom = get_single_geom(wkt_str);
        let polygon = geom.as_polygon().unwrap();
        let exterior_ls = LineString::from(exterior);
        let interiors_ls = interiors.into_iter().map(|i| LineString::from(i)).collect();
        assert_eq!(polygon, Polygon::new(exterior_ls, interiors_ls));
    }

    fn assert_equals_multipoint(wkt_str: &str, positions: Vec<(f64, f64)>) {
        let geom = get_single_geom(wkt_str);
        let multipoint = geom.as_multipoint().unwrap();
        assert_eq!(multipoint, MultiPoint::from(positions));
    }

    fn assert_equals_multilinestring(wkt_str: &str, positions: Vec<Vec<(f64, f64)>>) {
        let geom = get_single_geom(wkt_str);
        let multilinestring = geom.as_multilinestring().unwrap();
        assert_eq!(multilinestring, MultiLineString::from(positions));
    }

    fn assert_equals_multipolygon(wkt_str: &str, polys: Vec<Vec<(f64, f64)>>) {
        let geom = get_single_geom(wkt_str);
        let multipolygon = geom.as_multipolygon().unwrap();
        assert_eq!(multipolygon, MultiPolygon::from(polys));
    }

    #[test]
    fn check_empty_str() {
        assert_eq!(parse_wkt("").unwrap(), Vec::new());
    }

    #[test]
    fn check_bad_str() {
        assert!(parse_wkt("xyz").is_err());
    }

    #[test]
    fn check_point() {
        assert_equals_point("POINT(1.0 1.0)", 1.0, 1.0);
    }

    #[test]
    fn check_integer_point() {
        assert_equals_point("POINT (3 4)", 3.0, 4.0);
    }

    #[test]
    fn check_linestring_empty() {
        assert_equals_linestring("LINESTRING EMPTY", Vec::new());
    }

    #[test]
    fn check_linestring_single_point() {
        assert_equals_linestring("LINESTRING(1 1)", vec![(1.0, 1.0)]);
    }

    #[test]
    fn check_linestring_four_point() {
        assert_equals_linestring(
            "LINESTRING(1 1,2 3,4 8, -6 3)",
            vec![(1.0, 1.0), (2., 3.), (4., 8.), (-6., 3.)],
        );
    }

    #[test]
    fn check_linestring_duplicate() {
        assert_equals_linestring("LINESTRING(1 1, 1 1)", vec![(1.0, 1.0), (1., 1.)]);
    }

    // The wkt library doesn't deserialize these correctly
    // #[test]
    // fn check_polygon_empty() {
    //    assert_equals_polygon("POLYGON EMPTY", Vec::new(), Vec::new());
    // }

    #[test]
    fn check_polygon_simple() {
        assert_equals_polygon(
            "POLYGON((1 1, 3 3, 3 1, 1 1))",
            vec![(1., 1.), (3., 3.), (3., 1.), (1., 1.)],
            Vec::new(),
        );
    }

    #[test]
    fn check_polygon_interior() {
        assert_equals_polygon(
            "POLYGON((-5 -5, -5 5, 5 5, 5 -5, -5 -5),(0 0, 3 0, 3 3, 0 3, 0 0))",
            vec![(-5., -5.), (-5., 5.), (5., 5.), (5., -5.), (-5., -5.)],
            vec![vec![(0., 0.), (3., 0.), (3., 3.), (0., 3.), (0., 0.)]],
        );
    }

    #[test]
    fn check_polygon_two_interiors() {
        assert_equals_polygon(
            "POLYGON((-20 -20, -20 20, 20 20, 20 -20, -20 -20), (10 0, 0 10, 0 -10, 10 0), (-10 0, 0 10, -5 -10, -10 0))",
            vec![(-20., -20.), (-20., 20.), (20., 20.), (20., -20.), (-20., -20.)],
            vec![
                vec![(10., 0.), (0., 10.), (0., -10.), (10., 0.)],
                vec![(-10., 0.), (0., 10.), (-5., -10.), (-10., 0.)]
            ],
        );
    }

    #[test]
    fn check_multipoint() {
        assert_equals_multipoint("MULTIPOINT((2 3), (7 8))", vec![(2., 3.), (7., 8.)]);
    }

    #[test]
    fn check_multilinestring() {
        assert_equals_multilinestring(
            "MULTILINESTRING((1 1, 5 5), (1 3, 3 1))",
            vec![vec![(1., 1.), (5., 5.)], vec![(1., 3.), (3., 1.)]],
        )
    }

    #[test]
    fn check_multipolygon() {
        assert_equals_multipolygon(
            "MULTIPOLYGON(((1 1, 1 -1, -1 -1, -1 1, 1 1)),((1 1, 3 1, 3 3, 1 3, 1 1)))",
            vec![
                vec![(1., 1.), (1., -1.), (-1., -1.), (-1., 1.), (1., 1.)],
                vec![(1., 1.), (3., 1.), (3., 3.), (1., 3.), (1., 1.)],
            ],
        )
    }
}
