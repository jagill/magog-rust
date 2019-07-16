pub mod primitives;
pub mod types;
pub mod flatbush;

mod algorithms;
mod validation;

#[cfg(test)]
mod tests {

    #[test]
    fn check_trivial() {
        assert!(true);
    }

}
