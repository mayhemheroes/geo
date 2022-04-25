use super::Contains;
use crate::*;

// ┌────────────────────────────────┐
// │ Implementations for Point      │
// └────────────────────────────────┘

impl<T> Contains<Coord<T>> for Point<T>
where
    T: CoordNum,
{
    fn contains(&self, coord: &Coord<T>) -> bool {
        &self.0 == coord
    }
}

impl<T> Contains<Point<T>> for Point<T>
where
    T: CoordNum,
{
    fn contains(&self, p: &Point<T>) -> bool {
        self.contains(&p.0)
    }
}

// ┌────────────────────────────────┐
// │ Implementations for MultiPoint │
// └────────────────────────────────┘
impl<G, T> Contains<G> for MultiPoint<T>
where
    T: CoordNum,
    Point<T>: Contains<G>,
{
    fn contains(&self, rhs: &G) -> bool {
        self.iter().any(|p| p.contains(rhs))
    }
}
