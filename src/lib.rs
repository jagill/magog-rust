extern crate num_traits;

mod types;
pub use crate::types::*;

mod relation;
pub use crate::relation::*;

mod properties;
pub use crate::properties::*;

mod utils;

struct A {
    pub other: Option<Box<A>>,
}

fn make_A() -> A {
    let mut a = A{other: None};
    let mut b = A{other: None};
    a.other = Some(Box::new(b));
    b.other = Some(Box::new(a));
    return A{other: None};
}

#[cfg(test)]
mod tests {

    #[test]
    fn check_trivial() {
        assert!(true);
    }

}
