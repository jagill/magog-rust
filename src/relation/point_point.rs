use crate::types::Coordinate;
// use crate::types::Envelope;
use crate::relation::Relation;
use crate::relation::Relation::*;
use crate::types::Geometry;
use crate::types::MultiPoint;
use crate::types::Point;

pub fn check_relation_point_point<C: Coordinate>(p1: &Point<C>, p2: &Point<C>, relation: Relation) -> bool
{
    match relation {
        Contains | Intersects | Within => p1 == p2,
        _ => false,
    }
}

pub fn check_relation_multi_point_point<C: Coordinate>(
    mp1: &MultiPoint<C>,
    p2: &Point<C>,
    relation: Relation,
) -> bool
{
    let position = p2.0;
    match relation {
        Touches | Intersects => {
            if !mp1.envelope().contains(position) {
                return false;
            }
            mp1.points.iter().any(|p| p.0 == position)
        }
        _ => false,
    }
}
