use crate::types::primitive::{Coordinate, CoordinateType, Rect};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Envelope<T>
where
    T: CoordinateType,
{
    pub rect: Option<Rect<T>>,
}

// Rect -> Envelope
impl<T: CoordinateType, IR: Into<Rect<T>>> From<IR> for Envelope<T> {
    fn from(rectish: IR) -> Self {
        Envelope {
            rect: Some(rectish.into()),
        }
    }
}

// Vec<Coordinate> -> Envelope
impl<'a, T: CoordinateType> From<&'a Vec<Coordinate<T>>> for Envelope<T> {
    fn from(coords: &'a Vec<Coordinate<T>>) -> Self {
        let mut iter = coords.iter().map(|c| c.clone());
        if let Some(c) = iter.next() {
            let mut min_x = c.x;
            let mut max_x = c.x;
            let mut min_y = c.y;
            let mut max_y = c.y;
            for c in iter {
                if c.x < min_x {
                    min_x = c.x
                }
                if c.x > max_x {
                    max_x = c.x
                }
                if c.y < min_y {
                    min_y = c.y
                }
                if c.y > max_y {
                    max_y = c.y
                }
            }
            return Envelope {
                rect: Some(Rect::from(((min_x, min_y), (max_x, max_y)))),
            };
        }
        Envelope { rect: None }
    }
}

// Vec<Envelope> -> Envelope
impl<'a, T: CoordinateType> From<&'a Vec<Envelope<T>>> for Envelope<T> {
    fn from(envelopes: &'a Vec<Envelope<T>>) -> Self {
        let env = Envelope { rect: None };
        envelopes.iter().fold(env, |base_env, e| base_env.merge(e))
    }
}

impl<T: CoordinateType> Envelope<T> {
    pub fn new(rect: Option<Rect<T>>) -> Envelope<T> {
        Envelope { rect }
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

    pub fn contains(&self, c: Coordinate<T>) -> bool {
        match &self.rect {
            None => false,
            Some(r) => r.contains(c),
        }
    }

    // pub fn add_coord(&mut self, c: Coordinate<T>) {
    //     match &self.rect {
    //         None => {
    //             self.rect = Some(Rect{min: c.clone(), max: c.clone()});
    //         },
    //         Some(r) => {
    //             r.add_coord(c);
    //         }
    //     }
    // }
    //
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
        let e = Envelope::from(&vec![Coordinate::new(0., 1.), Coordinate::new(2., 0.)]);
        let min: Coordinate<f64> = Coordinate { x: 0., y: 0. };
        let max: Coordinate<f64> = Coordinate { x: 2., y: 1. };
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
        let min: Coordinate<f64> = Coordinate { x: 0., y: 0. };
        let max: Coordinate<f64> = Coordinate { x: 3., y: 2. };
        assert_eq!(
            e,
            Envelope {
                rect: Some(Rect { min, max })
            }
        );
    }
}
