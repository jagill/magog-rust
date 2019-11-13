use crate::planar::primitives::{Position, Rect};
use crate::Coordinate;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Envelope<C: Coordinate> {
    Empty,
    Bounds(Rect<C>),
}

// Rect -> Envelope
impl<C: Coordinate, IR: Into<Rect<C>>> From<IR> for Envelope<C> {
    fn from(rectish: IR) -> Self {
        Envelope::Bounds(rectish.into())
    }
}

// Vec<Position> -> Envelope
impl<'a, C: Coordinate> From<&'a Vec<Position<C>>> for Envelope<C> {
    fn from(positions: &'a Vec<Position<C>>) -> Self {
        positions
            .iter()
            .fold(Envelope::Empty, |env, p| env.add_position(*p))
    }
}

// Vec<Envelope> -> Envelope
impl<'a, C: Coordinate> From<&'a Vec<Envelope<C>>> for Envelope<C> {
    fn from(envelopes: &'a Vec<Envelope<C>>) -> Self {
        envelopes
            .iter()
            .fold(Envelope::Empty, |env, e| env.merge(*e))
    }
}

impl<C: Coordinate> Envelope<C> {
    pub fn new(rect: Option<Rect<C>>) -> Envelope<C> {
        match rect {
            None => Envelope::Empty,
            Some(r) => Envelope::Bounds(r),
        }
    }

    pub fn from_envelopes(envs: impl Iterator<Item = Envelope<C>>) -> Envelope<C> {
        envs.fold(Envelope::Empty, |base, env| base.merge(env))
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        match self {
            Envelope::Empty => Ok(()),
            Envelope::Bounds(r) => r.validate(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Envelope::Empty
    }

    pub fn contains(&self, p: Position<C>) -> bool {
        match self {
            Envelope::Empty => false,
            Envelope::Bounds(r) => r.contains(p),
        }
    }

    pub fn intersects(&self, other: Envelope<C>) -> bool {
        match (self, other) {
            (Envelope::Empty, _) | (_, Envelope::Empty) => false,
            (Envelope::Bounds(rect), Envelope::Bounds(other_rect)) => rect.intersects(other_rect),
        }
    }

    pub fn add_position(&self, p: Position<C>) -> Envelope<C> {
        match &self {
            Envelope::Empty => Envelope::Bounds(Rect::new(p, p)),
            Envelope::Bounds(r) => Envelope::Bounds(r.add_position(p)),
        }
    }

    pub fn merge(&self, other: Envelope<C>) -> Envelope<C> {
        match (*self, other) {
            (Envelope::Empty, Envelope::Empty) => Envelope::Empty,
            (Envelope::Empty, Envelope::Bounds(r)) | (Envelope::Bounds(r), Envelope::Empty) => {
                Envelope::Bounds(r)
            }
            (Envelope::Bounds(r1), Envelope::Bounds(r2)) => Envelope::Bounds(r1.merge(r2)),
        }
    }

    pub fn center(&self) -> Option<Position<C>> {
        match self {
            Envelope::Empty => None,
            Envelope::Bounds(r) => Some(r.center()),
        }
    }
}

pub trait HasEnvelope<C: Coordinate> {
    fn envelope(&self) -> Envelope<C>;
}

impl<C: Coordinate> HasEnvelope<C> for Envelope<C> {
    fn envelope(&self) -> Envelope<C> {
        return *self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_from_tuple_tuples() {
        let e = Envelope::from(((0., 1.), (2., 0.)));
        let min: Position<f64> = Position { x: 0., y: 0. };
        let max: Position<f64> = Position { x: 2., y: 1. };
        assert_eq!(e, Envelope::Bounds(Rect { min, max }));
    }

    #[test]
    fn check_from_tuple_positions() {
        let e = Envelope::from((Position::new(0., 1.), Position::new(2., 0.)));
        let min: Position<f64> = Position { x: 0., y: 0. };
        let max: Position<f64> = Position { x: 2., y: 1. };
        assert_eq!(e, Envelope::Bounds(Rect { min, max }));
    }

    #[test]
    fn check_from_vec_positions() {
        let e = Envelope::from(&vec![Position::new(0., 1.), Position::new(2., 0.)]);
        let min: Position<f64> = Position { x: 0., y: 0. };
        let max: Position<f64> = Position { x: 2., y: 1. };
        assert_eq!(e, Envelope::Bounds(Rect { min, max }));
    }

    #[test]
    fn check_from_vec_envelops() {
        let e = Envelope::from(&vec![
            Envelope::from(((0., 1.), (2., 0.))),
            Envelope::from(((0., 2.), (3., 0.))),
        ]);
        let min: Position<f64> = Position { x: 0., y: 0. };
        let max: Position<f64> = Position { x: 3., y: 2. };
        assert_eq!(e, Envelope::Bounds(Rect { min, max }));
    }
}
