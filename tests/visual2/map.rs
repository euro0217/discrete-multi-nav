use std::{ops::{Index, IndexMut}, vec::IntoIter};

use discrete_multi_nav::{index::index::Idx, map::Map, seat::Seat};


pub(crate) struct TestMap {
    occupied: Vec<Vec<TestNode>>,
}

impl TestMap {
    pub(crate) fn new(nx: usize, ny: usize) -> Self {
        Self { occupied: (0..nx).map(|_| vec![TestNode(None); ny]).collect::<Vec<_>>() }
    }

    pub(crate) fn nx(&self) -> usize { self.occupied.len() }
    pub(crate) fn ny(&self) -> usize { self.occupied[0].len() }
}

impl Map<u32> for TestMap {
    type C = u32;

    // y
    // ^
    // | . 2 . 1 .
    //   3 . . . 0
    //   . . o . .
    //   4 . . . 7
    //   . 5 . 6 .
    //           -> x
    type I = usize;
    type SI = (usize, usize);
    type Seat = TestNode;
    type Node = (usize, usize);
    
    type SIter = IntoIter<Self::SI>;
    type SCIter = IntoIter<(Self::I, Self::Node, Self::C)>;
    type SBIter = IntoIter<(Self::SI, Self::C)>;

    fn seats(&self, n: &Self::Node, _: &()) -> Self::SIter {
        vec![*n].into_iter()
    }

    fn successors(&self, &(x, y): &Self::Node, _: &()) -> Self::SCIter {
        DXYS
            .into_iter()
            .enumerate()
            .filter_map(|(i, (dx, dy))| {
                let (x, y) = (x as i32 + dx, y as i32 + dy);
                if 0 <= x && x < self.nx() as i32 && 0 <= y && y < self.ny() as i32 {
                    Some((i, (x as usize, y as usize), 4))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn successor(&self, &(x, y): &Self::Node, _: &(), &i: &Self::I) -> Self::Node {
        let (dx, dy) = DXYS[i];
        ((x as i32 + dx) as usize, (y as i32 + dy) as usize)
    }

    fn seats_between(&self, &(x, y): &Self::Node, _: &(), &i: &Self::I) -> Self::SBIter {
        BS[i]
            .into_iter()
            .map(|(dx, dy, c)| (((x as i32 + dx) as usize, (y as i32 + dy) as usize), c))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

const DXYS: [(i32, i32); 8] = [(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)];

const BS: [[(i32, i32, u32); 3]; 8] = [
    [(0, 0, 2), (1, 0, 3), (1, 1, 4)],
    [(0, 0, 2), (0, 1, 3), (1, 1, 4)],
    [(0, 0, 2), (0, 1, 3), (-1, 1, 4)],
    [(0, 0, 2), (-1, 0, 3), (-1, 1, 4)],
    [(0, 0, 2), (-1, 0, 3), (-1, -1, 4)],
    [(0, 0, 2), (0, -1, 3), (-1, -1, 4)],
    [(0, 0, 2), (0, -1, 3), (1, -1, 4)],
    [(0, 0, 2), (1, 0, 3), (1, -1, 4)],
];

#[derive(Clone, Copy, Debug)]
pub(crate) struct TestNode(Option<Idx<(), u32>>);

impl TestNode {
    pub(crate) fn get(&self) -> Option<Idx<(), u32>> { self.0 }
}

impl Index<(usize, usize)> for TestMap {
    type Output = TestNode;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.occupied[i][j]
    }
}

impl IndexMut<(usize, usize)> for TestMap {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.occupied[i][j]
    }
}

impl Seat<(), u32> for TestNode {
    fn is_empty_for(&self, i: Idx<(), u32>) -> bool {
        if let Some(j) = self.0 {
            i == j
        } else {
            true
        }
    }

    fn add(&mut self, idx: Idx<(), u32>) {
        self.0 = Some(idx)
    }

    fn remove(&mut self, _: Idx<(), u32>) {
        self.0 = None
    }
}