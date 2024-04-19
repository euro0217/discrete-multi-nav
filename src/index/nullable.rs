use num_traits::{bounds::UpperBounded, Unsigned};
use std::{fmt::{Debug, Formatter}, hash::{Hash, Hasher}};

use crate::seat::AgentIdxType;

use super::index::Idx;


pub struct Nullable<T, U: Unsigned + UpperBounded + Copy> {
    value: Idx<T, U>,
}

impl<T, U: Unsigned + UpperBounded + Copy> Nullable<T, U> {
    pub fn new(value: U) -> Self {
        Self { value: Idx::new(value) }
    }

    pub fn new_null() -> Self {
        Self { value: Idx::new(U::max_value()) }
    }

    pub fn is_null(&self) -> bool {
        self.value.value() == U::max_value()
    }

    pub fn value(&self) -> Option<Idx<T, U>> {
        if self.is_null() {
            None
        } else {
            Some(self.value)
        }
    }
}

impl<T, U: AgentIdxType + Hash> Hash for Nullable<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T, U: AgentIdxType + Debug> Debug for Nullable<T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            f.write_str("Idx {{ Null }}")
        } else {
            f.write_str(format!("{:?}", self).as_str())
        }
    }
}

impl<T, U: AgentIdxType> Clone for Nullable<T, U> {
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}

impl<T, U: AgentIdxType> Copy for Nullable<T, U> {}
