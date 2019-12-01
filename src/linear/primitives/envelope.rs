use crate::linear::primitives::Position;
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

// (Position, Position) -> Envelope
impl<C: Coordinate, IP: Into<Position<C>>> From<(IP, IP)> for Envelope<C> {
    fn from(positions: (IP, IP)) -> Self {
        Envelope::new(positions.0.into(), positions.1.into())
    }
}

impl<C: Coordinate> Envelope<C> {
    pub fn empty() -> Self {
        Envelope::Empty
    }

    pub fn new(position1: Position<C>, position2: Position<C>) -> Self {
        Envelope::Bounds {
            min: position1.min(position2),
            max: position1.max(position2),
        }
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
                if min > max {
                    Err("Envelope min is greater than max")
                } else {
                    Ok(())
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Envelope::Empty
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
            ) => min1 <= min2 && max2 <= max1,
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
            ) => min1 <= max2 && min2 <= max1,
        }
    }

    pub fn merge(&self, other: impl HasEnvelope<C>) -> Self {
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
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use core::f32;

    #[test]
    fn test_is_empty() {
        assert!(Envelope::<f32>::empty().is_empty());
        assert!(!Envelope::from((0., 0.)).is_empty());
        assert!(!Envelope::from((0., 1.)).is_empty());
        assert!(!Envelope::from((1., 0.)).is_empty());
    }

    #[test]
    fn test_contains() {
        assert!(Envelope::from((0., 0.)).contains(Position::new(0.)));
        assert!(!Envelope::from((0., 1.)).contains(Position::new(-0.1)));
        assert!(Envelope::from((0., 1.)).contains(Envelope::from((1., 0.))));
        assert!(!Envelope::from((0., 1.)).contains(Envelope::from((0.5, 2.))));
        assert!(!Envelope::from((0., 1.)).contains(Envelope::from((1.1, 2.))));
    }

    #[test]
    fn test_intersects() {
        assert!(Envelope::from((0., 1.)).intersects(Envelope::from((1., 0.))));
        assert!(Envelope::from((0., 1.)).intersects(Envelope::from((0.5, 2.))));
        assert!(!Envelope::from((0., 1.)).intersects(Envelope::from((1.1, 2.))));
    }

    #[test]
    fn test_merge() {
        let env1 = Envelope::from((0., 1.));
        let p = Position::new(2.);
        let env2 = env1.merge(p);
        match env1 {
            Envelope::Empty => panic!(),
            Envelope::Bounds { min, max } => {
                assert_eq!(min.x, 0.0);
                assert_eq!(max.x, 1.0);
            }
        }
        match env2 {
            Envelope::Empty => panic!(),
            Envelope::Bounds { min, max } => {
                assert_eq!(min.x, 0.0);
                assert_eq!(max.x, 2.0);
            }
        }
    }

    #[test]
    fn test_expand() {
        let mut env1 = Envelope::from((0., 1.));
        let p = Position::new(2.);
        env1.expand(p);
        match env1 {
            Envelope::Empty => panic!(),
            Envelope::Bounds { min, max } => {
                assert_eq!(min.x, 0.0);
                assert_eq!(max.x, 2.0);
            }
        }
    }

    #[test]
    fn test_of() {
        let env =
            Envelope::of(vec![Position::new(0.), Position::new(1.), Position::new(-1.)].iter());
        assert_eq!(env, Envelope::from((-1., 1.)));

        let env = Envelope::of(
            vec![
                Envelope::from((0., 1.)),
                Envelope::from((1., 1.)),
                Envelope::from((-1., 1.)),
            ]
            .iter(),
        );
        assert_eq!(env, Envelope::from((-1., 1.)));
    }
}
