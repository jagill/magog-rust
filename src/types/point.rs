use {Coordinate, CoordinateType};

#[derive(Debug, PartialEq)]
pub struct Point<T>(pub Coordinate<T>)
where
    T: CoordinateType;

/// Turn a `Coordinate`-ish object into a `Point`.
impl<T: CoordinateType, IC: Into<Coordinate<T>>> From<IC> for Point<T> {
    fn from(c: IC) -> Self {
        Point(c.into())
    }
}

impl<T: CoordinateType> Point<T> {
    pub fn new(coord: Coordinate<T>) -> Result<Point<T>, &'static str> {
        let p = Point(coord);
        p.validate()?;
        Ok(p)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        self.0.validate()?;
        Ok(())
    }

    pub fn x(&self) -> T {
        self.0.x
    }

    pub fn y(&self) -> T {
        self.0.y
    }

    pub fn area(&self) -> T {
        T::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_constructor() {
        let p = Point(Coordinate { x: 0.1, y: 1.0 });
        assert_eq!(p.x(), 0.1);
        assert_eq!(p.y(), 1.0);
    }

}
