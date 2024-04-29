use std::{collections::{BTreeMap, BinaryHeap, HashMap, VecDeque}, fmt::Debug, hash::Hash, marker::PhantomData};

use num_traits::One;

use crate::{agent_data::{AgentData, AgentState}, duration::Duration, index::index::Idx, map::Map, pathfind::{common::MultipleEnds, dijkstra::dijkstra_for_next_reservation}, seat::{AgentIdxType, Seat}};


pub struct Simulator<M: Map<U, T>, U: AgentIdxType + Ord, T = ()> 
{
    time: M::C,
    map: M,
    durations: BinaryHeap<Duration<M::C, M::SI, T, U>>,
    agents: BTreeMap<Idx<T, U>, AgentData<M::Node, M::C, T>>,
    queue: VecDeque<Idx<T, U>>,
    max_reservation_time: M::C,
}

impl<M: Map<U, T>, U: AgentIdxType + Ord, T> Simulator<M, U, T> where M::SI: Hash
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

    pub fn add(&mut self, agent: T, node: M::Node, destination: VecDeque<MultipleEnds<M::Node, M::C>>) -> Idx<T, U> {
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

    pub fn remove(&mut self, idx: Idx<T, U>) -> bool {
        let Some(a) = self.agents.get_mut(&idx) else { return false };
        a.remove()
    }

    pub fn step(&mut self) where <M as Map<U, T>>::SI: Debug, <M as Map<U, T>>::Node: Debug {

        // seat の解放
        while let Some(d) = self.durations.peek() {
            if d.time() > self.time {
                break;
            }

            let d = self.durations.pop().unwrap();
            let i = d.index();
            self.map[d.seat()].remove(i);
        }

        for &idx in &self.queue {
            let Some(a) = self.agents.get_mut(&idx) else { continue };
            match a.state() {
                AgentState::NotPlaced => {
                    let can_place = self.map
                        .seats(a.current(), a.kind())
                        .all(|n| self.map[n].is_empty_for(idx));
                    if can_place {
                        self.map
                            .seats(a.current(), a.kind())
                            .for_each(|n| self.map[n].add(idx));
                        a.place();
                    }
                },
                AgentState::Moving { nexts } => {
                    if nexts[0].1 <= self.time {
                        a.arrives();
                    }
                },
                _ => {},
            }
        }

        let (mut idxs_suc, mut idxs_fail) = (vec![], vec![]);
        while let Some(idx) = self.queue.pop_front() {
            let Some(a) = self.agents.get_mut(&idx) else { continue };

            let success = if let AgentState::Stop = a.state() {
                if a.removing() {
                    for s in self.map.seats(a.current(), a.kind()) {
                        self.map[s].remove(idx);
                    }
                    self.agents.remove(&idx);
                    continue;
                } else {
                    self.set_nexts(idx);
                }
                true
            } else {
                false
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

    fn set_nexts(&mut self, idx: Idx<T, U>) -> bool where <M as Map<U, T>>::SI: Debug, <M as Map<U, T>>::Node: Debug {
        let Some(a) = self.agents.get_mut(&idx) else {
            return false
        };

        let Some(destinations) = a.next_destinations() else {
            return false;
        };

        let path = dijkstra_for_next_reservation(
            a.current().clone(),
            destinations,
            |n| Successor::new(n.clone(), &self.map, a.kind()),
            |s: &M::SI| self.map[s.clone()].is_empty_for(idx),
            self.max_reservation_time,
        );

        let Some(path) = path else {
            return false
        };

        a.departs(path.iter().map(|(n, c, _)| (n.clone(), *c + self.time)));

        let len = path.len();

        let mut seats = HashMap::new();
        let mut c0 = self.time;
        for s in self.map.seats(a.current(), a.kind()) {
            Self::add_seats(&mut seats, s, if len > 0 { Some(c0 + M::C::one()) } else { None });
        }

        let mut n0 = a.current().clone();

        for (j, (n, c, i)) in path.into_iter().enumerate() {
            for (s, d) in self.map.seats_between(&n0, a.kind(), &i) {
                Self::add_seats(&mut seats, s, Some(c0 + d));
                n0 = n.clone();
            }
            if j < len - 1 {
                for s in self.map.seats(&n, a.kind()) {
                    Self::add_seats(&mut seats, s, Some(self.time + M::C::one() + c));
                }
            } else {
                for s in self.map.seats(&n, a.kind()) {
                    Self::add_seats(&mut seats, s, None);
                }
            }
            c0 = c0 + c;
        }

        for (s, t) in seats {
            self.map[s.clone()].add(idx);
            if let Some(t) = t {
                self.durations.push(Duration::new(t, idx, s));
            }
        }
        true
    }

    fn add_seats(seats: &mut HashMap<M::SI, Option<M::C>>, s: M::SI, t: Option<M::C>) {
        if let Some(&d0) = seats.get(&s) {
            let a = match (d0, t) {
                (None, Some(_)) => true,
                (Some(d0), Some(t)) => d0 >= t,
                _ => false,
            };
            if a { return }
        }
        seats.insert(s, t);
    }
}

struct Successor<'a, M: Map<U, T>, U: AgentIdxType, T = ()> {
    node: M::Node,
    map: &'a M,
    kind: &'a T,
    iter: M::SCIter,
    _phu: PhantomData<U>,
}

impl<'a, M: Map<U, T>, U: AgentIdxType, T> Successor<'a, M, U, T> {
    fn new(node: M::Node, map: &'a M, kind: &'a T) -> Self {
        let iter = map.successors(&node, &kind);
        Self { node, map, kind, iter, _phu: PhantomData }
    }
}

impl<'a, M: Map<U, T>, U: AgentIdxType, T> Iterator for Successor<'a, M, U, T> {
    type Item = (M::Node, M::C, SuccessorSeats<M, U, T>, M::I);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .and_then(|(i, m, c)| Some((
                m,
                c,
                SuccessorSeats::new(self.map, &self.node, &self.kind, &i),
                i,
            )))
    }
}

struct SuccessorSeats <M: Map<U, T>, U: AgentIdxType, T = ()> {
    s: M::SBIter,
    t: M::SIter,
}

impl<M: Map<U, T>, U: AgentIdxType, T> SuccessorSeats<M, U, T> {
    fn new<'a>(map: &'a M, node: &'a M::Node, kind: &'a T, index: &'a M::I) -> Self {

        let s: M::SBIter = map
            .seats_between(node, kind, index);

        let t: M::SIter = map
            .seats(&map.successor(node, kind, index), kind);

        Self { s, t }
    }
}

impl<M: Map<U, T>, U: AgentIdxType, T> Iterator for SuccessorSeats<M, U, T> {
    type Item = M::SI;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((s, _)) = self.s.next() {
            return Some(s)
        }
        self.t.next()
    }
}
