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
}

fn output_file(filename: &String, output: &Vec<Data>) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/visual2/viewer/outputs");
    let f = File::create(path.join(filename)).unwrap();
    serde_json::to_writer(f, &output).unwrap();
}


fn output_data(s: &Simulator<TestMap, u32>, idxs: &Vec<Idx<(), u32>>) -> Data {

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
                    Some((i.value(), Agent { x, y, state: state.to_string() }))
                })
        )
        .collect::<HashMap<_, _>>();

    Data{ map, agents }
}

#[test]
fn test1() {
    let mut s = Simulator::new(0, TestMap::new(8, 5), 5);

    let i0 = s.add((), (2, 1), VecDeque::from([MultipleEnds::new_as_all_zero(vec![(6, 4)])]));
    let idxs = vec![i0];

    let mut output = vec![];

    output.push(output_data(&s, &idxs));

    for _ in 1..=10 {
        s.step();
        output.push(output_data(&s, &idxs));
    }

    output_file(&"test1.json".to_string(), &output);
}