use std::cmp::Ordering;

use num_traits::{bounds::UpperBounded, Unsigned};

use crate::{index::index::Idx, pathfind::common::{Cost, Seat}};

#[derive(Debug)]
pub struct Duration<C: Cost, S: Seat, T, U: Unsigned + Copy + UpperBounded> {
    time: C,
    index: Idx<T, U>,
    seat: S,
}

impl<C: Cost, S: Seat, T, U: Unsigned + Copy + UpperBounded> Duration<C, S, T, U> {
    pub fn new(time: C, index: Idx<T, U>, seat: S) -> Self {
        Self { time, index, seat }
    }

    pub fn time(&self) -> C { self.time }
    pub fn index(&self) -> Idx<T, U> { self.index }
    pub fn seat(self) -> S { self.seat }
}

impl<C: Cost, S: Seat, T, U: Unsigned + Copy + UpperBounded> PartialEq for Duration<C, S, T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl<C: Cost, S: Seat, T, U: Unsigned + Copy + UpperBounded> Eq for Duration<C, S, T, U> {}

impl<C: Cost, S: Seat, T, U: Unsigned + Copy + UpperBounded> PartialOrd for Duration<C, S, T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.time.partial_cmp(&self.time)
    }
}

impl<C: Cost, S: Seat, T, U: Unsigned + Copy + UpperBounded> Ord for Duration<C, S, T, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}
