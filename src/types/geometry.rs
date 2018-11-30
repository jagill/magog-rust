use {CoordinateType, Envelope};

pub trait Geometry<T>
where T: CoordinateType,
{
    // Basic accessors
    fn dimension(&self) -> u8;
    fn geometry_type(&self) -> &'static str;
    fn envelope(&self) -> Envelope<T>;
    fn is_empty(&self) -> bool;
    fn is_simple(&self) -> bool;
    fn boundary(&self) -> Option<Box<Geometry<T>>>;
    // Intersection Relations
    // fn equals(&self, other: &Geometry<T>) -> bool;
    // fn disjoint(&self, other: &Geometry<T>) -> bool;
    // fn intersects(&self, other: &Geometry<T>) -> bool;
    // fn touches(&self, other: &Geometry<T>) -> bool;
    // fn crosses(&self, other: &Geometry<T>) -> bool;
    // fn within(&self, other: &Geometry<T>) -> bool;
    // fn contains(&self, other: &Geometry<T>) -> bool;
    // fn overlaps(&self, other: &Geometry<T>) -> bool;
}
