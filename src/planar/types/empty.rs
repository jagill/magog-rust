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

impl<C: Coordinate> Empty<C> {
    pub fn new() -> Self {
        Empty {
            phantom: PhantomData,
        }
    }

    pub fn dimension(&self) -> u8 {
        0
    }

    pub fn geometry_type(&self) -> &'static str {
        "Empty"
    }

    pub fn is_empty(&self) -> bool {
        true
    }

    pub fn is_simple(&self) -> bool {
        true
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        Ok(())
    }

    pub fn boundary(&self) -> Geometry<C> {
        Geometry::empty()
    }
}
