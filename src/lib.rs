pub mod flatbush;
pub mod primitives;
pub mod serde;
pub mod types;

mod algorithms;
mod validation;

#[cfg(test)]
mod tests {

    #[test]
    fn check_trivial() {
        assert!(true);
    }

}
