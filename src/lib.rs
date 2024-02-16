#![feature(impl_trait_in_assoc_type)]
#![warn(missing_docs)]


pub
mod sets;
mod maps;
mod builder;

use std::fmt::{Debug, Formatter};
use std::ops::RangeBounds;
use sets::Set;
use maps::Map;
use crate::maps::SortedMap;


pub trait MultiMap {
    type Key;
    type Val;
    type ValSet: Set;
    type Iter<'a>: Iterator<Item=(&'a Self::Key, &'a Self::ValSet)> where Self: 'a;
    type FlatIter<'a>: Iterator<Item=(&'a Self::Key, &'a Self::Val)> where Self: 'a;
    type KeyIter<'a>: Iterator<Item=&'a Self::Key> where Self: 'a;
    type ValIter<'a>: Iterator<Item=&'a Self::Val> where Self: 'a;

    fn insert(&mut self, key: Self::Key, value: Self::Val) -> bool;

    fn remove(&mut self, key: &Self::Key, value: &Self::Val) -> bool;

    fn contains(&self, key: &Self::Key, value: &Self::Val) -> bool;

    fn contains_key(&self, key: &Self::Key) -> bool;

    fn get(&self, key: &Self::Key) -> Option<&Self::ValSet>;

    fn keys<'a>(&'a self) -> Self::KeyIter<'a>;

    fn values<'a>(&'a self) -> Self::ValIter<'a>;

    fn iter<'a>(&'a self) -> Self::Iter<'a>;

    fn flat_iter<'a>(&'a self) -> Self::FlatIter<'a>;

    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;
}

pub trait SortedMultiMap: MultiMap {
    type RangeIter<'a>: Iterator<Item=(&'a Self::Key, &'a Self::ValSet)> where Self: 'a;
    type FlatRangeIter<'a, R>: Iterator<Item=(&'a Self::Key, &'a Self::Val)> where R: RangeBounds<Self::Key>, Self: 'a;

    fn range<'a, R: RangeBounds<Self::Key>>(&'a self, range: R) -> Self::RangeIter<'a>;

    fn flat_range<'a, R: RangeBounds<Self::Key>>(&'a self, range: R) -> Self::FlatRangeIter<'a, R>;
}


struct MultiMapImpl<M> {
    data: M,
    len: usize,
}

impl<M: Default> MultiMapImpl<M> {
    fn new() -> Self {
        MultiMapImpl {
            data: M::default(),
            len: 0,
        }
    }
}

impl<M: Default> Default for MultiMapImpl<M> {
    fn default() -> Self {
        MultiMapImpl::new()
    }
}

impl<M: PartialEq> PartialEq for MultiMapImpl<M> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.data == other.data
    }
}

impl<M: Eq> Eq for MultiMapImpl<M> {}

impl<M: Clone> Clone for MultiMapImpl<M> {
    fn clone(&self) -> Self {
        MultiMapImpl {
            data: self.data.clone(),
            len: self.len,
        }
    }
}

impl<M: Debug> Debug for MultiMapImpl<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<M> MultiMap for MultiMapImpl<M> where M: Map, M::Val: Set {
    type Key = M::Key;
    type Val = <<M as Map>::Val as Set>::Elem;
    type ValSet = M::Val;
    type Iter<'a> = impl Iterator<Item=(&'a Self::Key, &'a Self::ValSet)> where Self: 'a;
    type FlatIter<'a> = impl Iterator<Item=(&'a Self::Key, &'a Self::Val)> where Self: 'a;
    type KeyIter<'a> = impl Iterator<Item=&'a Self::Key> where Self: 'a;
    type ValIter<'a> = impl Iterator<Item=&'a Self::Val> where Self: 'a;

    fn insert(&mut self, key: Self::Key, value: Self::Val) -> bool {
        let set = self.data.get_or_insert(key, || Default::default());
        let r = set.insert(value);
        if r {
            self.len += 1;
        }
        r
    }

    fn remove(&mut self, key: &Self::Key, value: &Self::Val) -> bool {
        if let Some(set) = self.data.get_mut(key) {
            if set.remove(value) {
                self.len -= 1;
                if set.is_empty() {
                    self.data.remove(key);
                }
                return true;
            }
        }
        false
    }

    fn contains(&self, key: &Self::Key, value: &Self::Val) -> bool {
        self.data.get(key).map_or(false, |set| set.contains(value))
    }

    fn contains_key(&self, key: &Self::Key) -> bool {
        self.data.contains(key)
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::ValSet> {
        self.data.get(key)
    }

    fn keys<'a>(&'a self) -> Self::KeyIter<'a> {
        self.data.keys()
    }

    fn values<'a>(&'a self) -> Self::ValIter<'a> {
        self.data.values().flat_map(|s| s.iter())
    }

    fn iter<'a>(&'a self) -> Self::Iter<'a> {
        self.data.iter()
    }

    fn flat_iter<'a>(&'a self) -> Self::FlatIter<'a> {
        self.data.iter().flat_map(|(k, s)| s.iter().map(move |v| (k, v)))
    }


    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl<M> SortedMultiMap for MultiMapImpl<M> where M: SortedMap + Default, M::Val: Set + Default {
    type RangeIter<'a> = impl Iterator<Item=(&'a Self::Key, &'a Self::ValSet)> where Self: 'a;
    type FlatRangeIter<'a, R> = impl Iterator<Item=(&'a Self::Key, &'a Self::Val)> where R: RangeBounds< Self::Key>, Self: 'a;

    fn range<'a, R: RangeBounds<Self::Key>>(&'a self, range: R) -> Self::RangeIter<'a> {
        self.data.range(range)
    }

    fn flat_range<'a, R: RangeBounds<Self::Key>>(&'a self, range: R) -> Self::FlatRangeIter<'a, R> {
        self.data.range(range).flat_map(|(k, s)| s.iter().map(move |v| (k, v)))
    }
}


#[cfg(test)]
mod tests {
    use crate::builder::MultiMapBuilder;
    use super::*;

    #[test]
    fn test_hash_multi_map_insert() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 2), false);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.insert(2, 3), false);
    }

    #[test]
    fn test_hash_multi_map_remove() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.remove(&1, &2), true);
        assert_eq!(map.remove(&1, &2), false);
        assert_eq!(map.remove(&1, &3), true);
        assert_eq!(map.remove(&1, &3), false);
        assert_eq!(map.remove(&2, &3), true);
        assert_eq!(map.remove(&2, &3), false);
    }

    #[test]
    fn test_hash_multi_map_contains() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.contains(&1, &2), true);
        assert_eq!(map.contains(&1, &3), true);
        assert_eq!(map.contains(&2, &3), true);
        assert_eq!(map.contains(&1, &4), false);
        assert_eq!(map.contains(&2, &4), false);
    }

    #[test]
    fn test_hash_multi_map_contains_key() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), true);
        assert_eq!(map.contains_key(&3), false);
    }

    #[test]
    fn test_hash_multi_map_get() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.get(&1).unwrap().contains(&2), true);
        assert_eq!(map.get(&1).unwrap().contains(&3), true);
        assert_eq!(map.get(&2).unwrap().contains(&3), true);
        assert_eq!(map.get(&1).unwrap().contains(&4), false);
        assert_eq!(map.get(&2).unwrap().contains(&4), false);
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_hash_multi_map_keys() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![&1, &2];
        let mut actual = map.keys().collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hash_multi_map_values() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![&2, &3, &3];
        let mut actual = map.values().collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hash_multi_map_flat_iter() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![(&1, &2), (&1, &3), (&2, &3)];
        let mut actual = map.flat_iter().collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hash_multi_map_is_empty() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert_eq!(map.is_empty(), true);
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.is_empty(), false);
        assert_eq!(map.remove(&1, &2), true);
        assert_eq!(map.is_empty(), true);
    }

    #[test]
    fn test_hash_multi_map_len() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert_eq!(map.len(), 0);
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.len(), 1);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.len(), 2);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.len(), 3);
        assert_eq!(map.remove(&1, &2), true);
        assert_eq!(map.len(), 2);
        assert_eq!(map.remove(&1, &3), true);
        assert_eq!(map.len(), 1);
        assert_eq!(map.remove(&2, &3), true);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_sorted_multi_map_insert() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 2), false);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.insert(2, 3), false);
    }

    #[test]
    fn test_sorted_multi_map_remove() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.remove(&1, &2), true);
        assert_eq!(map.remove(&1, &2), false);
        assert_eq!(map.remove(&1, &3), true);
        assert_eq!(map.remove(&1, &3), false);
        assert_eq!(map.remove(&2, &3), true);
        assert_eq!(map.remove(&2, &3), false);
    }

    #[test]
    fn test_sorted_multi_map_contains() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.contains(&1, &2), true);
        assert_eq!(map.contains(&1, &3), true);
        assert_eq!(map.contains(&2, &3), true);
        assert_eq!(map.contains(&1, &4), false);
        assert_eq!(map.contains(&2, &4), false);
    }

    #[test]
    fn test_sorted_multi_map_contains_key() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), true);
        assert_eq!(map.contains_key(&3), false);
    }

    #[test]
    fn test_sorted_multi_map_get() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.get(&1).unwrap().contains(&2), true);
        assert_eq!(map.get(&1).unwrap().contains(&3), true);
        assert_eq!(map.get(&2).unwrap().contains(&3), true);
        assert_eq!(map.get(&1).unwrap().contains(&4), false);
        assert_eq!(map.get(&2).unwrap().contains(&4), false);
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_sorted_multi_map_keys() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![&1, &2];
        let actual = map.keys().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_values() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![&2, &3, &3];
        let actual = map.values().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_flat_iter() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![(&1, &2), (&1, &3), (&2, &3)];
        let actual = map.flat_iter().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_is_empty() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.is_empty(), true);
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.is_empty(), false);
        assert_eq!(map.remove(&1, &2), true);
        assert_eq!(map.is_empty(), true);
    }

    #[test]
    fn test_sorted_multi_map_len() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.len(), 0);
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.len(), 1);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.len(), 2);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.len(), 3);
        assert_eq!(map.remove(&1, &2), true);
        assert_eq!(map.len(), 2);
        assert_eq!(map.remove(&1, &3), true);
        assert_eq!(map.len(), 1);
        assert_eq!(map.remove(&2, &3), true);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_sorted_multi_map_range() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.insert(3, 4), true);
        assert_eq!(map.insert(4, 5), true);

        let expected = vec![&3, &4];
        let actual = map.range(2..=3).flat_map(|(_, s)| s.iter()).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_flat_range() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![(&1, &2), (&1, &3)];
        let actual = map.flat_range(1..2).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
}

