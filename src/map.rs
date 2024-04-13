use std::{fmt::Debug, hash::Hash, ops::IndexMut};

use num_traits::{bounds::UpperBounded, One, Unsigned, Zero};

use crate::seat::Seat;


pub trait Map<U: Copy + Unsigned + UpperBounded, T = ()>: IndexMut<Self::SI, Output = Self::Seat> {
    type SI: Eq + Clone;
    type Seat: Seat<T, U>;
    type Node: Eq + Hash + Clone;
    type C: Zero + One + Ord + Copy + Hash + Debug;
    type I: Eq + Hash + Clone + Default;
    type SIter: Iterator<Item = Self::SI>;
    type SCIter: Iterator<Item = (Self::I, Self::Node, Self::C)>;
    type SBIter: Iterator<Item = (Self::SI, Self::C)>;

    fn seats(&self, n: &Self::Node, t: &T) -> Self::SIter;
    fn successors(&self, n: &Self::Node, t: &T) -> Self::SCIter;
    fn successor(&self, n: &Self::Node, t: &T, i: &Self::I) -> Self::Node;
    fn seats_between(&self, n: &Self::Node, t: &T, i: &Self::I) -> Self::SBIter;
}
