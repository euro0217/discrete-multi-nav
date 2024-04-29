use pathfinding::directed::dijkstra::dijkstra;

use crate::pathfind::common::RCost;

use super::common::{Cost, MultipleEnds, Node, NodeCost, NodeDest, Path, Seat};

pub fn dijkstra_for_next_reservation<N, C, S, FN, IN, IS, FS, T>(
    start: N,
    ends: &MultipleEnds<N, C>,
    mut successors: FN,
    seats_reservation: FS,
    max_reservation_cost: C
)
-> Option<Path<N, C, T>> where
    N: Node ,
    C: Cost,
    S: Seat,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C, IS, T)>,
    IS: Iterator<Item = S>,
    FS: Fn(&S) -> bool,
    T: Default + Clone,
{
    if ends.is_empty() { return None }

    let successors = |n: &N| {
        successors(n)
            .into_iter()
            .map(|(m, dc, ss, t)| {
                if ss.into_iter().all(|s| {
                    seats_reservation(&s)
                }) {
                    (m, RCost::Add { dc, max: max_reservation_cost }, t)
                } else {
                    (m, RCost::AddBlocked { dc }, t)
                }
            })
    };

    let p: Option<Path<N, RCost<C>, T>> = dijkstra_for_multiple_ends(&start, ends, successors, |c| RCost::Add { dc: c, max: max_reservation_cost });
    p.and_then(|path|
        Some(Path::new(
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
        ))
    )
}

pub fn dijkstra_for_multiple_ends<N, C, MC, FN, IN, T, FC>(start: &N, ends: &MultipleEnds<N, MC>, mut successors: FN, converter: FC)
-> Option<Path<N, C, T>> where
    N: Node,
    C: Cost,
    MC: Cost,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C, T)>,
    FC: Fn(MC) -> C,
    T: Default + Clone,
{
    if ends.is_empty() { return None }

    let successors = |n: &NodeCost<NodeDest<N>, C, T>| {
        let (node, c0) = (n.node(), n.cost());
        match node {
            NodeDest::Node(n) => successors(n)
                .into_iter()
                .map(move |(m, c, t)| (NodeCost::new(NodeDest::Node(m), c0 + c, t), c))
                .chain(
                    ends.end_index(&n)
                        .and_then(|i| Some(vec![(NodeCost::new(NodeDest::Dest, C::zero(), T::default()), converter(i))]))
                        .unwrap_or_default()
                ),
            NodeDest::Dest => panic!(),
        }
    };

    dijkstra(&NodeCost::new(NodeDest::Node(start.clone()), C::zero(), T::default()), successors, |n| n.node() == &NodeDest::Dest)
        .and_then(|(path, _)|
            Some(Path::new(path[1..]
                .into_iter()
                .filter_map(|node|
                    match node.node() {
                        NodeDest::Node(n) => Some((n.clone(), node.cost(), node.attr().clone())),
                        NodeDest::Dest => None,
                    }
                )
                .collect::<Vec<(N, C, T)>>()
        )))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::pathfind::{common::MultipleEnds, dijkstra::dijkstra_for_next_reservation};

    use super::dijkstra_for_multiple_ends;

    #[test]
    fn multiple_ends_test0() {
        let ends = MultipleEnds::new_as_all_zero(vec![]);
        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)].into_iter().map(|p| (p, 1, ()));
        
        assert!(dijkstra_for_multiple_ends(&(2, 3), &ends, successors, |c| c).is_none());
    }

    #[test]
    fn multiple_ends_test1() {
        let ends = MultipleEnds::new_as_all_zero(vec![(5, 1)]);
        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)].into_iter().map(|p| (p, 1, ()));
        
        let path = dijkstra_for_multiple_ends(&(2, 3), &ends, successors, |c| c).unwrap();

        assert_eq!(path.total_cost(), 5);
        assert_eq!(path.len(), 5);
        assert_eq!(path[path.len() - 1], ((5, 1), 5, ()));
        assert_eq!(path.iter().map(|(_, c, _)| *c).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn multiple_ends_test2() {
        let ends = MultipleEnds::new(HashMap::from([((6, -1), 0), ((5, 1), 1000)]));
        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)].into_iter().map(|p| (p, 1, ()));
        
        let path = dijkstra_for_multiple_ends(&(2i32, 3i32), &ends, successors, |c| c).unwrap();

        assert_eq!(path.total_cost(), 8);
        assert_eq!(path.len(), 8);
        assert!((path[0].0.0 - 2).abs() <= 1);
        assert!((path[0].0.1 - 3).abs() <= 1);
        assert_eq!(path[path.len() - 1], ((6, -1), 8, ()));
    }

    #[test]
    fn multiple_ends_test3() {
        let ends = MultipleEnds::new(HashMap::from([((6, -1), 0), ((-1, 7), 0), ((5, 1), 1000)]));
        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)].into_iter().map(|p| (p, 1, ()));
        
        let path = dijkstra_for_multiple_ends(&(2, 3), &ends, successors, |c| c).unwrap();

        assert_eq!(path.total_cost(), 7);
        assert_eq!(path.len(), 7);
        assert_eq!(path[path.len() - 1], ((-1, 7), 7, ()));
    }

    #[test]
    fn multiple_ends_test4() {
        let ends = MultipleEnds::new_as_all_zero(vec![(13, 10)]);
        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
            .into_iter()
            .filter_map(|(x, y): (i32, i32)| if x.abs() < 10 && y.abs() < 10 { Some(((x, y), 1, ())) } else { None });
        
        assert!(dijkstra_for_multiple_ends(&(2, 3), &ends, successors, |c| c).is_none());
    }

    #[test]
    fn multiple_ends_test5() {
        let ends = MultipleEnds::new(HashMap::from([((13, 10), 0), ((6, -1), 1000)]));
        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
            .into_iter()
            .filter_map(|(x, y): (i32, i32)| if x.abs() < 10 && y.abs() < 10 { Some(((x, y), 1, ())) } else { None });
        
        let path = dijkstra_for_multiple_ends(&(2, 3), &ends, successors, |c| c).unwrap();

        assert_eq!(path.total_cost(), 8);
        assert_eq!(path.len(), 8);
        assert_eq!(path[path.len() - 1], ((6, -1), 8, ()));
    }

    #[test]
    fn multiple_ends_test6() {
        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
            .into_iter()
            .filter_map(|(x, y): (i32, i32)| if x.abs() < 10 && y.abs() < 10 { Some(((x, y), 1, ())) } else { None });

        let ends = MultipleEnds::new(HashMap::from([((6, 3), 1), ((2, 7), 0)])); // 4 6
        let path = dijkstra_for_multiple_ends(&(3, 2), &ends, successors, |c| c).unwrap();

        assert_eq!(path.total_cost(), 4);
        assert_eq!(path.len(), 4);
        assert_eq!(path[path.len() - 1], ((6, 3), 4, ()));

        let ends = MultipleEnds::new(HashMap::from([((6, 3), 3), ((2, 7), 0)])); // 4 6
        let path = dijkstra_for_multiple_ends(&(3, 2), &ends, successors, |c| c).unwrap();

        assert_eq!(path.total_cost(), 6);
        assert_eq!(path.len(), 6);
        assert_eq!(path[path.len() - 1], ((2, 7), 6, ()));
    }

    #[test]
    fn next_seats_test1() {
        let s3x3 = |x: i32, y: i32| [(x, y), (x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1), (x + 1, y + 1), (x - 1, y + 1), (x + 1, y + 1), (x - 1, y - 1)].into_iter();

        let successors = |&(x, y): &_| vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
            .into_iter()
            .filter_map(|(x, y): (i32, i32)| if 0 <= x && x < 10 && 0 <= y && y < 10 {
                Some(((x, y), 1, s3x3(x, y), ())) } else { None }
            );

        let seats_reservation = |&(x, y): &_| {
            match x {
                ..=2 => y < 4,
                ..=5 => y < 4 || y >= 6,
                _ => y >= 6,
            }
        };

        let cases = [
            ((1, 2), (4, 6), 2, Some(vec![(2, 2), (3, 2)])),
            ((1, 2), (4, 6), 3, Some(vec![(2, 2), (3, 2), (4, 2)])),
            ((1, 2), (4, 6), 4, Some(vec![(2, 2), (3, 2), (4, 2)])),
            ((4, 2), (4, 6), 3, Some(vec![])),
            ((4, 1), (5, 7), 3, Some(vec![(4, 2)])),
            ((4, 1), (5, 7), 0, Some(vec![])),
            ((8, 7), (1, 5), 2, Some(vec![(7, 7), (6, 7)])),
            ((8, 7), (11, 6), 2, None),
        ];

        //
        // 9 x x x
        // 8 x x x
        // 7 x x x   
        // 6 x x x   e
        // 5 x x x x x x x x x x
        // 4 x x x x x x x x x x
        // 3 + + +       x x x x
        // 2 + s - - -   x x x x
        // 1 + + +       x x x x
        // 0             x x x x
        //   0 1 2 3 4 5 6 7 8 9
        //
        // s: start, e: end (for first 3 cases)
        // x: cannot be entered


        for (start, ends, len, expected) in cases {
            let ends = MultipleEnds::new_as_all_zero(vec![ends]);
            let actual = dijkstra_for_next_reservation(start, &ends, successors, seats_reservation, len);

            match (expected, actual) {
                (None, None) => {},
                (None, Some(_)) => panic!("expected None, but Some"),
                (Some(_), None) => panic!("expected Some, but None"),
                (Some(e), Some(a)) => {
                    assert_eq!(e.len(), a.len());
                    for i in 0..e.len() {
                        assert_eq!((e[i], i + 1, ()), a[i]);
                    }
                },
            }
        }
    }
}
