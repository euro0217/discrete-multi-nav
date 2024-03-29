use std::{collections::VecDeque, hash::Hash};

use num_traits::Zero;

use crate::pathfind::common::MultipleEnds;

pub struct AgentData<N: Eq + Hash + Clone, C: Zero + Ord + Copy + Hash, T = ()>
{
    kind: T,
    current: N,
    state: AgentState<N, C>,
    destinations: MultipleEnds<N>,
}

impl<T: Default, N: Eq + Hash + Clone, C: Zero + Ord + Copy + Hash> AgentData<N, C, T> {
    pub fn new_default(current: N, destinations: MultipleEnds<N>) -> Self {
        Self { kind: T::default(), current, state: AgentState::NotPlaced, destinations }
    }
}

impl<T, N: Eq + Hash + Clone, C: Zero + Ord + Copy + Hash> AgentData<N, C, T> {
    pub fn new(kind: T, current: N, destinations: MultipleEnds<N>) -> Self {
        Self { kind, current, state: AgentState::NotPlaced, destinations }
    }

    // pub fn agent(&self) -> &A { &self.agent }
    pub fn kind(&self) -> &T { &self.kind }
    pub fn current(&self) -> &N { &self.current }
    pub fn state(&self) -> &AgentState<N, C> { &self.state }
    pub fn destinations(&self) -> &MultipleEnds<N> { &self.destinations }
    
    pub(crate) fn place(&mut self) {
        self.state = AgentState::Stop;
    }
    pub(crate) fn departs<I: Iterator<Item = (N, C)>>(&mut self, nexts: I) {
        let nexts = VecDeque::from_iter(nexts);
        if nexts.is_empty() {
            return
        }
        self.state = AgentState::Moving { nexts }
    }
    pub(crate) fn arrives(&mut self) {
        if let AgentState::Moving { nexts } = &mut self.state {
            if let Some((n, _)) = nexts.pop_front() {
                self.current = n;
            }
            if nexts.is_empty() {
                self.state = AgentState::Stop;
            }
        }
    }
    
}

#[derive(Debug, PartialEq, Eq)]
pub enum AgentState<N, C> {
    NotPlaced,
    Stop,
    Moving { nexts: VecDeque<(N, C)> }
}
