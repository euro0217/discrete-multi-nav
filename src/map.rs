use std::{fmt::Debug, hash::Hash, ops::IndexMut};

use num_traits::One;

use crate::{pathfind::common::{Cost, Node, Seat as TSeat}, seat::{AgentIdxType, Seat}};


pub trait Map<U: AgentIdxType, T = ()>: IndexMut<Self::SI, Output = Self::Seat> {
    type SI: TSeat;
    type Seat: Seat<T, U>;
    type Node: Node;
    type C: Cost + One + Debug;
    type I: Eq + Hash + Clone + Default;
    type SIter: Iterator<Item = Self::SI>;
    type SCIter: Iterator<Item = (Self::I, Self::Node, Self::C)>;
    type SBIter: Iterator<Item = (Self::SI, Self::C)>;

    fn seats(&self, n: &Self::Node, t: &T) -> Self::SIter;
    fn successors(&self, n: &Self::Node, t: &T) -> Self::SCIter;
    fn successor(&self, n: &Self::Node, t: &T, i: &Self::I) -> Self::Node;
    fn seats_between(&self, n: &Self::Node, t: &T, i: &Self::I) -> Self::SBIter;
}
