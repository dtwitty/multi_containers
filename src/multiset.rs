use crate::maps::{Lookup, Map};
use std::borrow::Borrow;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct MultiSet<M> {
    map: M,
    length: usize,
}

impl<M> MultiSet<M>
where
    M: Map<Val = usize>,
{
    pub fn insert(&mut self, value: M::Key) -> usize {
        let count = self.map.get_or_insert(value, || 0_usize);
        let prev = *count;
        *count += 1;
        self.length += 1;
        prev
    }

    pub fn remove<Q>(&mut self, value: &Q) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.remove_at_most(value, 1)
    }

    pub fn remove_at_most<Q>(&mut self, value: &Q, max: usize) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        if let Some(count) = self.map.get_mut(value) {
            let prev = *count;
            let removed = (*count).min(max);
            *count -= removed;
            self.length -= removed;
            if *count == 0 {
                self.map.remove(value);
            }
            prev
        } else {
            0
        }
    }

    pub fn remove_all<Q>(&mut self, value: &Q) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        if let Some(count) = self.map.remove(value) {
            self.length -= count;
            return count
        }

        0
    }

    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.contains_key(value)
    }

    pub fn count<Q>(&self, value: &Q) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.get(value).copied().unwrap_or(0_usize)
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn iter(&self) -> impl Iterator<Item = &M::Key> {
        self.map
            .iter()
            .flat_map(|(k, &v)| std::iter::repeat(k).take(v))
    }

    pub fn iter_counts(&self) -> impl Iterator<Item = (&M::Key, &usize)> {
        self.map.iter()
    }
}
