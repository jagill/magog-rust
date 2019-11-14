use super::Position;
use crate::Coordinate;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Envelope<C: Coordinate> {
    Empty,
    Bounds { min: Position<C>, max: Position<C> },
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
        Envelope::new(*self, *self)
    }
}

impl<C: Coordinate> Envelope<C> {
    pub fn empty() -> Self {
        Envelope::Empty
    }

    pub fn new(pos1: Position<C>, pos2: Position<C>) -> Self {
        let (min, max) = Position::min_max(pos1, pos2);
        Envelope::Bounds { min, max }
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
            Envelope::Bounds { min, max } => {
                min.validate()?;
                max.validate()?;
                if min.x > max.x {
                    return Err("Envelope min.x is greater than max.x");
                }
                if min.y > max.y {
                    return Err("Envelope min.y is greater than max.y");
                }
                Ok(())
            }
        }
    }

    pub fn min(&self) -> Option<Position<C>> {
        match self {
            Envelope::Empty => None,
            Envelope::Bounds { min, max } => Some(*min),
        }
    }

    pub fn max(&self) -> Option<Position<C>> {
        match self {
            Envelope::Empty => None,
            Envelope::Bounds { min, max } => Some(*max),
        }
    }

    pub fn contains(&self, other: impl HasEnvelope<C>) -> bool {
        match (*self, other.envelope()) {
            (Envelope::Empty, _) | (_, Envelope::Empty) => false,
            (
                Envelope::Bounds {
                    min: min1,
                    max: max1,
                },
                Envelope::Bounds {
                    min: min2,
                    max: max2,
                },
            ) => min1.x <= min2.x && max2.x <= max1.x && min1.y <= min2.y && max2.y <= max2.y,
        }
    }

    pub fn intersects(&self, other: Envelope<C>) -> bool {
        match (*self, other) {
            (Envelope::Empty, _) | (_, Envelope::Empty) => false,
            (
                Envelope::Bounds {
                    min: min1,
                    max: max1,
                },
                Envelope::Bounds {
                    min: min2,
                    max: max2,
                },
            ) => min1.x <= max2.x && max1.x >= min2.x && min1.y <= max2.y && max1.y >= min2.y,
        }
    }

    pub fn merge(&self, other: impl HasEnvelope<C>) -> Envelope<C> {
        match (*self, other.envelope()) {
            (Envelope::Empty, x) | (x, Envelope::Empty) => x,
            (
                Envelope::Bounds {
                    min: min1,
                    max: max1,
                },
                Envelope::Bounds {
                    min: min2,
                    max: max2,
                },
            ) => Envelope::Bounds {
                min: min1.min(min2),
                max: max1.max(max2),
            },
        }
    }

    pub fn expand(&mut self, other: impl HasEnvelope<C>) {
        match (&self, other.envelope()) {
            (_, Envelope::Empty) => (),
            (Envelope::Empty, x) => *self = x,
            (
                Envelope::Bounds {
                    min: min1,
                    max: max1,
                },
                Envelope::Bounds {
                    min: min2,
                    max: max2,
                },
            ) => {
                *self = Envelope::Bounds {
                    min: min1.min(min2),
                    max: max1.max(max2),
                }
            }
        }
    }

    pub fn center(&self) -> Option<Position<C>> {
        match self {
            Envelope::Empty => None,
            Envelope::Bounds { min, max } => Some((*max + *min) / (C::one() + C::one())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_from_tuple_tuples() {
        let e = Envelope::new((0., 1.).into(), (2., 0.).into());
        let min: Position<f64> = Position { x: 0., y: 0. };
        let max: Position<f64> = Position { x: 2., y: 1. };
        assert_eq!(e, Envelope::Bounds { min, max });
    }

    #[test]
    fn check_from_vec_positions() {
        let e = Envelope::of(vec![Position::new(0., 1.), Position::new(2., 0.)].iter());
        let min: Position<f64> = Position { x: 0., y: 0. };
        let max: Position<f64> = Position { x: 2., y: 1. };
        assert_eq!(e, Envelope::Bounds { min, max });
    }

    #[test]
    fn check_from_vec_envelops() {
        let e = Envelope::of(
            vec![
                Envelope::new((0., 1.).into(), (2., 0.).into()),
                Envelope::new((0., 2.).into(), (3., 0.).into()),
            ]
            .iter(),
        );
        let min: Position<f64> = Position { x: 0., y: 0. };
        let max: Position<f64> = Position { x: 3., y: 2. };
        assert_eq!(e, Envelope::Bounds { min, max });
    }
}
