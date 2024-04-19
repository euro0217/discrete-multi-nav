use num_traits::bounds::UpperBounded;
use trait_set::trait_set;

use crate::index::index::{Idx, IdxType};

trait_set! {
    pub trait AgentIdxType = IdxType + UpperBounded
}

pub trait Seat<T, U: AgentIdxType> {

    fn is_empty_for(&self, idx: Idx<T, U>) -> bool;
    fn add(&mut self, idx: Idx<T, U>);
    fn remove(&mut self, idx: Idx<T, U>);
}
