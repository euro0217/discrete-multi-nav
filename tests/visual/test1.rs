use std::collections::{HashMap, VecDeque};

use discrete_multi_nav::{pathfind::common::MultipleEnds, simulator::Simulator};
use crate::{json_schema::{Data, Seat}, output_data, output_file, test_map::TestMap, test_node::TestNode};

fn testdata1(max_reservation_time: u32) -> ([(u32, u32, Vec<(usize, u32)>); 14], Simulator<TestMap, u32>, Vec<Seat>) {
    //  10 <- 9 <- 8 <--  7 <- 6
    //   v         v           ^
    //  11        12 <-> 13 -> 5 
    //   v         v      ^    ^
    //   0 -> 1 -> 2 -->  3 -> 4
    //
    let map = [
        (0, 0, vec![(1, 1)]),
        (1, 0, vec![(2, 1)]),
        (2, 0, vec![(3, 1)]),
        (3, 0, vec![(4, 1), (13, 1)]),
        (4, 0, vec![(5, 1)]),
        (4, 1, vec![(6, 1)]),
        (4, 2, vec![(7, 1)]),
        (3, 2, vec![(8, 1)]),
        (2, 2, vec![(9, 1), (12, 1)]),
        (1, 2, vec![(10, 1)]),
        (0, 2, vec![(11, 1)]),
        (0, 1, vec![(0, 1)]),
        (2, 1, vec![(2, 1), (13, 1)]),
        (3, 1, vec![(5, 1), (12, 1)]),
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
fn test1_1() {

    let (map, mut s, seats) = testdata1(3);

    let i0 = s.add((), VecDeque::from([1, 0]), VecDeque::from([MultipleEnds::new_as_all_zero(vec![VecDeque::from([8, 7])])]));
    
    let mut output = Data{ seats, agents: HashMap::new() };

    output_data(&s, &map, 0, &mut output, &vec![i0]);
    for t in 1..=12 {
        s.step();
        output_data(&s, &map, t, &mut output, &vec![i0])
    }

    output_file(&"test1-1.json".to_string(), &output);
}

#[test]
fn test1_2() {

    let (map, mut s, seats) = testdata1(3);

    let i0 = s.add((), VecDeque::from([0, 11, 10]), VecDeque::from([MultipleEnds::new_as_all_zero(vec![VecDeque::from([7, 6, 5])])]));
    let i1 = s.add((), VecDeque::from([7, 6]), VecDeque::from([MultipleEnds::new_as_all_zero(vec![VecDeque::from([13, 3])])]));
    
    let mut output = Data{ seats, agents: HashMap::new() };

    output_data(&s, &map, 0, &mut output, &vec![i0, i1]);
    for t in 1..=12 {
        s.step();
        output_data(&s, &map, t, &mut output, &vec![i0, i1])
    }

    output_file(&"test1-2.json".to_string(), &output);
}

#[test]
fn test1_3() {

    let (map, mut s, seats) = testdata1(3);

    let idxs = (0..3)
        .map(|_| s.add((), VecDeque::from([7, 6]), VecDeque::from([MultipleEnds::new_as_all_zero(vec![VecDeque::from([5, 4])])])))
        .collect::<Vec<_>>();
    
    let mut output = Data{ seats, agents: HashMap::new()};

    output_data(&s, &map, 0, &mut output, &idxs);
    for t in 1..=12 {
        s.step();
        output_data(&s, &map, t, &mut output, &idxs)
    }

    output_file(&"test1-3.json".to_string(), &output);
}

#[test]
fn test1_4() {

    let (map, mut s, seats) = testdata1(3);

    let idxs = [0, 1, 7, 8, 10, 11]
        .iter()
        .map(|&i| s.add((), VecDeque::from([i]), VecDeque::from([MultipleEnds::new_as_all_zero(vec![VecDeque::from([10])])])))
        .collect::<Vec<_>>();
    
    let mut output = Data{ seats, agents: HashMap::new()};

    output_data(&s, &map, 0, &mut output, &idxs);
    for t in 1..=12 {
        s.step();
        output_data(&s, &map, t, &mut output, &idxs)
    }

    output_file(&"test1-4.json".to_string(), &output);
}

#[test]
fn test1_5() {

    let (map, mut s, seats) = testdata1(3);

    let i0 = s.add((), VecDeque::from([1, 0, 11]), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![VecDeque::from([2, 12, 13])]),
        MultipleEnds::new_as_all_zero(vec![VecDeque::from([1, 0, 11])]),
    ]));

    let idxs = vec![i0];
    
    let mut output = Data{ seats, agents: HashMap::new()};

    output_data(&s, &map, 0, &mut output, &idxs);
    for t in 1..=20 {
        s.step();
        output_data(&s, &map, t, &mut output, &idxs)
    }

    output_file(&"test1-5.json".to_string(), &output);
}

#[test]
fn test1_6() {

    let (map, mut s, seats) = testdata1(3);

    let i0 = s.add((), VecDeque::from([1, 0, 11]), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![VecDeque::from([2, 12, 13])]),
        MultipleEnds::new_as_all_zero(vec![VecDeque::from([1, 0, 11])]),
    ]));

    let i1 = s.add((), VecDeque::from([6, 5]), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![VecDeque::from([13, 12])]),
        MultipleEnds::new_as_all_zero(vec![VecDeque::from([11, 10])]),
    ]));

    let idxs = vec![i0, i1];
    
    let mut output = Data{ seats, agents: HashMap::new()};

    output_data(&s, &map, 0, &mut output, &idxs);
    for t in 1..=8 {
        s.step();
        output_data(&s, &map, t, &mut output, &idxs)
    }

    output_file(&"test1-6.json".to_string(), &output);
}

#[test]
fn test1_7() {

    let (map, mut s, seats) = testdata1(3);

    let i0 = s.add((), VecDeque::from([4, 3]), VecDeque::from([
        MultipleEnds::new(HashMap::from([(VecDeque::from([2, 12]), 2), (VecDeque::from([10, 9]), 0)])),
        MultipleEnds::new_as_all_zero(vec![VecDeque::from([4, 3])]),
    ]));

    let i1 = s.add((), VecDeque::from([7, 6]), VecDeque::from([
        MultipleEnds::new(HashMap::from([(VecDeque::from([2, 12]), 3), (VecDeque::from([2, 1]), 0)])),
        MultipleEnds::new_as_all_zero(vec![VecDeque::from([7, 6])]),
    ]));

    let idxs = vec![i0, i1];
    
    let mut output = Data{ seats, agents: HashMap::new()};

    output_data(&s, &map, 0, &mut output, &idxs);
    for t in 1..=15 {
        s.step();
        output_data(&s, &map, t, &mut output, &idxs)
    }

    output_file(&"test1-7.json".to_string(), &output);
}
