#![feature(impl_trait_in_assoc_type)]
#![warn(missing_docs)]


pub
mod sets;
mod maps;
mod builder;

use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::ops::RangeBounds;
use sets::Set;
use maps::Map;
use crate::maps::{Lookup, SortedMap};


struct MultiMap<M> {
    data: M,
    len: usize,
}

impl<M: Default> MultiMap<M> {
    fn new() -> Self {
        MultiMap {
            data: M::default(),
            len: 0,
        }
    }
}

impl<M: Default> Default for MultiMap<M> {
    fn default() -> Self {
        MultiMap::new()
    }
}

impl<M: PartialEq> PartialEq for MultiMap<M> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.data == other.data
    }
}

impl<M: Eq> Eq for MultiMap<M> {}

impl<M: Clone> Clone for MultiMap<M> {
    fn clone(&self) -> Self {
        MultiMap {
            data: self.data.clone(),
            len: self.len,
        }
    }
}

impl<M: Debug> Debug for MultiMap<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<M> MultiMap<M> where M: Map, M::Val: Set + Default {
    fn insert(&mut self, key: M::Key, value: <<M as Map>::Val as Set>::Elem) -> bool {
        let set = self.data.get_or_insert(key, Default::default);
        let r = set.insert(value);
        if r {
            self.len += 1;
        }
        r
    }

    fn remove<Q>(&mut self, key: &Q, value: &<<M as Map>::Val as Set>::Elem) -> bool where M: Lookup<Q>, M::Key: Borrow<Q>, Q: ?Sized {
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

    fn contains<Q>(&self, key: &Q, value: &<<M as Map>::Val as Set>::Elem) -> bool where M: Lookup<Q>, M::Key: Borrow<Q>, Q: ?Sized {
        self.data.get(key).map_or(false, |set| set.contains(value))
    }

    fn contains_key<Q>(&self, key: &Q) -> bool where M: Lookup<Q>, M::Key: Borrow<Q>, Q: ?Sized {
        self.data.contains_key(key)
    }

    fn get<Q>(&self, key: &Q) -> Option<&M::Val> where M: Lookup<Q>, M::Key: Borrow<Q>, Q: ?Sized {
        self.data.get(key)
    }

    fn keys(&self) -> M::KeyIter<'_> {
        self.data.keys()
    }

    fn values(&self) -> impl Iterator<Item=&<<M as Map>::Val as Set>::Elem> {
        self.data.values().flat_map(|s| s.iter())
    }

    fn iter(&self) -> M::Iter<'_> {
        self.data.iter()
    }

    fn flat_iter(&self) -> impl Iterator<Item=(&M::Key, &<<M as Map>::Val as Set>::Elem)> {
        self.iter().flat_map(|(k, s)| s.iter().map(move |v| (k, v)))
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl<M> MultiMap<M> where M: SortedMap + Default, M::Val: Set + Default {
    fn range<R: RangeBounds<M::Key>>(&self, range: R) -> impl Iterator<Item=(&M::Key, &M::Val)> {
        self.data.range(range)
    }

    fn flat_range<R: RangeBounds<M::Key>>(&self, range: R) -> impl Iterator<Item=(&M::Key, &<<M as Map>::Val as Set>::Elem)> {
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
        assert!(map.insert(1, 2));
        assert!(!map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(!map.insert(2, 3));
    }

    #[test]
    fn test_hash_multi_map_remove() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.remove(&1, &2));
        assert!(!map.remove(&1, &2));
        assert!(map.remove(&1, &3));
        assert!(!map.remove(&1, &3));
        assert!(map.remove(&2, &3));
        assert!(!map.remove(&2, &3));
    }

    #[test]
    fn test_hash_multi_map_contains() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.contains(&1, &2));
        assert!(map.contains(&1, &3));
        assert!(map.contains(&2, &3));
        assert!(!map.contains(&1, &4));
        assert!(!map.contains(&2, &4));
    }

    #[test]
    fn test_hash_multi_map_contains_key() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.contains_key(&1));
        assert!(map.contains_key(&2));
        assert!(!map.contains_key(&3));
    }

    #[test]
    fn test_hash_multi_map_get() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.get(&1).unwrap().contains(&2));
        assert!(map.get(&1).unwrap().contains(&3));
        assert!(map.get(&2).unwrap().contains(&3));
        assert!(!map.get(&1).unwrap().contains(&4));
        assert!(!map.get(&2).unwrap().contains(&4));
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_hash_multi_map_keys() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![&1, &2];
        let mut actual = map.keys().collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hash_multi_map_values() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![&2, &3, &3];
        let mut actual = map.values().collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hash_multi_map_flat_iter() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![(&1, &2), (&1, &3), (&2, &3)];
        let mut actual = map.flat_iter().collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hash_multi_map_is_empty() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert!(map.is_empty());
        assert!(map.insert(1, 2));
        assert!(!map.is_empty());
        assert!(map.remove(&1, &2));
        assert!(map.is_empty());
    }

    #[test]
    fn test_hash_multi_map_len() {
        let mut map = MultiMapBuilder::new().hash_values().hash_keys().build();
        assert_eq!(map.len(), 0);
        assert!(map.insert(1, 2));
        assert_eq!(map.len(), 1);
        assert!(map.insert(1, 3));
        assert_eq!(map.len(), 2);
        assert!(map.insert(2, 3));
        assert_eq!(map.len(), 3);
        assert!(map.remove(&1, &2));
        assert_eq!(map.len(), 2);
        assert!(map.remove(&1, &3));
        assert_eq!(map.len(), 1);
        assert!(map.remove(&2, &3));
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_sorted_multi_map_insert() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(!map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(!map.insert(2, 3));
    }

    #[test]
    fn test_sorted_multi_map_remove() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.remove(&1, &2));
        assert!(!map.remove(&1, &2));
        assert!(map.remove(&1, &3));
        assert!(!map.remove(&1, &3));
        assert!(map.remove(&2, &3));
        assert!(!map.remove(&2, &3));
    }

    #[test]
    fn test_sorted_multi_map_contains() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.contains(&1, &2));
        assert!(map.contains(&1, &3));
        assert!(map.contains(&2, &3));
        assert!(!map.contains(&1, &4));
        assert!(!map.contains(&2, &4));
    }

    #[test]
    fn test_sorted_multi_map_contains_key() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.contains_key(&1));
        assert!(map.contains_key(&2));
        assert!(!map.contains_key(&3));
    }

    #[test]
    fn test_sorted_multi_map_get() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.get(&1).unwrap().contains(&2));
        assert!(map.get(&1).unwrap().contains(&3));
        assert!(map.get(&2).unwrap().contains(&3));
        assert!(!map.get(&1).unwrap().contains(&4));
        assert!(!map.get(&2).unwrap().contains(&4));
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_sorted_multi_map_keys() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![&1, &2];
        let actual = map.keys().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_values() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![&2, &3, &3];
        let actual = map.values().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_flat_iter() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![(&1, &2), (&1, &3), (&2, &3)];
        let actual = map.flat_iter().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_is_empty() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert!(map.is_empty());
        assert!(map.insert(1, 2));
        assert!(!map.is_empty());
        assert!(map.remove(&1, &2));
        assert!(map.is_empty());
    }

    #[test]
    fn test_sorted_multi_map_len() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert_eq!(map.len(), 0);
        assert!(map.insert(1, 2));
        assert_eq!(map.len(), 1);
        assert!(map.insert(1, 3));
        assert_eq!(map.len(), 2);
        assert!(map.insert(2, 3));
        assert_eq!(map.len(), 3);
        assert!(map.remove(&1, &2));
        assert_eq!(map.len(), 2);
        assert!(map.remove(&1, &3));
        assert_eq!(map.len(), 1);
        assert!(map.remove(&2, &3));
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_sorted_multi_map_range() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.insert(3, 4));
        assert!(map.insert(4, 5));

        let expected = vec![&3, &4];
        let actual = map.range(2..=3).flat_map(|(_, s)| s.iter()).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_flat_range() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![(&1, &2), (&1, &3)];
        let actual = map.flat_range(1..2).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_borrowed_lookup_types() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        map.insert("a".to_string(), 1);
        assert!(map.contains_key("a"));
        let a: String = "a".to_string();
        assert!(map.contains_key(&a));
    }

    #[test]
    fn test_borrowed_key_types() {
        let mut map = MultiMapBuilder::new().sorted_values().sorted_keys().build();
        map.insert("a", 1);
        assert!(map.contains_key("a"));
        assert!(map.contains_key(&"a"));
        assert!(map.contains_key(&("a".to_string())[..]));
    }
}

