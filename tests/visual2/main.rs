use std::{collections::{HashMap, HashSet, VecDeque}, fs::File, io::Write, path::Path, time::Instant};

use discrete_multi_nav::{agent_data::AgentState, index::index::Idx, map::Movement, pathfind::common::MultipleEnds, simulator::Simulator};
use map::TestMap;
use rand::{thread_rng, Rng};
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

    for t in 1..=40 {
        s.step();
        output.push(output_data(&s, &idxs, t));
    }

    s.agent_destination_mut(i3).unwrap()
        .push_front(MultipleEnds::new_as_all_zero(vec![(4, 2)]));

    for t in 41..=58 {
        s.step();
        output.push(output_data(&s, &idxs, t));
    }

    output_file(&"test5.json".to_string(), &output);
}

#[test]
fn movement_test() {
    let mut s = Simulator::new(0, TestMap::new(7, 4), 5);

    let i0 = s.add((), (6, 0), VecDeque::from([MultipleEnds::new_as_all_zero(vec![(0, 3)])]));

    fn assert_seat(m: &Movement<TestMap, u32, ()>, expected: Vec<((usize, usize), Option<u32>)>) {
        let actual = m.seats().into_iter().copied().collect::<HashSet<_>>();
        assert_eq!(HashSet::from_iter(expected.into_iter()), actual);
    }

    let a = s.agent(i0).unwrap();
    assert_eq!(*a.current(), (6, 0));
    assert_seat(&s.movement_of(i0, 2).unwrap(), vec![((6, 0), Some(2)), ((6, 1), Some(3)), ((5, 1), Some(4)), ((5, 2), None)]);
    assert!(s.movement_of(i0, 1).is_none());

    s.step();
    
    for _ in 0..4 {
        assert_eq!(*s.agent(i0).unwrap().current(), (6, 0));
        assert_seat(&s.movement_of(i0, 1).unwrap(), vec![((4, 1), Some(2)), ((4, 2), Some(3)), ((5, 2), Some(4)), ((5, 3), None)]);
        assert!(s.movement_of(i0, 6).is_none());

        s.step();
    }

    for _ in 0..4 {
        assert_eq!(*s.agent(i0).unwrap().current(), (4, 1));
        assert_seat(&s.movement_of(i0, 5).unwrap(), vec![((2, 2), Some(2)), ((2, 1), Some(3)), ((1, 1), Some(4)), ((1, 0), None)]);
        assert!(s.movement_of(i0, 8).is_none());

        s.step();
    }
    
    for _ in 0..4 {
        assert_eq!(*s.agent(i0).unwrap().current(), (2, 2));
        assert_seat(&s.movement_of(i0, 7).unwrap(), vec![((0, 3), Some(2)), ((1, 3), Some(3)), ((1, 2), Some(4)), ((2, 2), None)]);
        assert!(s.movement_of(Idx::new(99999), 8).is_none());

        s.step();
    }
    assert_eq!(*s.agent(i0).unwrap().current(), (0, 3));
    assert_seat(&s.movement_of(i0, 6).unwrap(), vec![((0, 3), Some(2)), ((0, 2), Some(3)), ((1, 2), Some(4)), ((1, 1), None)]);
}


fn performance_test_data(map_size: usize, n_agents: usize, n_destinations: usize) -> (Simulator<TestMap, u32>, Vec<Idx<(), u32>>) {
    let mut s = Simulator::new(0, TestMap::new(map_size, map_size), 5);

    let mut rng = thread_rng();
    
    let idxs = (0..n_agents)
        .map(|_| {
            let (x, y) = (rng.gen_range(0..map_size), rng.gen_range(0..map_size));
            let dests = (0..n_destinations)
                .map(|_| {
                    let (x, y) = (rng.gen_range(0..map_size), rng.gen_range(0..map_size));
                    MultipleEnds::new_as_all_zero(vec![(x, y)])
                });
            s.add((), (x, y), VecDeque::from_iter(dests))
        })
        .collect::<Vec<_>>();

    (s, idxs)
}

#[test]
fn performance_test_visual() {
    let n_step = 1000;

    let (mut s, idxs) = performance_test_data(30, 5, 3);

    let mut output = vec![];
    output.push(output_data(&s, &idxs, 0));

    let mut n_stops = idxs.iter().map(|&i| (i, 0)).collect::<HashMap<_, _>>();

    for t in 1..=n_step {
        s.step();
        output.push(output_data(&s, &idxs, t));

        for (idx, a) in s.agents() {
            let n = n_stops.get_mut(idx).unwrap();
            match a.state() {
                AgentState::Moving { nexts: _ } => { *n = 0 },
                _ => { *n += 1 }
            }
        }
        if n_stops.values().all(|&n| n >= 2) {
            println!("finished at t = {}", t);
            if s.agents().values().all(|a| a.all_destinations().is_empty()) {
                println!("all agents completed their movements.");
            } else {
                println!("some agents may finished with deadlock.");
            }
            break;
        }
    }

    output_file(&"visual_test.json".to_string(), &output);
}

#[test]
fn performance_test() {

    let n_step = 100;

    let map_sizes = [50, 100, 200];
    let n_agents = [5, 10, 20];

    let n_try = 5;

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/visual2/viewer/outputs");
    let mut f = File::create(path.join("visual.csv")).unwrap();
    f.write_all(b"map_size,n_agent,total_time(msec),steps,time_per_step(msec)\n").unwrap();

    for map_size in map_sizes {
        for n_agent in n_agents {

            for _ in 0..n_try {
                let (mut s, idxs) = performance_test_data(map_size, n_agent, 3);

                let mut n_stops = idxs.iter().map(|&i| (i, 0)).collect::<HashMap<_, _>>();

                let mut total_time = 0;
                let mut n = 0;
                for t in 1..=n_step {

                    let t0 = Instant::now();
                    s.step();
                    total_time += t0.elapsed().as_micros();
            
                    for (idx, a) in s.agents() {
                        let n = n_stops.get_mut(idx).unwrap();
                        match a.state() {
                            AgentState::Moving { nexts: _ } => { *n = 0 },
                            _ => { *n += 1 }
                        }
                    }
                    if n_stops.values().all(|&n| n >= 2) {
                        println!("finished at t = {}", t);
                        if s.agents().values().all(|a| a.all_destinations().is_empty()) {
                            println!("all agents completed their movements.");
                        } else {
                            println!("some agents may finished with deadlock.");
                        }
                        n = t;
                        break;
                    }
                }
                if n == 0 {
                    n = n_step;
                }

                total_time /= 1000; // milli

                f.write_all(format!("{},{},{},{},{}\n", map_size, n_agent, total_time, n, total_time as f64 / n as f64).as_bytes()).unwrap();
            }
        }
    }
}
