use std::{any::type_name, cmp::Ordering, fmt::{Debug, Formatter}, hash::{Hash, Hasher}, marker::PhantomData};

use num_traits::Unsigned;

pub struct Idx<T, U: Copy + Unsigned> {
    idx: U,
    _p: PhantomData<T>,
}

impl<T, U: Copy + Unsigned> Idx<T, U> {
    pub fn new(idx: U) -> Self {
        Self { idx, _p: PhantomData }
    }

    pub fn value(&self) -> U { self.idx }
}

impl<T, U: Copy + Unsigned> PartialEq for Idx<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

impl<T, U: Copy + Unsigned> Eq for Idx<T, U> {}

impl<T, U: Copy + Unsigned + PartialOrd> PartialOrd for Idx<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.idx.partial_cmp(&other.idx)
    }
}

impl<T, U: Copy + Unsigned + Ord> Ord for Idx<T, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.idx.cmp(&other.idx)
    }
}

impl<T, U: Copy + Unsigned + Hash> Hash for Idx<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}

impl<T, U: Copy + Unsigned + Debug> Debug for Idx<T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = type_name::<T>();
        f.write_str(format!("Idx {{ {} {:?} }}", name, self.idx).as_str())
    }
}

impl<T, U: Copy + Unsigned> Clone for Idx<T, U> {
    fn clone(&self) -> Self {
        Self { idx: self.idx, _p: PhantomData }
    }
}

impl<T, U: Copy + Unsigned> Copy for Idx<T, U> {}

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
