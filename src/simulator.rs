use std::{collections::{BTreeMap, BinaryHeap, HashMap, VecDeque}, hash::Hash};

use num_traits::{bounds::UpperBounded, Unsigned, One};

use crate::{agent_data::{AgentData, AgentState}, duration::Duration, index::index::Idx, map::Map, pathfind::{common::MultipleEnds, dijkstra::dijkstra_for_next_reservation}, seat::Seat};


pub struct Simulator<M, U, T = ()> 
where
    U: Copy + Unsigned + UpperBounded + Ord,
    M: Map<U, T>,
{
    time: M::C,
    map: M,
    durations: BinaryHeap<Duration<M::C, M::SI, T, U>>,
    agents: BTreeMap<Idx<T, U>, AgentData<M::Node, M::C, T>>,
    queue: VecDeque<Idx<T, U>>,
    max_reservation_time: M::C,
}

impl<M, U, T> Simulator<M, U, T>
where
    U: Copy + Unsigned + UpperBounded + Ord,
    M: Map<U, T>,
    M::SI: Hash,
{
    pub fn new(init_time: M::C, map: M, max_reservation_time: M::C) -> Self {
        Self {
            time: init_time,
            map,
            durations: BinaryHeap::new(),
            agents: BTreeMap::new(),
            queue: VecDeque::new(),
            max_reservation_time,
        }
    }

    pub fn map(&self) -> &M { &self.map }

    pub fn agents(&self) -> &BTreeMap<Idx<T, U>, AgentData<M::Node, M::C, T>> { &self.agents }
    pub fn agent(&self, idx: Idx<T, U>) -> Option<&AgentData<M::Node, M::C, T>> { self.agents.get(&idx) }

    pub fn add(&mut self, agent: T, node: M::Node, destination: MultipleEnds<M::Node>) -> Idx<T, U> {
        // node のバリデーション
        let idx = self.new_idx();
        self.agents.insert(idx, AgentData::new(agent, node, destination));
        self.queue.push_back(idx);
        idx
    }

    fn new_idx(&self) -> Idx<T, U> {
        let Some(&i) = self.agents.keys().min() else { return Idx::new(U::zero()) };
        
        if i.value() != U::zero() {
            return Idx::new(i.value() - U::one())
        }

        return Idx::new(self.agents.keys().max().unwrap().value() + U::one())
    }

    pub fn step(&mut self) {

        // seat の解放
        while let Some(d) = self.durations.peek() {
            if d.time() > self.time {
                break;
            }

            let d = self.durations.pop().unwrap();
            let i = d.index();
            self.map[d.seat()].remove(i);
        }

        //
        let (mut idxs_suc, mut idxs_fail) = (vec![], vec![]);
        while let Some(idx) = self.queue.pop_front() {
            let Some(a) = self.agents.get_mut(&idx) else { continue };

            let success = match a.state() {
                AgentState::NotPlaced => {
                    if self.map
                        .seats(a.current(), a.kind())
                        .all(|n| self.map[n].is_empty_for(idx)) {
                        self.map
                            .seats(a.current(), a.kind())
                            .for_each(|n| self.map[n].add(idx));
                        a.place();
                        self.set_nexts(idx)
                    } else {
                        false
                    }
                },
                AgentState::Stop => self.set_nexts(idx),
                AgentState::Moving { nexts } => {
                    if nexts[0].1 <= self.time {
                        a.arrives();
                        self.set_nexts(idx);
                        true
                    } else {
                        false
                    }
                },
            };

            if success {
                idxs_suc.push(idx);
            } else {
                idxs_fail.push(idx);
            }
        }
        self.queue.extend(idxs_fail);
        self.queue.extend(idxs_suc);
        
        self.time = self.time + M::C::one();
    }

    fn set_nexts(&mut self, idx: Idx<T, U>) -> bool {
        let Some(a) = self.agents.get_mut(&idx) else {
            return false
        };

        let path = dijkstra_for_next_reservation(
            a.current().clone(),
            a.destinations(),
            |n| self.map
                .successors(n, a.kind())
                .map(|(i, n, c)| (
                    n.clone(),
                    c,
                    self.map
                        .seats_between(&n, a.kind(), &i)
                        .map(|(s, _)| s)
                        .chain(
                            self.map
                                .seats(&self.map.successor(&n, a.kind(), &i), a.kind())
                                .into_iter()
                        ),
                    i,
                )),
            |s: &M::SI| self.map[s.clone()].is_empty_for(idx),
            self.max_reservation_time,
        );

        let Some(path) = path else {
            return false
        };
        
        a.departs(path.iter().map(|(n, c, _)| (n.clone(), *c)));

        let mut seats = HashMap::new();
        let mut c0 = self.time;
        for s in self.map.seats(a.current(), a.kind()) {
            Self::add_seats(&mut seats, s, c0);
        }
        for (n, c, i) in path.into_iter() {
            for s in self.map.seats(&n, a.kind()) {
                Self::add_seats(&mut seats, s, self.time + c);
            }

            for (s, d) in self.map.seats_between(&n, a.kind(), &i) {
                Self::add_seats(&mut seats, s, c0 + d);
            }
            c0 = c0 + c;
        }

        for (s, t) in seats {
            self.map[s.clone()].add(idx);
            self.durations.push(Duration::new(t, idx, s));
        }
        true
    }

    fn add_seats(seats: &mut HashMap<M::SI, M::C>, s: M::SI, t: M::C) {
        if let Some(&d0) = seats.get(&s) {
            if d0 >= t { return }
        }
        seats.insert(s, t);
    }
}
