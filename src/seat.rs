use num_traits::{bounds::UpperBounded, Unsigned};

use crate::index::index::Idx;

pub trait Seat<T, U: Unsigned + Copy + UpperBounded> {

    fn is_empty_for(&self, idx: Idx<T, U>) -> bool;
    fn add(&mut self, idx: Idx<T, U>);
    fn remove(&mut self, idx: Idx<T, U>);
}
