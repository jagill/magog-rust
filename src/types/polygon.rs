use crate::types::{CoordinateType, Envelope, Geometry, LineString};

#[derive(Debug, PartialEq)]
pub struct Polygon<T>
where
    T: CoordinateType,
{
    pub exterior: LineString<T>,
    pub interiors: Vec<LineString<T>>,
    pub envelope: Envelope<T>,
}

/// Turn a `Vec` of `Coordinate`-ish objects into a `Polygon`.
impl<T: CoordinateType, ILS: Into<LineString<T>>> From<ILS> for Polygon<T> {
    fn from(ext: ILS) -> Self {
        let exterior: LineString<T> = ext.into();
        let envelope = exterior.envelope().clone();
        Polygon {
            exterior: exterior,
            interiors: vec![],
            envelope: envelope,
        }
    }
}

/// Turn a `Vec` of `Coordinate`-ish objects into a `Polygon`.
// impl<'a, T: CoordinateType, ILS: Into<LineString<T>>> From<(ILS, &'a Vec<ILS>)> for Polygon<T> {
//     fn from(data: (ILS, &'a Vec<ILS>)) -> Self {
//         let exterior: LineString<T> = data.0.into();
//         let envelope = exterior.envelope.clone();
//         let interiors: Vec<LineString<T>> = data.1.iter().map(|ls| ls.into());
//         Polygon{exterior: exterior, interiors: vec![], envelope: envelope}
//     }
// }

impl<T> Polygon<T>
where
    T: CoordinateType,
{
    pub fn new(exterior: LineString<T>, interiors: Vec<LineString<T>>) -> Polygon<T> {
        let envelope = exterior.envelope().clone();
        Polygon {
            exterior,
            interiors,
            envelope,
        }
    }

    pub fn new_validate(
        exterior: LineString<T>,
        interiors: Vec<LineString<T>>,
    ) -> Result<Polygon<T>, &'static str> {
        let p = Polygon::new(exterior, interiors);
        p.validate()?;
        Ok(p)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        &self.exterior.validate()?;
        if !&self.exterior.is_closed() {
            return Err("Exterior is not a loop.");
        };
        for interior in &self.interiors {
            interior.validate()?;
            if !interior.is_closed() {
                return Err("Interior linestring is not a loop.");
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_basic_polygon() {
        let p = Polygon::from(vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, 0.0)]);
        assert_eq!(p.exterior.num_points(), 4);
        assert_eq!(p.interiors.len(), 0);
    }
}
