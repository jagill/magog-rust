use crate::types::{CoordinateType, Envelope, Geometry, LineString};

#[derive(Debug, PartialEq)]
pub struct MultiLineString<T>
where
    T: CoordinateType,
{
    pub line_strings: Vec<LineString<T>>,
    pub envelope: Envelope<T>,
}

impl<T: CoordinateType> MultiLineString<T> {
    pub fn new(line_strings: Vec<LineString<T>>) -> Self {
        let envs: Vec<Envelope<T>> = line_strings.iter().map(|ls| ls.envelope()).collect();
        let envelope = Envelope::from(&envs);
        MultiLineString {
            line_strings,
            envelope,
        }
    }
}
