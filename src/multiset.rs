use crate::maps::{Lookup, Map};
use std::borrow::Borrow;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct MultiSet<M> {
    map: M,
    length: usize,
}

impl<M> MultiSet<M> {
    /// Creates a new multi-map with the given map and value set factory.
    /// This is an advanced method. If you don't have a good reason to use it, you probably want to use
    /// `MultiSetBuilder` instead.
    pub fn from_parts(map: M) -> Self {
        MultiSet { map, length: 0 }
    }
}

impl<M> MultiSet<M>
where
    M: Map<Val = usize>,
{
    /// Inserts a value into the multi-set. Returns the previous count of the value.
    pub fn insert(&mut self, value: M::Key) -> usize {
        let count = self.map.get_or_insert(value, || 0_usize);
        let prev = *count;
        *count += 1;
        self.length += 1;
        prev
    }

    /// Removes a value from the multi-set. Returns the previous count of the value.
    pub fn remove<Q>(&mut self, value: &Q) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.remove_at_most(value, 1)
    }

    /// Removes at most `max` occurrences of a value from the multi-set. Returns the previous count of the value.
    pub fn remove_at_most<Q>(&mut self, value: &Q, max: usize) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        match self.map.get_mut(value) {
            Some(count) => {
                let prev = *count;
                let removed = (*count).min(max);
                *count -= removed;
                self.length -= removed;
                if *count == 0 {
                    self.map.remove(value);
                }
                prev
            }
            None => 0,
        }
    }

    /// Removes all occurrences of a value from the multi-set. Returns the previous count of the value.
    pub fn remove_all<Q>(&mut self, value: &Q) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        match self.map.remove(value) {
            Some(count) => {
                self.length -= count;
                count
            }
            None => 0,
        }
    }

    /// Returns `true` if the multi-set contains the given value.
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.contains_key(value)
    }

    /// Returns the number of occurrences of a value in the multi-set.
    pub fn count<Q>(&self, value: &Q) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.get(value).copied().unwrap_or(0_usize)
    }

    /// Returns `true` if the multi-set is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the number of values in the multi-set, including duplicates.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns an iterator over the values of the multi-set, including duplicates.
    pub fn iter(&self) -> impl Iterator<Item = &M::Key> {
        self.map
            .iter()
            .flat_map(|(k, &v)| std::iter::repeat(k).take(v))
    }

    /// Returns an iterator over the unique values of the multi-set, with their counts.
    pub fn iter_counts(&self) -> impl Iterator<Item = (&M::Key, &usize)> {
        self.map.iter()
    }
}
