use crate::types::{CoordinateType, Envelope, Geometry, LineString};

#[derive(Debug, PartialEq)]
pub struct MultiLineString<T>
where
    T: CoordinateType,
{
    pub line_strings: Vec<LineString<T>>,
    _envelope: Envelope<T>,
}

impl<T: CoordinateType> MultiLineString<T> {
    pub fn new(line_strings: Vec<LineString<T>>) -> Self {
        let envs: Vec<Envelope<T>> = line_strings.iter().map(|ls| ls.envelope()).collect();
        let _envelope = Envelope::from(&envs);
        MultiLineString {
            line_strings,
            _envelope,
        }
    }
}

// MultiLineString implementation
impl<T: CoordinateType> MultiLineString<T> {
    pub fn is_closed(&self) -> bool {
        self.line_strings.iter().all(|ls| ls.is_closed())
    }

    pub fn length(&self) -> T {
        self.line_strings.iter().map(|ls| ls.length()).sum()
    }
}

// GEOMETRY implementation
impl<T: CoordinateType> MultiLineString<T> {
    pub fn dimension(&self) -> u8 {
        1
    }

    pub fn geometry_type(&self) -> &'static str {
        "MultiLineString"
    }

    pub fn envelope(&self) -> Envelope<T> {
        self._envelope
    }

    pub fn is_empty(&self) -> bool {
        self.line_strings.iter().all(|ls| ls.is_empty())
    }

    /// A MultiLineString is simple if each LineString is simple, and none
    /// intersect each other.
    pub fn is_simple(&self) -> bool {
        self.line_strings.iter().all(|ls| ls.is_simple())
        // TODO: STUB
          && true
    }

    /// The boundary of a MultiLineString is are the boundaries of
    /// the component LineStrings that don't touch any other LineString.
    pub fn boundary(&self) -> Geometry<T> {
        // TODO: STUB
        Geometry::Empty
    }
}
