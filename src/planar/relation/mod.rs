mod contains;
pub use crate::relation::contains::*;

/// Location of a point in relation to a line
#[derive(PartialEq, Clone, Debug)]
pub enum Intersection {
    Contains,
    Boundary,
    Outside,
}

/// OGC Geometry relationships.
#[derive(PartialEq, Clone, Debug)]
pub enum Relation {
    Touches,
    Contains,
    Intersects,
    Within,
    Crosses,
    Overlaps,
}

impl Relation {
    pub fn invert(self) -> Relation {
        match self {
            Relation::Within => Relation::Contains,
            Relation::Contains => Relation::Within,
            _ => self,
        }
    }
}
