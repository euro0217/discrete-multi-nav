use std::collections::VecDeque;

use crate::pathfind::common::{Cost, MultipleEnds, Node};

pub struct AgentData<N: Node, C: Cost, T = ()>
{
    kind: T,
    current: N,
    state: AgentState<N, C>,
    destinations: VecDeque<MultipleEnds<N, C>>,
    removing: bool,
}

impl<T: Default, N: Node, C: Cost> AgentData<N, C, T> {
    pub fn new_default(current: N, destinations: VecDeque<MultipleEnds<N, C>>) -> Self {
        Self { kind: T::default(), current, state: AgentState::NotPlaced, destinations, removing: false }
    }
}

impl<T, N: Node, C: Cost> AgentData<N, C, T> {
    pub fn new(kind: T, current: N, destinations: VecDeque<MultipleEnds<N, C>>) -> Self {
        Self { kind, current, state: AgentState::NotPlaced, destinations, removing: false }
    }

    pub fn kind(&self) -> &T { &self.kind }
    pub fn current(&self) -> &N { &self.current }
    pub fn state(&self) -> &AgentState<N, C> { &self.state }
    pub fn next_destinations(&self) -> Option<&MultipleEnds<N, C>> { self.destinations.get(0) }
    pub fn all_destinations(&self) -> &VecDeque<MultipleEnds<N, C>> { &self.destinations }
    pub fn destinations_mut(&mut self) -> &mut VecDeque<MultipleEnds<N, C>> { &mut self.destinations }
    pub fn removing(&self) -> bool { self.removing }

    pub(crate) fn remove(&mut self) -> bool {
        if self.removing {
            return false
        }
        self.removing = true;
        return true
    }
    
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

                if self.destinations[0].end_index(&self.current).is_some() {
                    self.destinations.pop_front();
                }
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
