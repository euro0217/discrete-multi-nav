use std::{any::type_name, cmp::Ordering, fmt::{Debug, Formatter}, hash::{Hash, Hasher}, marker::PhantomData};

use num_traits::Unsigned;
use trait_set::trait_set;

trait_set! {
    pub trait IdxType =  Copy + Unsigned
}

pub struct Idx<T, U: IdxType> {
    idx: U,
    _p: PhantomData<T>,
}

impl<T, U: IdxType> Idx<T, U> {
    pub fn new(idx: U) -> Self {
        Self { idx, _p: PhantomData }
    }

    pub fn value(&self) -> U { self.idx }
}

impl<T, U: IdxType> PartialEq for Idx<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

impl<T, U: IdxType> Eq for Idx<T, U> {}

impl<T, U: IdxType + PartialOrd> PartialOrd for Idx<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.idx.partial_cmp(&other.idx)
    }
}

impl<T, U: IdxType + Ord> Ord for Idx<T, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.idx.cmp(&other.idx)
    }
}

impl<T, U: IdxType + Hash> Hash for Idx<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}

impl<T, U: IdxType + Debug> Debug for Idx<T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = type_name::<T>();
        f.write_str(format!("Idx {{ {} {:?} }}", name, self.idx).as_str())
    }
}

impl<T, U: IdxType> Clone for Idx<T, U> {
    fn clone(&self) -> Self {
        Self { idx: self.idx, _p: PhantomData }
    }
}

impl<T, U: IdxType> Copy for Idx<T, U> {}

#[cfg(test)]
mod tests {
    use super::Idx;

    struct T {}

    #[test]
    fn test() {
        let i = Idx::<T, u32>::new(1);
        let j = Idx::<T, _>::new(1);
        let k = Idx::<T, _>::new(2);

        assert!(i == j);
        assert!(i != k);
        assert!(i < k);
    }
}
