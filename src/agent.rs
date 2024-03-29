// use std::hash::Hash;

// use num_traits::Zero;

// pub trait Agent
// {
//     type N: Eq + Hash + Clone;
//     type C: Zero + Ord + Copy + Hash;
//     type S: Eq + Clone;
//     type Iter: IntoIterator<Item = (Self::N, Self::C, Self::SIter)>;
//     type SIter: Iterator<Item = (Self::S, u64)>;
//     type FIter: Iterator<Item = Self::S>;

//     fn successors(&self, n: &Self::N) -> Self::Iter;
//     fn seats(&self, n: &Self::N) -> Self::FIter;
// }

// #[cfg(test)]
// mod tests {
//     use std::{collections::HashSet, vec::IntoIter};

//     use crate::pathfind::{common::MultipleEnds, dijkstra::{dijkstra_for_multiple_ends, dijkstra_for_next_reservation}};

//     use super::Agent;

//     struct TestAgent<const N: usize> {
//         steps: [usize; N]
//     }
    
//     impl<const N: usize> TestAgent<N> {
//         fn new(steps: [usize; N]) -> Self { Self { steps } }
//     }
    

//     struct Iter<const N: usize> {
//         steps: [usize; N],
//         n: usize,
//         i: usize,
//     }
    
//     impl<const N: usize> Iter<N> {
//         fn new(steps: [usize; N], n: usize) -> Self {
//             Self { steps, n, i: 0 }
//         }
//     }

//     impl<const N: usize> Iterator for Iter<N> {
//         type Item = (usize, u32, SIter);
    
//         fn next(&mut self) -> Option<Self::Item> {
//             let j = self.i;
//             if j < self.steps.len() {
//                 let d = self.steps[j];
//                 self.i += 1;
//                 return Some((self.n + d, d as u32, SIter::new_up(self.n, d)))
//             }
//             for j in self.i - self.steps.len()..self.steps.len() {
//                 let d = self.steps[j];
//                 self.i += 1;
//                 if self.n > d {
//                     return Some((self.n - d, d as u32, SIter::new_down(self.n, d)))
//                 }
//             }
//             None
//         }
//     }

//     struct SIter {
//         n: usize,
//         k: usize,
//         step: bool,
//         i: usize
//     }
    
//     impl SIter {
//         fn new_up(n: usize, k: usize) -> Self {
//             Self { n, k, step: true, i: 0 }
//         }
//         fn new_down(n: usize, k: usize) -> Self {
//             Self { n, k, step: false, i: 0 }
//         }
//         fn value(&self, j: usize) -> usize {
//             if self.step {
//                 self.n + j
//             } else {
//                 self.n - j
//             }
//         }
//     }

//     impl Iterator for SIter {
//         type Item = (usize, u64);
        
//         fn next(&mut self) -> Option<Self::Item> {
//             let j = self.i;
//             if j < self.k {
//                 self.i += 1;
//                 Some((self.value(j), self.i as u64))
//             } else if j == self.k {
//                 self.i += 1;
//                 Some((self.value(j), u64::MAX))
//             } else {
//                 None
//             }
//         }   
//     }

//     impl<const N: usize> Agent for TestAgent<N> {

//         type N = usize;
//         type C = u32;
//         type S = usize;

//         type Iter = Iter<N>;
//         type SIter = SIter;
//         type FIter = IntoIter<usize>;

//         fn successors(&self, n: &usize) -> Self::Iter {
//             Self::Iter::new(self.steps, *n)
//         }

//         fn seats(&self, &n: &usize) -> Self::FIter {
//             vec![n].into_iter()
//         }
//     }

//     #[test]
//     fn test1() {

//         let a = TestAgent::new([7, 13]);

//         let end = MultipleEnds::new(&vec![HashSet::from([4])]);

//         let suc = |n: &_| a.successors(n);

//         let path = dijkstra_for_next_reservation(0, &end, suc, |_| true, 20);

//         println!("{:?}", path);

//         let suc = |n: &_| a.successors(n).map(|(n, c, _)| (n, c));

//         let path = dijkstra_for_multiple_ends(&0, &end, suc);

//         println!("{:?}", path);
//     }
// }
