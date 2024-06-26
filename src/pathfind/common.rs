use std::{collections::HashMap, hash::{Hash, Hasher}, ops::{Add, Index}, slice::Iter, vec::IntoIter};

use num_traits::Zero;
use trait_set::trait_set;

trait_set! {
    pub trait Node = Eq + Hash + Clone;
    pub trait Cost = Zero + Ord + Copy + Hash;
    pub trait Seat = Eq + Clone;
}

pub struct MultipleEnds<N: Node, C: Cost> {
    ends: HashMap<N, C>,
}

impl<N: Node, C: Cost> MultipleEnds<N, C> {
    pub fn new(ends: HashMap<N, C>) -> Self {
        Self { ends }
    }
    pub fn new_as_all_zero(ends: Vec<N>) -> Self {
        Self { ends: ends.into_iter().map(|n| (n, C::zero())).collect::<HashMap<_, _>>() }
    }

    pub fn is_empty(&self) -> bool { self.ends.is_empty() }
    pub fn end_index(&self, node: &N) -> Option<C> { self.ends.get(node).copied() }
    pub fn ends(&self) -> &HashMap<N, C> { &self.ends }
}

#[derive(Debug)]
pub struct Path<N: Node, C: Cost, T = ()> {
    nodes: Vec<(N, C, T)>,
}

impl<N: Node, C: Cost, T> Path<N, C, T> {
    pub(crate) fn new(nodes: Vec<(N, C, T)>) -> Self {
        Self { nodes }
    }

    pub fn node(&self, index: usize) -> &(N, C, T) { &self.nodes[index] }
    pub fn total_cost(&self) -> C { self.nodes[self.nodes.len() - 1].1 }
    pub fn len(&self) -> usize { self.nodes.len() }
    pub fn iter(&self) -> Iter<'_, (N, C, T)> { self.nodes.iter() }
    pub fn into_iter(self) -> IntoIter<(N, C, T)> { self.nodes.into_iter() }
}

impl<N: Node, C: Cost, T> Index<usize> for Path<N, C, T> {
    type Output = (N, C, T);
    fn index(&self, index: usize) -> &Self::Output { self.node(index) }
}

#[derive(Debug, Clone)]
pub(crate) struct NodeCost<N: Node, C: Cost, T: Clone = ()> {
    node: N,
    cost: C,
    attr: T,
}

impl<N: Node, C: Cost, T: Clone> NodeCost<N, C, T> {
    pub fn new(node: N, cost: C, attr: T) -> Self {
        Self { node, cost, attr }
    }
    pub fn node(&self) -> &N { &self.node }
    pub fn cost(&self) -> C { self.cost }
    pub fn attr(&self) -> &T { &self.attr }
}

impl<N: Node, C: Cost, T: Clone> Eq for NodeCost<N, C, T> {}

impl<N: Node, C: Cost, T: Clone> PartialEq for NodeCost<N, C, T> {
    fn eq(&self, other: &Self) -> bool { self.node == other.node }
}

impl<N: Node, C: Cost, T: Clone> Hash for NodeCost<N, C, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) enum NodeDest<N: Node> { Node(N), Dest }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RCost<C: Cost> {
    Cost { cost: C, r: C, blocked: bool },
    Add { dc: C, max: C },
    AddBlocked { dc: C },
}

impl<C: Cost> Zero for RCost<C> {
    fn zero() -> Self { Self::Cost { cost: C::zero(), r: C::zero(), blocked: false }}
    fn is_zero(&self) -> bool {
        if let Self::Cost { cost, r, blocked } = self {
            cost.is_zero() && r.is_zero() && !blocked
        } else {
            false
        }
    }
}

impl<C: Cost> Add for RCost<C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            RCost::Cost { cost, r, blocked } => {
                match rhs {
                    RCost::Add { dc, max } => {
                        if blocked || r + dc > max {
                            RCost::Cost { cost: cost + dc, r, blocked: true }
                        } else {
                            RCost::Cost { cost, r: r + dc, blocked: false }
                        }
                    },
                    RCost::AddBlocked { dc } => {
                        RCost::Cost { cost: cost + dc, r, blocked: true }
                    },
                    RCost::Cost { cost: cost2, r: r2, blocked: blocked2 } => {
                        RCost::Cost { cost: cost + cost2, r: r + r2, blocked: blocked || blocked2 }
                    },
                }
            },
            _ => panic!()
        }
    }
}

pub(crate) fn collect_path<N: Node, C: Cost, T: Clone>(path: Vec<NodeCost<NodeDest<N>, C, T>>) -> Path<N, C, T> {
    Path::new(path[1..]
        .into_iter()
        .filter_map(|node|
            match node.node() {
                NodeDest::Node(n) => Some((n.clone(), node.cost(), node.attr().clone())),
                NodeDest::Dest => None,
            }
        )
        .collect::<Vec<(N, C, T)>>()
    )
}

pub(crate) fn collect_path_for_reservation<N: Node, C: Cost, T: Clone>(path: Path<N, RCost<C>, T>) -> Path<N, C, T> {
    Path::new(
        path.into_iter()
            .take_while(|(_, c, _)| {
                if let RCost::Cost { cost: _, r: _, blocked } = c {
                    !blocked
                } else {
                    false
                }
            })
            .map(|(n, c, t)| {
                if let RCost::Cost { cost, r, blocked: _ } = c {
                    (n, cost + r, t)
                } else {
                    panic!()
                }
            })
            .collect::<Vec<(N, C, T)>>()
    )
}