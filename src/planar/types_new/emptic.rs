use super::{Empty, Geometric};
use crate::planar::primitives::{Envelope, HasEnvelope};
use crate::Coordinate;

enum Emptic<C: Coordinate, G: Geometric<C>> {
    Empty(Empty<C>),
    Some(G),
}

macro_rules! delegate_accessor {
    // This macro takes the name of an accessor function and delegates it
    // to each of the options of the Geometry Enum.
    ($func_name:ident, $ret_type:ty) => (
        fn $func_name(&self) -> $ret_type {
            match self {
                Emptic::Empty(x) => x.$func_name(),
                Emptic::Some(x) => x.$func_name(),
            }
        }
    )
}

impl<C, G> HasEnvelope<C> for Emptic<C, G>
where
    C: Coordinate,
    G: Geometric<C>,
{
    delegate_accessor!(envelope, Envelope<C>);
}

impl<C, G> Geometric<C> for Emptic<C, G>
where
    C: Coordinate,
    G: Geometric<C>,
{
    delegate_accessor!(dimension, u8);
    delegate_accessor!(geometry_type, &'static str);
    delegate_accessor!(is_empty, bool);
    delegate_accessor!(boundary, dyn Geometric<C>);
}
