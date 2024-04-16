use std::{collections::{HashMap, HashSet, VecDeque}, fs::File, path::Path};

use discrete_multi_nav::{agent_data::AgentState, index::index::Idx, pathfind::common::MultipleEnds, simulator::Simulator};
use json_schema::{Agent, Data, Seat};
use test_map::TestMap;
use test_node::TestNode;

extern crate discrete_multi_nav;

mod test_map;
mod test_node;
mod json_schema;

fn output_file(filename: &String, output: &Data) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/visual/viewer/outputs");
    let f = File::create(path.join(filename)).unwrap();
    serde_json::to_writer(f, &output).unwrap();
}

fn output_data<const N: usize>(s: &Simulator<TestMap, u32>, map: &[(u32, u32, Vec<usize>); N], t: i32, output: &mut Data, idxs: &Vec<Idx<(), u32>>) {

    for i0 in idxs {
        let a0 = s.agent(*i0).unwrap();

        let mut shape = vec![];
        for &i in a0.current() {
            let &(x, y, _) = &map[i];
            let (x, y) = (x as f32, y as f32);
            shape.extend([(x - 0.4, y - 0.4), (x + 0.4, y - 0.4), (x + 0.4, y + 0.4), (x - 0.4, y + 0.4), (x - 0.4, y - 0.4)].iter().map(|&p| Some(p)));
            shape.push(None);
        }
        
        let state = match a0.state() {
            AgentState::NotPlaced => "n",
            AgentState::Stop => "s",
            AgentState::Moving { nexts: _ } => "m",
        };

        output.agents.entry(i0.value()).or_insert(HashMap::new()).insert(t, Agent { shape, state: state.to_string() });
    }

    for (i, n) in s.map().nodes().iter().enumerate() {
        output.seats[i].agent.push(n.occupied().and_then(|n| Some(n.value())));
    }
}

fn testdata1(max_reservation_time: u32) -> ([(u32, u32, Vec<usize>); 14], Simulator<TestMap, u32>, Vec<Seat>) {
    //  10 <- 9 <- 8 <--  7 <- 6
    //   v         v           ^
    //  11        12 <-> 13 -> 5 
    //   v         v      ^    ^
    //   0 -> 1 -> 2 -->  3 -> 4
    //
    let map = [
        (0, 0, vec![1]),
        (1, 0, vec![2]),
        (2, 0, vec![3]),
        (3, 0, vec![4, 13]),
        (4, 0, vec![5]),
        (4, 1, vec![6]),
        (4, 2, vec![7]),
        (3, 2, vec![8]),
        (2, 2, vec![9, 12]),
        (1, 2, vec![10]),
        (0, 2, vec![11]),
        (0, 1, vec![0]),
        (2, 1, vec![2, 13]),
        (3, 1, vec![5, 12]),
    ];
    
    let ns = map.iter().map(|(x, y, js)| TestNode::new(*x, *y, js.to_vec())).collect::<Vec<_>>();
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

    let i0 = s.add((), VecDeque::from([1, 0]), MultipleEnds::new(&vec![HashSet::from([VecDeque::from([8, 7])])]));
    
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

    let i0 = s.add((), VecDeque::from([0, 11, 10]), MultipleEnds::new(&vec![HashSet::from([VecDeque::from([7, 6, 5])])]));
    let i1 = s.add((), VecDeque::from([7, 6]), MultipleEnds::new(&vec![HashSet::from([VecDeque::from([13, 3])])]));
    
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
        .map(|_| s.add((), VecDeque::from([7, 6]), MultipleEnds::new(&vec![HashSet::from([VecDeque::from([5, 4])])])))
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
        .map(|&i| s.add((), VecDeque::from([i]), MultipleEnds::new(&vec![HashSet::from([VecDeque::from([10])])])))
        .collect::<Vec<_>>();
    
    let mut output = Data{ seats, agents: HashMap::new()};

    output_data(&s, &map, 0, &mut output, &idxs);
    for t in 1..=12 {
        s.step();
        output_data(&s, &map, t, &mut output, &idxs)
    }

    output_file(&"test1-4.json".to_string(), &output);
}