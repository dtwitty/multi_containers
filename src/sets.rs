use std::borrow::Borrow;
use std::collections::{btree_set, hash_set, BTreeSet, HashSet};
use std::hash::Hash;

/// A set of elements.
pub trait Set {
    /// The type of elements in the set.
    type Elem;

    /// The type of iterator over the elements of the set.
    type Iter<'a>: Iterator<Item = &'a Self::Elem>
    where
        Self: 'a;

    /// Inserts a value into the set. Returns `true` if the value was not already present.
    fn insert(&mut self, value: Self::Elem) -> bool;

    /// Returns `true` if the set is empty.
    fn is_empty(&self) -> bool;

    /// Returns the number of elements in the set.
    fn len(&self) -> usize;

    /// Returns an iterator over the elements of the set.
    fn iter(&self) -> Self::Iter<'_>;
}

pub trait Container<Q>: Set
where
    Q: ?Sized,
    Self::Elem: Borrow<Q>,
{
    /// Removes a value from the set. Returns `true` if the value was present.
    fn remove(&mut self, value: &Q) -> bool;

    /// Returns `true` if the set contains the given value.
    fn contains(&self, value: &Q) -> bool;
}

impl<T> Set for HashSet<T>
where
    T: Hash + Eq,
{
    type Elem = T;

    type Iter<'a> = hash_set::Iter<'a, T> where T: 'a;

    fn insert(&mut self, value: Self::Elem) -> bool {
        self.insert(value)
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

impl<T, Q> Container<Q> for HashSet<T>
where
    Q: Hash + Eq + ?Sized,
    T: Hash + Eq + Borrow<Q>,
{
    fn remove(&mut self, value: &Q) -> bool {
        self.remove(value)
    }

    fn contains(&self, value: &Q) -> bool {
        self.contains(value)
    }
}

impl<T> Set for BTreeSet<T>
where
    T: Ord,
{
    type Elem = T;
    type Iter<'a> = btree_set::Iter<'a, T> where T: 'a;

    fn insert(&mut self, value: Self::Elem) -> bool {
        self.insert(value)
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

impl<T, Q> Container<Q> for BTreeSet<T>
where
    Q: Ord + ?Sized,
    T: Ord + Borrow<Q>,
{
    fn remove(&mut self, value: &Q) -> bool {
        self.remove(value)
    }

    fn contains(&self, value: &Q) -> bool {
        self.contains(value)
    }
}
