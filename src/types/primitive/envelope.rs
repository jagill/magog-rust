use crate::types::primitive::{Coord2, Coordinate, Rect};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Envelope<T>
where
    T: Coordinate,
{
    pub rect: Option<Rect<T>>,
}

// Rect -> Envelope
impl<T: Coordinate, IR: Into<Rect<T>>> From<IR> for Envelope<T> {
    fn from(rectish: IR) -> Self {
        Envelope {
            rect: Some(rectish.into()),
        }
    }
}

// Vec<Coord2> -> Envelope
impl<'a, T: Coordinate> From<&'a Vec<Coord2<T>>> for Envelope<T> {
    fn from(coords: &'a Vec<Coord2<T>>) -> Self {
        let empty_env = Envelope { rect: None };
        coords.iter().fold(empty_env, |env, c| env.add_coord(*c))
    }
}

// Vec<Envelope> -> Envelope
impl<'a, T: Coordinate> From<&'a Vec<Envelope<T>>> for Envelope<T> {
    fn from(envelopes: &'a Vec<Envelope<T>>) -> Self {
        let env = Envelope { rect: None };
        envelopes.iter().fold(env, |base_env, e| base_env.merge(e))
    }
}

impl<T: Coordinate> Envelope<T> {
    pub fn new(rect: Option<Rect<T>>) -> Envelope<T> {
        Envelope { rect }
    }

    pub fn empty() -> Envelope<T> {
        Envelope { rect: None }
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

    pub fn contains(&self, c: Coord2<T>) -> bool {
        match &self.rect {
            None => false,
            Some(r) => r.contains(c),
        }
    }

    pub fn add_coord(&self, c: Coord2<T>) -> Envelope<T> {
        match &self.rect {
            None => Envelope {
                rect: Some(Rect::new(c.clone(), c.clone())),
            },
            Some(r) => Envelope {
                rect: Some(r.add_coord(c)),
            },
        }
    }

    pub fn merge(&self, other: &Envelope<T>) -> Envelope<T> {
        match &self.rect {
            None => Envelope {
                rect: other.rect.clone(),
            },
            Some(r) => match other.rect {
                None => self.clone(),
                Some(other_r) => Envelope {
                    rect: Some(r.merge(other_r)),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_from_vec_coords() {
        let e = Envelope::from(&vec![Coord2::new(0., 1.), Coord2::new(2., 0.)]);
        let min: Coord2<f64> = Coord2 { x: 0., y: 0. };
        let max: Coord2<f64> = Coord2 { x: 2., y: 1. };
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
        let min: Coord2<f64> = Coord2 { x: 0., y: 0. };
        let max: Coord2<f64> = Coord2 { x: 3., y: 2. };
        assert_eq!(
            e,
            Envelope {
                rect: Some(Rect { min, max })
            }
        );
    }
}
