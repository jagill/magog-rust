use num_traits::{Bounded, Float, Signed};
use std::fmt::Debug;
use std::iter::Sum;

pub trait Coordinate: Float + Sum + Bounded + Signed + Debug + 'static {}
impl<C: Float + Sum + Bounded + Signed + Debug + 'static> Coordinate for C {}
