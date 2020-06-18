use super::Geometric;

use crate::planar::primitives::{Envelope, HasEnvelope};
use crate::planar::types::Geometry;
use crate::Coordinate;
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct Empty<C: Coordinate> {
    phantom: PhantomData<C>,
}

impl<C: Coordinate> HasEnvelope<C> for Empty<C> {
    fn envelope(&self) -> Envelope<C> {
        Envelope::empty()
    }
}

impl<C: Coordinate> Geometric<C> for Empty<C> {
    fn dimension(&self) -> u8 {
        0
    }

    fn geometry_type(&self) -> &'static str {
        "Empty"
    }

    fn is_empty(&self) -> bool {
        true
    }
}

impl<C: Coordinate> Empty<C> {
    pub fn new() -> Self {
        Empty {
            phantom: PhantomData,
        }
    }

    pub fn boundary(&self) -> Geometry<C> {
        Geometry::empty()
    }
}
