#![feature(impl_trait_in_assoc_type)]
#![warn(missing_docs)]


pub
mod sets;
mod maps;
mod builder;

use std::ops::RangeBounds;
use sets::Set;
use maps::Map;
use crate::maps::SortedMap;


pub trait MultiMap<'a> {
    type Key: 'a;
    type Val: 'a;
    type ValSet: Set<'a> + 'a;
    type Iter: Iterator<Item=(&'a Self::Key, &'a Self::ValSet)>;
    type FlatIter: Iterator<Item=(&'a Self::Key, &'a Self::Val)>;
    type KeyIter: Iterator<Item=&'a Self::Key>;
    type ValIter: Iterator<Item=&'a Self::Val>;

    fn insert(&mut self, key: Self::Key, value: Self::Val) -> bool;

    fn remove(&mut self, key: &Self::Key, value: &Self::Val) -> bool;

    fn contains(&self, key: &Self::Key, value: &Self::Val) -> bool;

    fn contains_key(&self, key: &Self::Key) -> bool;

    fn get(&self, key: &Self::Key) -> Option<&Self::ValSet>;

    fn keys(&'a self) -> Self::KeyIter;

    fn values(&'a self) -> Self::ValIter;

    fn iter(&'a self) -> Self::Iter;

    fn iter_flat(&'a self) -> Self::FlatIter;

    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;
}

pub trait SortedMultiMap<'a>: MultiMap<'a> {
    type RangeIter: Iterator<Item=(&'a Self::Key, &'a Self::ValSet)>;
    type FlatRangeIter<R>: Iterator<Item=(&'a Self::Key, &'a Self::Val)> where R: RangeBounds<Self::Key>, Self: 'a;
    fn range<R: RangeBounds<Self::Key>>(&'a self, range: R) -> Self::RangeIter;
    fn flat_range<R: RangeBounds<Self::Key>>(&'a self, range: R) -> Self::FlatRangeIter<R>;
}


#[derive(Debug, PartialEq, Eq)]
struct MultiMapImpl<M: Default> {
    data: M,
    len: usize,
}

impl <M: Default> MultiMapImpl<M> {
    fn new() -> Self {
        MultiMapImpl {
            data: M::default(),
            len: 0,
        }
    }
}

impl<'a, M> MultiMap<'a> for MultiMapImpl<M> where M: Map<'a> + Default + 'a, M::Val: Set<'a> + Default + 'a {
    type Key = M::Key;
    type Val = <<M as Map<'a>>::Val as Set<'a>>::Elem;
    type ValSet = M::Val;
    type Iter = M::Iter;
    type FlatIter = impl Iterator<Item=(&'a Self::Key, &'a Self::Val)>;
    type KeyIter = M::KeyIter;
    type ValIter = impl Iterator<Item=&'a Self::Val>;

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

    fn keys(&'a self) -> Self::KeyIter {
        self.data.keys()
    }

    fn values(&'a self) -> Self::ValIter {
        self.data.values().flat_map(|s| s.iter())
    }

    fn iter(&'a self) -> Self::Iter {
        self.data.iter()
    }

    fn iter_flat(&'a self) -> Self::FlatIter {
        self.iter().flat_map(|(k, s)| s.iter().map(move |v| (k, v)))
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, M> SortedMultiMap<'a> for MultiMapImpl<M> where M: SortedMap<'a> + Default + 'a, M::Val: Set<'a> + Default + 'a {
    type RangeIter = M::RangeIter;
    type FlatRangeIter<R> = impl Iterator<Item=(&'a Self::Key, &'a Self::Val)> where R: RangeBounds< Self::Key>, Self: 'a;

    fn range<R: RangeBounds<Self::Key>>(&'a self, range: R) -> Self::RangeIter {
        self.data.range(range)
    }

    fn flat_range<R: RangeBounds<Self::Key>>(&'a self, range: R) -> Self::FlatRangeIter<R> {
        self.range(range).flat_map(|(k, s)| s.iter().map(move |v| (k, v)))
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
    fn test_hash_multi_map_iter_flat() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![(&1, &2), (&1, &3), (&2, &3)];
        let mut actual = map.iter_flat().collect::<Vec<_>>();
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
    fn test_sorted_multi_map_iter_flat() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![(&1, &2), (&1, &3), (&2, &3)];
        let actual = map.iter_flat().collect::<Vec<_>>();
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

