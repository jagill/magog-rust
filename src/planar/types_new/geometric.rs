use crate::planar::primitives::HasEnvelope;
use crate::Coordinate;

pub trait Geometric<C: Coordinate>: HasEnvelope<C> {
    fn dimension(&self) -> u8;

    fn geometry_type(&self) -> &'static str;

    fn is_empty(&self) -> bool;

    fn boundary(&self) -> dyn Geometric<C>;
}
