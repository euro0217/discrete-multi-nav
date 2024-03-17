use std::hash::Hash;

use num_traits::Zero;
use pathfinding::directed::dijkstra::dijkstra;

use super::common::{Cost, MultipleEnds, Node, NodeCost, Path};

pub fn dijkstra_for_multiple_ends<N, C, FN, IN>(start: &N, ends: &MultipleEnds<N>, mut successors: FN)
-> Option<Path<N, C>> where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy + Hash,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    if ends.is_empty() { return None }

    let successors = |n: &NodeCost<Node<N>, C>| {
        let (node, c0) = (n.node(), n.cost());
        match node {
            Node::Node(n) => successors(n)
                .into_iter()
                .map(move |(m, c)| (NodeCost::new(Node::Node(m), c0 + c), Cost::new(0, c)))
                .chain(
                    ends.end_index(&n)
                        .and_then(|i| Some(vec![(NodeCost::new(Node::Dest, C::zero()), Cost::new(i, C::zero()))]))
                        .unwrap_or_default()
                ),
            Node::Dest => panic!(),
        }
    };

    dijkstra(&NodeCost::new(Node::Node(start.clone()), C::zero()), successors, |n| n.node() == &Node::Dest)
        .and_then(|(path, _)| Some(Path::new(path)))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::pathfind::common::MultipleEnds;

    use super::dijkstra_for_multiple_ends;

    #[test]
    fn test0() {
        let ends = MultipleEnds::new(&vec![]);
        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)].into_iter().map(|p| (p, 1));
        
        assert!(dijkstra_for_multiple_ends(&(2, 3), &ends, successors).is_none());
    }

    #[test]
    fn test1() {
        let ends = MultipleEnds::new(&vec![HashSet::from([(5, 1)])]);
        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)].into_iter().map(|p| (p, 1));
        
        let path = dijkstra_for_multiple_ends(&(2, 3), &ends, successors).unwrap();

        assert_eq!(path.total_cost(), 5);
        assert_eq!(path.len(), 6);
        assert_eq!(path[0], ((2, 3), 0));
        assert_eq!(path[path.len() - 1], ((5, 1), 5));
        assert_eq!(path.iter().map(|(_, c)| *c).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test2() {
        let ends = MultipleEnds::new(&vec![HashSet::from([(6, -1)]), HashSet::from([(5, 1)])]);
        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)].into_iter().map(|p| (p, 1));
        
        let path = dijkstra_for_multiple_ends(&(2, 3), &ends, successors).unwrap();

        assert_eq!(path.total_cost(), 8);
        assert_eq!(path.len(), 9);
        assert_eq!(path[0], ((2, 3), 0));
        assert_eq!(path[path.len() - 1], ((6, -1), 8));
    }

    #[test]
    fn test3() {
        let ends = MultipleEnds::new(&vec![HashSet::from([(6, -1), (-1, 7)]), HashSet::from([(5, 1)])]);
        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)].into_iter().map(|p| (p, 1));
        
        let path = dijkstra_for_multiple_ends(&(2, 3), &ends, successors).unwrap();

        assert_eq!(path.total_cost(), 7);
        assert_eq!(path.len(), 8);
        assert_eq!(path[0], ((2, 3), 0));
        assert_eq!(path[path.len() - 1], ((-1, 7), 7));
    }

    #[test]
    fn test4() {
        let ends = MultipleEnds::new(&vec![HashSet::from([(13, 10)])]);
        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
            .into_iter()
            .filter_map(|(x, y): (i32, i32)| if x.abs() < 10 && y.abs() < 10 { Some(((x, y), 1)) } else { None });
        
        assert!(dijkstra_for_multiple_ends(&(2, 3), &ends, successors).is_none());
    }

    #[test]
    fn test5() {
        let ends = MultipleEnds::new(&vec![HashSet::from([(13, 10)]), HashSet::from([(6, -1)])]);
        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
            .into_iter()
            .filter_map(|(x, y): (i32, i32)| if x.abs() < 10 && y.abs() < 10 { Some(((x, y), 1)) } else { None });
        
        let path = dijkstra_for_multiple_ends(&(2, 3), &ends, successors).unwrap();

        assert_eq!(path.total_cost(), 8);
        assert_eq!(path.len(), 9);
        assert_eq!(path[0], ((2, 3), 0));
        assert_eq!(path[path.len() - 1], ((6, -1), 8));
    }
}
