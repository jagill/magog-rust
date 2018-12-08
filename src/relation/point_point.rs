use crate::types::CoordinateType;
// use crate::types::Envelope;
use crate::relation::Relation;
use crate::relation::Relation::*;
use crate::types::Geometry;
use crate::types::MultiPoint;
use crate::types::Point;

pub fn check_relation_point_point<T>(p1: &Point<T>, p2: &Point<T>, relation: Relation) -> bool
where
    T: CoordinateType,
{
    match relation {
        Contains | Intersects | Within => p1 == p2,
        _ => false,
    }
}

pub fn check_relation_multi_point_point<T>(
    mp1: &MultiPoint<T>,
    p2: &Point<T>,
    relation: Relation,
) -> bool
where
    T: CoordinateType,
{
    let coord = p2.0;
    match relation {
        Touches | Intersects => {
            if !mp1.envelope().contains(coord) {
                return false;
            }
            mp1.points.iter().any(|p| p.0 == coord)
        }
        _ => false,
    }
}
