use super::rect::Rect;
use super::Position;
use crate::Coordinate;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Envelope<C: Coordinate> {
    Empty,
    Bounds(Rect<C>),
}

pub trait HasEnvelope<C: Coordinate> {
    fn envelope(&self) -> Envelope<C>;
}

impl<C: Coordinate> HasEnvelope<C> for Envelope<C> {
    fn envelope(&self) -> Envelope<C> {
        *self
    }
}

impl<C: Coordinate> HasEnvelope<C> for Position<C> {
    fn envelope(&self) -> Envelope<C> {
        Envelope::from((*self, *self))
    }
}

// Rect -> Envelope
impl<C: Coordinate, IR: Into<Rect<C>>> From<IR> for Envelope<C> {
    fn from(rectish: IR) -> Self {
        Envelope::Bounds(rectish.into())
    }
}

impl<C: Coordinate> Envelope<C> {
    pub fn empty() -> Self {
        Envelope::Empty
    }

    pub fn new(rect: Option<Rect<C>>) -> Self {
        match rect {
            None => Envelope::Empty,
            Some(r) => Envelope::Bounds(r),
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Envelope::Empty
    }

    pub fn of<'a>(objs: impl Iterator<Item = &'a (impl HasEnvelope<C> + 'a)>) -> Self {
        objs.fold(Envelope::empty(), |base, other| {
            base.merge(other.envelope())
        })
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        match self {
            Envelope::Empty => Ok(()),
            Envelope::Bounds(r) => r.validate(),
        }
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
        let e = Envelope::of(vec![Position::new(0., 1.), Position::new(2., 0.)].iter());
        let min: Position<f64> = Position { x: 0., y: 0. };
        let max: Position<f64> = Position { x: 2., y: 1. };
        assert_eq!(e, Envelope::Bounds(Rect { min, max }));
    }

    #[test]
    fn check_from_vec_envelops() {
        let e = Envelope::of(
            vec![
                Envelope::from(((0., 1.), (2., 0.))),
                Envelope::from(((0., 2.), (3., 0.))),
            ]
            .iter(),
        );
        let min: Position<f64> = Position { x: 0., y: 0. };
        let max: Position<f64> = Position { x: 3., y: 2. };
        assert_eq!(e, Envelope::Bounds(Rect { min, max }));
    }
}
