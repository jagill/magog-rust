extern crate num_traits;

mod types;
pub use crate::types::*;

mod relation;
pub use crate::relation::*;

mod properties;
pub use crate::properties::*;

mod iter_tests;

#[cfg(test)]
mod tests {

    #[test]
    fn check_trivial() {
        assert!(true);
    }

}
