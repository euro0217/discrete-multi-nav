use std::{collections::{HashMap, VecDeque}, fs::File, path::Path};

use discrete_multi_nav::{agent_data::AgentState, index::index::Idx, pathfind::common::MultipleEnds, simulator::Simulator};
use map::TestMap;
use serde::Serialize;

extern crate discrete_multi_nav;

mod map;

#[derive(Serialize)]
pub(crate) struct Data {
    map: Vec<Vec<Option<u32>>>,
    agents: HashMap<u32, Agent>,
}

#[derive(Serialize)]
pub(crate) struct Agent {
    x: usize,
    y: usize,
    state: String,
    next: Option<(usize, usize, f32)>,
    dest: Option<Vec<(usize, usize)>>,
}

fn output_file(filename: &String, output: &Vec<Data>) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/visual2/viewer/outputs");
    let f = File::create(path.join(filename)).unwrap();
    serde_json::to_writer(f, &output).unwrap();
}


fn output_data(s: &Simulator<TestMap, u32>, idxs: &Vec<Idx<(), u32>>, t: u32) -> Data {

    let map = (0..s.map().nx())
        .map(|x| (0..s.map().ny()).map(move |y| s.map()[(x, y)].get().and_then(|i| Some(i.value()))).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let agents = idxs
        .into_iter()
        .filter_map(|&i| 
            s.agent(i)
                .and_then(|a| {
                    let &(x, y) = a.current();
                    let state = match a.state() {
                        AgentState::NotPlaced => "n",
                        AgentState::Stop => "s",
                        AgentState::Moving { nexts: _ } => "m",
                    };
                    let next = if let AgentState::Moving { nexts } = a.state() {
                        Some((nexts[0].0.0, nexts[0].0.1, 1. - (nexts[0].1 + 1 - t) as f32 / 4.))
                    } else {
                        None
                    };
                    let dest = a.next_destinations()
                        .and_then(|m| Some(m.ends().keys().copied().collect::<Vec<_>>()) );
                    Some((i.value(), Agent { x, y, state: state.to_string(), next, dest }))
                })
        )
        .collect::<HashMap<_, _>>();

    Data{ map, agents }
}

#[test]
fn test1() {
    let mut s = Simulator::new(0, TestMap::new(8, 5), 5);

    let i0 = s.add((), (0, 0), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![(7, 4)]),
        MultipleEnds::new_as_all_zero(vec![(0, 0)]),
    ]));
    let idxs = vec![i0];

    let mut output = vec![];

    output.push(output_data(&s, &idxs, 0));

    for t in 1..=42 {
        s.step();
        output.push(output_data(&s, &idxs, t));
    }

    output_file(&"test1.json".to_string(), &output);
}

#[test]
fn test2() {
    let mut s = Simulator::new(0, TestMap::new(8, 5), 5);

    let i0 = s.add((), (0, 0), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![(7, 4)]),
        MultipleEnds::new_as_all_zero(vec![(0, 0)]),
    ]));
    let i1 = s.add((), (0, 4), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![(7, 0)]),
        MultipleEnds::new_as_all_zero(vec![(0, 4)]),
    ]));
    let idxs = vec![i0, i1];

    let mut output = vec![];

    output.push(output_data(&s, &idxs, 0));

    for t in 1..=53 {
        s.step();
        output.push(output_data(&s, &idxs, t));
    }

    output_file(&"test2.json".to_string(), &output);
}

#[test]
fn test3() {
    let mut s = Simulator::new(0, TestMap::new(12, 10), 10);

    let ps = [(11, 0), (11, 9), (0, 9)];

    let mut idxs = vec![];
    for i in 0..3 {
        let j = s.add((), (0, 0), VecDeque::from_iter(
            (0..3).map(|k| MultipleEnds::new_as_all_zero(vec![ps[(i + k) % 3]]))
                .chain(vec![MultipleEnds::new_as_all_zero(vec![(0, 0)])].into_iter())
        ));
        idxs.push(j);
    }

    let mut output = vec![];

    output.push(output_data(&s, &idxs, 0));

    for t in 1..=124 {
        s.step();
        output.push(output_data(&s, &idxs, t));
    }
    s.remove(idxs[0]);
    for t in 125..=128 {
        s.step();
        output.push(output_data(&s, &idxs, t));
    }
    s.remove(idxs[1]);
    for t in 128..=134 {
        s.step();
        output.push(output_data(&s, &idxs, t));
    }

    output_file(&"test3.json".to_string(), &output);
}

#[test]
fn test4() {
    let mut s = Simulator::new(0, TestMap::new(8, 8), 5);

    let i0 = s.add((), (0, 0), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![(3, 4), (4, 3)]),
        MultipleEnds::new_as_all_zero(vec![(0, 7), (7, 0)]),
        MultipleEnds::new_as_all_zero(vec![(7, 7)]),
        MultipleEnds::new_as_all_zero(vec![(0, 0)]),
        MultipleEnds::new_as_all_zero(vec![(3, 4), (4, 3)]),
        MultipleEnds::new_as_all_zero(vec![(0, 7), (7, 0)]),
        MultipleEnds::new_as_all_zero(vec![(7, 7)]),
        MultipleEnds::new_as_all_zero(vec![(0, 0)]),
    ]));
    let i1 = s.add((), (7, 7), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![(3, 4), (4, 3)]),
        MultipleEnds::new_as_all_zero(vec![(0, 7), (7, 0)]),
        MultipleEnds::new_as_all_zero(vec![(0, 0)]),
        MultipleEnds::new_as_all_zero(vec![(7, 7)]),
        MultipleEnds::new_as_all_zero(vec![(3, 4), (4, 3)]),
        MultipleEnds::new_as_all_zero(vec![(0, 7), (7, 0)]),
        MultipleEnds::new_as_all_zero(vec![(0, 0)]),
        MultipleEnds::new_as_all_zero(vec![(7, 7)]),
    ]));
    let i2 = s.add((), (7, 0), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![(3, 3), (4, 4)]),
        MultipleEnds::new_as_all_zero(vec![(0, 0), (7, 7)]),
        MultipleEnds::new_as_all_zero(vec![(0, 7)]),
        MultipleEnds::new_as_all_zero(vec![(7, 0)]),
        MultipleEnds::new_as_all_zero(vec![(3, 3), (4, 4)]),
        MultipleEnds::new_as_all_zero(vec![(0, 0), (7, 7)]),
        MultipleEnds::new_as_all_zero(vec![(0, 7)]),
        MultipleEnds::new_as_all_zero(vec![(7, 0)]),
    ]));
    let i3 = s.add((), (0, 7), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![(3, 3), (4, 4)]),
        MultipleEnds::new_as_all_zero(vec![(0, 0), (7, 7)]),
        MultipleEnds::new_as_all_zero(vec![(7, 0)]),
        MultipleEnds::new_as_all_zero(vec![(0, 7)]),
        MultipleEnds::new_as_all_zero(vec![(3, 3), (4, 4)]),
        MultipleEnds::new_as_all_zero(vec![(0, 0), (7, 7)]),
        MultipleEnds::new_as_all_zero(vec![(7, 0)]),
        MultipleEnds::new_as_all_zero(vec![(0, 7)]),
    ]));
    let idxs = vec![i0, i1, i2, i3];

    let mut output = vec![];

    output.push(output_data(&s, &idxs, 0));

    for t in 1..=148 {
        s.step();
        output.push(output_data(&s, &idxs, t));
    }

    output_file(&"test4.json".to_string(), &output);
}

#[test]
fn test5() {
    let mut s = Simulator::new(0, TestMap::new(8, 5), 5);

    let i0 = s.add((), (0, 0), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![(7, 4)]),
        MultipleEnds::new_as_all_zero(vec![(0, 0)]),
    ]));
    let i1 = s.add((), (0, 4), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![(7, 0)]),
        MultipleEnds::new_as_all_zero(vec![(0, 4)]),
    ]));
    let i2 = s.add((), (7, 4), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![(0, 0)]),
        MultipleEnds::new_as_all_zero(vec![(7, 4)]),
    ]));
    let i3 = s.add((), (7, 0), VecDeque::from([
        MultipleEnds::new_as_all_zero(vec![(0, 4)]),
        MultipleEnds::new_as_all_zero(vec![(7, 0)]),
    ]));
    let idxs = vec![i0, i1, i2, i3];

    let mut output = vec![];

    output.push(output_data(&s, &idxs, 0));

    for t in 1..=60 {
        s.step();
        output.push(output_data(&s, &idxs, t));
    }

    output_file(&"test5.json".to_string(), &output);
}
