use std::{hash::Hash, ops::IndexMut};

use num_traits::One;

use crate::{pathfind::common::{Cost, MultipleEnds, Node, Seat as TSeat}, seat::{AgentIdxType, Seat}};


pub trait Map<U: AgentIdxType, T = ()>: IndexMut<Self::SeatIndex, Output = Self::Seat> {
    type SeatIndex: TSeat;
    type Seat: Seat<T, U>;
    type Node: Node;
    type Cost: Cost + One;
    type I: Eq + Hash + Clone + Default;
    type SIter: Iterator<Item = Self::SeatIndex>;
    type SCIter: Iterator<Item = (Self::I, Self::Node, Self::Cost)>;
    type SBIter: Iterator<Item = (Self::SeatIndex, Self::Cost)>;
    type FH: Heuristic<Self::Node, Self::Cost>;

    fn seats(&self, n: &Self::Node, t: &T) -> Self::SIter;
    fn successors(&self, n: &Self::Node, t: &T) -> Self::SCIter;
    fn successor(&self, n: &Self::Node, t: &T, i: &Self::I) -> Option<Self::Node>;
    fn seats_between(&self, n: &Self::Node, t: &T, i: &Self::I) -> Self::SBIter;

    fn heuristic(&self, _dest: &MultipleEnds<Self::Node, Self::Cost>) -> Option<Self::FH> { None }

    fn movement(&self, n: &Self::Node, t: &T, i: &Self::I) -> Option<Movement<Self, U, T>> where Self: Sized {
        let node = self.successor(n, t, i)?;
        let seats_between = self.seats_between(n, t, i)
            .map(|(s, c)| (s, Some(c)));
        let seats = self.seats(&node, t)
            .map(|s| (s, None));
        Some(Movement { node, seats: seats_between.chain(seats).collect::<Vec<_>>() })
    }
}

pub trait Heuristic<N: Node, C: Cost> {
    fn heuristic(&self, n: &N) -> C;
}

pub struct DummyHeuristic {}

impl<N: Node, C: Cost> Heuristic<N, C> for DummyHeuristic {
    fn heuristic(&self, _: &N) -> C { C::zero() }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Movement<M: Map<U, T>, U: AgentIdxType, T> {
    node: M::Node,
    seats: Vec<(M::SeatIndex, Option<M::Cost>)>
}

impl<M: Map<U, T>, U: AgentIdxType, T> Movement<M, U, T> {
    pub fn node(&self) -> &M::Node { &self.node }
    pub fn seats(&self) -> &Vec<(M::SeatIndex, Option<M::Cost>)> { &self.seats }
}
