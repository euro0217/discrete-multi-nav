use std::collections::{HashMap, VecDeque};

use discrete_multi_nav::{pathfind::common::MultipleEnds, simulator::Simulator};
use crate::{json_schema::{Data, Seat}, output_data, output_file, test_map::TestMap, test_node::TestNode};

fn testdata2(max_reservation_time: u32) -> ([(u32, u32, Vec<(usize, u32)>); 20], Simulator<TestMap, u32>, Vec<Seat>) {
    
    //      6 -> 7 -> * -> 8 -> * -> 17 -> * -> 18 -> * -> 19
    //                          |                         ^
    //                          |                  -> * -/
    //                          v                /
    //                         10 -> 14 -> * -> 15 -> * -> 16
    //                          |      \
    //                          ^       -> * -\
    //                          |              v
    // 3 -> * -> 4 -> * -> 5 -> 9 -> 11 -> * -> 12 -> * -> 13
    //                          ^
    // 0 -> * -> 1 -> * -> 2 -> *
    //
    let map = [
        (0, 0, vec![(1, 2)]), // 0
        (2, 0, vec![(2, 2)]),
        (4, 0, vec![(9, 2)]),
        (0, 1, vec![(4, 2)]),
        (2, 1, vec![(5, 2)]),
        (4, 1, vec![(9, 1)]), // 5
        (1, 3, vec![(7, 1)]),
        (2, 3, vec![(8, 2)]),
        (4, 3, vec![(10, 2), (17, 2)]),
        (5, 1, vec![(10, 1), (11, 1)]),
        (5, 2, vec![(14, 1)]), // 10
        (6, 1, vec![(12, 2)]),
        (8, 1, vec![(13, 2)]),
        (10, 1, vec![]),
        (6, 2, vec![(12, 2), (15, 2)]),
        (8, 2, vec![(16, 2), (19, 2)]), // 15
        (10, 2, vec![]),
        (6, 3, vec![(18, 2)]),
        (8, 3, vec![(19, 2)]),
        (10, 3, vec![]),
    ];
    
    let ns = map.iter().map(|(x, y, js)| TestNode::new(*x, *y, js.iter().map(|&j| j).collect::<Vec<_>>())).collect::<Vec<_>>();
    let seats = ns.iter()
        .map(|n| Seat{ x: n.x() as i32, y: n.y() as i32, nexts: n.nexts().clone(), agent: vec![] })
        .collect::<Vec<_>>();

    let m = TestMap::new(ns);
    let s = Simulator::new(0, m, max_reservation_time);

    (map, s, seats)
}

#[test]
fn test2_1() {

    let (map, mut s, seats) = testdata2(3);

    let i0 = s.add((), VecDeque::from([0]), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![VecDeque::from([10])]),
        MultipleEnds::new_as_all_zero(vec![VecDeque::from([13])])
    ]));
    
    let mut output = Data{ seats, agents: HashMap::new() };

    output_data(&s, &map, 0, &mut output, &vec![i0]);
    for t in 1..=15 {
        s.step();
        output_data(&s, &map, t, &mut output, &vec![i0])
    }

    output_file(&"test2-1.json".to_string(), &output);
}

#[test]
fn test2_2() {

    let (map, mut s, seats) = testdata2(3);

    let i0 = s.add((), VecDeque::from([3]), VecDeque::from([MultipleEnds::new_as_all_zero(vec![VecDeque::from([16])])]));
    let i1 = s.add((), VecDeque::from([6]), VecDeque::from([MultipleEnds::new_as_all_zero(vec![VecDeque::from([13])])]));
    let i2 = s.add((), VecDeque::from([0]), VecDeque::from([MultipleEnds::new_as_all_zero(vec![VecDeque::from([19])])]));
    let i3 = s.add((), VecDeque::from([3]), VecDeque::from([MultipleEnds::new_as_all_zero(vec![VecDeque::from([19])])]));
    let i4 = s.add((), VecDeque::from([0]), VecDeque::from([MultipleEnds::new_as_all_zero(vec![VecDeque::from([16])])]));
    let i5 = s.add((), VecDeque::from([6]), VecDeque::from([MultipleEnds::new_as_all_zero(vec![VecDeque::from([13])])]));

    let idxs = vec![i0, i1, i2, i3, i4, i5];
    
    let mut output = Data{ seats, agents: HashMap::new() };

    output_data(&s, &map, 0, &mut output, &idxs);
    for t in 1..=22 {
        s.step();
        output_data(&s, &map, t, &mut output, &idxs)
    }

    output_file(&"test2-2.json".to_string(), &output);
}
