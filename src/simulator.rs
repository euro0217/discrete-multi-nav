use std::{collections::{BTreeMap, BinaryHeap, HashMap, VecDeque}, hash::Hash, marker::PhantomData};

use num_traits::One;

use crate::{agent_data::{AgentData, AgentState}, duration::Duration, index::index::Idx, map::{Map, Movement}, pathfind::{astar::astar_for_next_reservation, common::MultipleEnds, dijkstra::dijkstra_for_next_reservation}, seat::{AgentIdxType, Seat}};

use crate::map::Heuristic;
pub struct Simulator<M: Map<U, T>, U: AgentIdxType + Ord, T = ()> 
{
    time: M::Cost,
    map: M,
    durations: BinaryHeap<Duration<M::Cost, M::SeatIndex, T, U>>,
    agents: BTreeMap<Idx<T, U>, AgentData<M::Node, M::Cost, T>>,
    queue: VecDeque<Idx<T, U>>,
    max_reservation_time: M::Cost,
}

impl<M: Map<U, T>, U: AgentIdxType + Ord, T> Simulator<M, U, T> where M::SeatIndex: Hash
{
    pub fn new(init_time: M::Cost, map: M, max_reservation_time: M::Cost) -> Self {
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

    pub fn agents(&self) -> &BTreeMap<Idx<T, U>, AgentData<M::Node, M::Cost, T>> { &self.agents }
    pub fn agent(&self, idx: Idx<T, U>) -> Option<&AgentData<M::Node, M::Cost, T>> { self.agents.get(&idx) }

    pub fn agent_destination_mut(&mut self, idx: Idx<T, U>) -> Option<&mut VecDeque<MultipleEnds<<M as Map<U, T>>::Node, <M as Map<U, T>>::Cost>>> {
        self.agents.get_mut(&idx)
            .and_then(|a| Some(a.destinations_mut()))
    }

    pub fn movement_of(&self, idx: Idx<T, U>, index: M::I) -> Option<Movement<M, U, T>> {
        let Some(a) = self.agent(idx) else { return None };

        let c = if let AgentState::Moving { nexts } = a.state() {
            &nexts[nexts.len() - 1].0
        } else {
            a.current()
        };
        self.map.movement(c, a.kind(), &index)
    }

    pub fn is_empty_for(&self, idx: Idx<T, U>, s: &Movement<M, U, T>) -> bool {
        s.seats()
            .iter()
            .all(|(s, _)| self.map[s.clone()].is_empty_for(idx))
    }

    pub fn add(&mut self, agent: T, node: M::Node, destination: VecDeque<MultipleEnds<M::Node, M::Cost>>) -> Idx<T, U> {
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
        
        self.time = self.time + M::Cost::one();
    }

    fn set_nexts(&mut self, idx: Idx<T, U>) -> bool {
        let Some(a) = self.agents.get_mut(&idx) else {
            return false
        };

        let Some(destinations) = a.next_destinations() else {
            return false;
        };

        let path = if let Some(heuristic) = self.map.heuristic(destinations) {
            astar_for_next_reservation(
                a.current().clone(),
                destinations,
                |n| Successor::new(n.clone(), &self.map, a.kind()),
                |s: &M::SeatIndex| self.map[s.clone()].is_empty_for(idx),
                self.max_reservation_time,
                |n| heuristic.heuristic(n),
            )
        } else {
            dijkstra_for_next_reservation(
                a.current().clone(),
                destinations,
                |n| Successor::new(n.clone(), &self.map, a.kind()),
                |s: &M::SeatIndex| self.map[s.clone()].is_empty_for(idx),
                self.max_reservation_time,
            )
        };

        let Some(path) = path else {
            return false
        };

        a.departs(path.iter().map(|(n, c, _)| (n.clone(), *c + self.time)));

        let len = path.len();

        let mut seats = HashMap::new();
        let mut c0 = self.time;
        for s in self.map.seats(a.current(), a.kind()) {
            Self::add_seats(&mut seats, s, if len > 0 { Some(c0 + M::Cost::one()) } else { None });
        }

        let mut n0 = a.current().clone();

        for (j, (n, c, i)) in path.into_iter().enumerate() {
            for (s, d) in self.map.seats_between(&n0, a.kind(), &i) {
                Self::add_seats(&mut seats, s, Some(c0 + d));
                n0 = n.clone();
            }
            if j < len - 1 {
                for s in self.map.seats(&n, a.kind()) {
                    Self::add_seats(&mut seats, s, Some(self.time + M::Cost::one() + c));
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

    fn add_seats(seats: &mut HashMap<M::SeatIndex, Option<M::Cost>>, s: M::SeatIndex, t: Option<M::Cost>) {
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
    type Item = (M::Node, M::Cost, SuccessorSeats<M, U, T>, M::I);

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
    t: Option<M::SIter>,
}

impl<M: Map<U, T>, U: AgentIdxType, T> SuccessorSeats<M, U, T> {
    fn new<'a>(map: &'a M, node: &'a M::Node, kind: &'a T, index: &'a M::I) -> Self {
        let s = map.seats_between(node, kind, index);
        let t = map
            .successor(node, kind, index)
            .and_then(|ss| Some(map.seats(&ss, kind)));
        Self { s, t }
    }
}

impl<M: Map<U, T>, U: AgentIdxType, T> Iterator for SuccessorSeats<M, U, T> {
    type Item = M::SeatIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((s, _)) = self.s.next() {
            return Some(s)
        }
        if let Some(t) = &mut self.t {
            return t.next()
        }
        return None
    }
}
