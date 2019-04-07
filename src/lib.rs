extern crate num_traits;

mod primitives;
pub use crate::primitives::*;

mod types;
pub use crate::types::*;

mod rtree;

// mod relation;
// pub use crate::relation::*;
//
// mod properties;
// pub use crate::properties::*;

#[cfg(test)]
mod tests {

    #[test]
    fn check_trivial() {
        assert!(true);
    }

}
