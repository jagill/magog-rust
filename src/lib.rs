pub mod coordinate;
pub mod flatbush;
pub mod linear;
pub mod planar;
pub mod serde;

pub use crate::coordinate::Coordinate;

#[cfg(test)]
mod tests {

    #[test]
    fn check_trivial() {
        assert!(true);
    }
}
