use std::{collections::{vec_deque, VecDeque}, ops::{Index, IndexMut}, vec::IntoIter};

use discrete_multi_nav::{index::index::Idx, map::Map, seat::Seat};

use crate::test_node::TestNode;

pub(crate) struct TestMap {
    nodes: Vec<TestNode>
}

impl TestMap {
    pub(crate) fn new(nodes: Vec<TestNode>) -> Self {
        for n in &nodes {
            for &i in n.nexts() {
                if i >= nodes.len() {
                    panic!("index out of bound: {}", i)
                }
            }
        }
        Self { nodes }
    }

    pub fn nodes(&self) -> &Vec<TestNode> { &self.nodes }
}

impl Map<u32> for TestMap {
    type C = u32;
    type I = usize;
    type SI = usize;
    type Seat = TestNode;
    type Node = VecDeque<usize>;

    type SIter = vec_deque::IntoIter<Self::SI>;
    type SCIter = IntoIter<(Self::I, Self::Node, Self::C)>;
    type SBIter = IntoIter<(Self::SI, Self::C)>;

    fn seats(&self, idxs: &Self::Node, _: &()) -> Self::SIter {
        idxs.clone().into_iter()
    }

    fn successors(&self, idxs: &Self::Node, _: &()) -> Self::SCIter {
        self.nodes[idxs[0]]
            .nexts()
            .iter()
            .enumerate()
            .map(|(k, &j)| {
                let mut js = idxs.clone();
                js.push_front(j);
                js.pop_back();
                (k, js, 1)
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn successor(&self, idxs: &Self::Node, _: &(), &j: &Self::I) -> Self::Node {
        let j = self.nodes[idxs[0]].nexts()[j];
        let mut js = idxs.clone();
        js.push_front(j);
        js.pop_back();
        js
    }

    fn seats_between(&self, _: &Self::Node, _: &(), &_: &Self::I) -> Self::SBIter {
        vec![].into_iter()
    }
}

impl Index<usize> for TestMap {
    type Output = TestNode;

    fn index(&self, i: usize) -> &Self::Output {
        &self.nodes[i]
    }
}

impl IndexMut<usize> for TestMap {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.nodes[i]
    }
}

impl Seat<(), u32> for TestNode {
    fn is_empty_for(&self, i: Idx<(), u32>) -> bool {
        if let Some(j) = self.occupied() {
            i == j
        } else {
            true
        }
    }

    fn add(&mut self, idx: Idx<(), u32>) {
        self.set_occupied(Some(idx))
    }

    fn remove(&mut self, _: Idx<(), u32>) {
        self.set_occupied(None)
    }
}
