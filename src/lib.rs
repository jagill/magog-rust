pub mod primitives;
pub mod types;

mod algorithms;
mod flatbush;
mod validation;

#[cfg(test)]
mod tests {

    #[test]
    fn check_trivial() {
        assert!(true);
    }

}
