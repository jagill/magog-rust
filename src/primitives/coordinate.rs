use num_traits::{Bounded, Float, Signed};
use std::fmt::Debug;
use std::iter::Sum;

pub trait Coordinate: Float + Sum + Bounded + Signed + Debug + Send + Sync {
    /**
     * Order self, other into (min, max).
     *
     * If self or other is NAN, set min/max to be the other.
     * If both are NAN, return (NAN, NAN).
     */
    fn min_max(&self, other: Self) -> (Self, Self) {
        (self.min(other), self.max(other))
    }
}

impl<C: Float + Sum + Bounded + Signed + Debug + Send + Sync> Coordinate for C {}
