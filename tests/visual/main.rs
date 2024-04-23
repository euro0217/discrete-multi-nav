use std::{collections::HashMap, fs::File, path::Path};

use discrete_multi_nav::{agent_data::AgentState, index::index::Idx, simulator::Simulator};
use json_schema::{Agent, Data};
use test_map::TestMap;

extern crate discrete_multi_nav;

mod test_map;
mod test_node;
mod json_schema;

mod test1;
mod test2;

fn output_file(filename: &String, output: &Data) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/visual/viewer/outputs");
    let f = File::create(path.join(filename)).unwrap();
    serde_json::to_writer(f, &output).unwrap();
}

fn output_data<const N: usize>(s: &Simulator<TestMap, u32>, map: &[(u32, u32, Vec<(usize, u32)>); N], t: i32, output: &mut Data, idxs: &Vec<Idx<(), u32>>) {

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
