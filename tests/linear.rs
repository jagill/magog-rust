use magog::linear::primitives::{Envelope, HasEnvelope};
use magog::linear::types::{Line, MultiLine, MultiPoint, Point};

#[test]
fn test_point() {
    let p = Point::from(0.0);
    let mp = MultiPoint::from(vec![0.0, 1.0]);
    assert_eq!(mp.get_point(0).unwrap(), p);
}

#[test]
fn test_line() {
    let l = Line::from((0., 1.));
    let ml = MultiLine::from(vec![(0., 1.), (2., 3.)]);
    assert_eq!(ml.get_line(0).unwrap(), l);
}

#[test]
fn test_envelope_of() {
    let p = Point::from(-1.0);
    let mp = MultiPoint::from(vec![0.0, 1.0]);
    let l = Line::from((0., 1.));
    let ml = MultiLine::from(vec![(0., 1.), (2., 3.)]);
    let env = Envelope::of(vec![p.envelope(), mp.envelope(), l.envelope(), ml.envelope()].iter());
    assert_eq!(env, Envelope::from((-1.0, 3.)));
}
