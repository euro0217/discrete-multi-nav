use std::{cmp::Ordering, hash::Hash};

use num_traits::{bounds::UpperBounded, Unsigned, Zero};

use crate::index::index::Idx;


#[derive(Debug)]
pub struct Duration<C: Zero + Ord + Copy + Hash, S: Eq + Clone, T, U: Unsigned + Copy + UpperBounded> {
    time: C,
    index: Idx<T, U>,
    seat: S,
}

impl<C: Zero + Ord + Copy + Hash, S: Eq + Clone, T, U: Unsigned + Copy + UpperBounded> Duration<C, S, T, U> {
    pub fn new(time: C, index: Idx<T, U>, seat: S) -> Self {
        Self { time, index, seat }
    }

    pub fn time(&self) -> C { self.time }
    pub fn index(&self) -> Idx<T, U> { self.index }
    pub fn seat(self) -> S { self.seat }
}

impl<C: Zero + Ord + Copy + Hash, S: Eq + Clone, T, U: Unsigned + Copy + UpperBounded> PartialEq for Duration<C, S, T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl<C: Zero + Ord + Copy + Hash, S: Eq + Clone, T, U: Unsigned + Copy + UpperBounded> Eq for Duration<C, S, T, U> {}

impl<C: Zero + Ord + Copy + Hash, S: Eq + Clone, T, U: Unsigned + Copy + UpperBounded> PartialOrd for Duration<C, S, T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.time.partial_cmp(&self.time)
    }
}

impl<C: Zero + Ord + Copy + Hash, S: Eq + Clone, T, U: Unsigned + Copy + UpperBounded> Ord for Duration<C, S, T, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}
