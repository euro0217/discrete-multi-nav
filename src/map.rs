use std::{hash::Hash, ops::IndexMut};

use num_traits::One;

use crate::{pathfind::common::{Cost, Node, Seat as TSeat}, seat::{AgentIdxType, Seat}};


pub trait Map<U: AgentIdxType, T = ()>: IndexMut<Self::SI, Output = Self::Seat> {
    type SI: TSeat;
    type Seat: Seat<T, U>;
    type Node: Node;
    type C: Cost + One;
    type I: Eq + Hash + Clone + Default;
    type SIter: Iterator<Item = Self::SI>;
    type SCIter: Iterator<Item = (Self::I, Self::Node, Self::C)>;
    type SBIter: Iterator<Item = (Self::SI, Self::C)>;

    fn seats(&self, n: &Self::Node, t: &T) -> Self::SIter;
    fn successors(&self, n: &Self::Node, t: &T) -> Self::SCIter;
    fn successor(&self, n: &Self::Node, t: &T, i: &Self::I) -> Self::Node;
    fn seats_between(&self, n: &Self::Node, t: &T, i: &Self::I) -> Self::SBIter;

    fn suc(&self, n: &Self::Node, t: &T, i: &Self::I) -> Successor<Self, U, T> where Self: Sized {
        let node = self.successor(n, t, i);
        let seats_between = self.seats_between(n, t, i)
            .map(|(s, c)| (s, Some(c)));
        let seats = self.seats(&node, t)
            .map(|s| (s, None));
        Successor { node, seats: seats_between.chain(seats).collect::<Vec<_>>() }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Successor<M: Map<U, T>, U: AgentIdxType, T> {
    node: M::Node,
    seats: Vec<(M::SI, Option<M::C>)>
}

impl<M: Map<U, T>, U: AgentIdxType, T> Successor<M, U, T> {
    pub fn node(&self) -> &M::Node { &self.node }
    pub fn seats(&self) -> &Vec<(M::SI, Option<M::C>)> { &self.seats }
}
