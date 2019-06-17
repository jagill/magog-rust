use crate::primitives::{Coordinate, Position, Rect};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Envelope<C: Coordinate> {
    pub rect: Option<Rect<C>>,
}

// Rect -> Envelope
impl<C: Coordinate, IR: Into<Rect<C>>> From<IR> for Envelope<C> {
    fn from(rectish: IR) -> Self {
        Envelope {
            rect: Some(rectish.into()),
        }
    }
}

// Vec<Position> -> Envelope
impl<'a, C: Coordinate> From<&'a Vec<Position<C>>> for Envelope<C> {
    fn from(positions: &'a Vec<Position<C>>) -> Self {
        positions
            .iter()
            .fold(Envelope::empty(), |env, p| env.add_position(*p))
    }
}

// Vec<Envelope> -> Envelope
impl<'a, C: Coordinate> From<&'a Vec<Envelope<C>>> for Envelope<C> {
    fn from(envelopes: &'a Vec<Envelope<C>>) -> Self {
        envelopes
            .iter()
            .fold(Envelope::empty(), |env, e| env.merge(*e))
    }
}

impl<C: Coordinate> Envelope<C> {
    pub fn new(rect: Option<Rect<C>>) -> Envelope<C> {
        Envelope { rect }
    }

    pub fn empty() -> Envelope<C> {
        Envelope { rect: None }
    }

    pub fn from_envelopes(envs: impl Iterator<Item = Envelope<C>>) -> Envelope<C> {
        envs.fold(Envelope::empty(), |base, env| base.merge(env))
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        match &self.rect {
            None => Ok(()),
            Some(r) => r.validate(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rect == None
    }

    pub fn contains(&self, p: Position<C>) -> bool {
        match &self.rect {
            None => false,
            Some(r) => r.contains(p),
        }
    }

    pub fn intersects(&self, other_env: Envelope<C>) -> bool {
        match (&self.rect, other_env.rect) {
            (None, _) | (_, None) => false,
            (Some(rect), Some(other_rect)) => rect.intersects(other_rect),
        }
    }

    pub fn add_position(&self, p: Position<C>) -> Envelope<C> {
        match &self.rect {
            None => Envelope {
                rect: Some(Rect::new(p, p)),
            },
            Some(r) => Envelope {
                rect: Some(r.add_position(p)),
            },
        }
    }

    pub fn merge(&self, other: Envelope<C>) -> Envelope<C> {
        match (self.rect, other.rect) {
            (None, None) => Envelope::empty(),
            (None, Some(r)) | (Some(r), None) => r.into(),
            (Some(r1), Some(r2)) => r1.merge(r2).into(),
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
        assert_eq!(
            e,
            Envelope {
                rect: Some(Rect { min, max })
            }
        );
    }

    #[test]
    fn check_from_tuple_positions() {
        let e = Envelope::from((Position::new(0., 1.), Position::new(2., 0.)));
        let min: Position<f64> = Position { x: 0., y: 0. };
        let max: Position<f64> = Position { x: 2., y: 1. };
        assert_eq!(
            e,
            Envelope {
                rect: Some(Rect { min, max })
            }
        );
    }

    #[test]
    fn check_from_vec_positions() {
        let e = Envelope::from(&vec![Position::new(0., 1.), Position::new(2., 0.)]);
        let min: Position<f64> = Position { x: 0., y: 0. };
        let max: Position<f64> = Position { x: 2., y: 1. };
        assert_eq!(
            e,
            Envelope {
                rect: Some(Rect { min, max })
            }
        );
    }

    #[test]
    fn check_from_vec_envelops() {
        let e = Envelope::from(&vec![
            Envelope::from(((0., 1.), (2., 0.))),
            Envelope::from(((0., 2.), (3., 0.))),
        ]);
        let min: Position<f64> = Position { x: 0., y: 0. };
        let max: Position<f64> = Position { x: 3., y: 2. };
        assert_eq!(
            e,
            Envelope {
                rect: Some(Rect { min, max })
            }
        );
    }

}
