use std::hash::Hash;

use discrete_multi_nav::index::index::Idx;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct TestNode {
    x: u32, // only for display
    y: u32, // only for display
    nexts: Vec<(usize, u32)>, // (index, cost)
    occupied: Option<Idx<(), u32>>,
}

impl TestNode {
    pub(crate) fn new(x: u32, y: u32, nexts: Vec<(usize, u32)>) -> Self {
        Self { x, y, nexts, occupied: None }
    }
    pub(crate) fn x(&self) -> u32 { self.x }
    pub(crate) fn y(&self) -> u32 { self.y }
    pub(crate) fn nexts(&self) -> &Vec<(usize, u32)> { &self.nexts }
    pub(crate) fn occupied(&self) -> Option<Idx<(), u32>> { self.occupied }
    pub(crate) fn set_occupied(&mut self, idx: Option<Idx<(), u32>>) {
        self.occupied = idx;
    }
}
