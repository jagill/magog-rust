mod contains;
pub use crate::relation::contains::*;

/// Location of a point in relation to a line
#[derive(PartialEq, Clone, Debug)]
pub enum Intersection {
    Contains,
    Boundary,
    Outside,
}
