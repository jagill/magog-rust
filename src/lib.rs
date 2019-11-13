pub mod coordinate;
pub mod flatbush;
pub mod primitives;
pub mod serde;
pub mod types;

mod algorithms;
mod linear;
mod validation;

pub use crate::coordinate::Coordinate;

#[cfg(test)]
mod tests {

    #[test]
    fn check_trivial() {
        assert!(true);
    }
}
