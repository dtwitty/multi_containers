#![feature(impl_trait_in_assoc_type)]
#![warn(missing_docs)]


pub
mod sets;
mod maps;

use sets::{Set, HashTableSet, TreeSet};
use maps::{Map, HashTableMap, TreeMap};


trait MultiMap<'a> {
    type Key: 'a;
    type Val: 'a;
    type ValSet: Set<'a>;
    type Iter: Iterator<Item=(&'a Self::Key, &'a Self::Val)>;
    type KeyIter: Iterator<Item=&'a Self::Key>;
    type ValIter: Iterator<Item=&'a Self::Val>;

    fn new() -> Self;

    fn insert(&mut self, key: Self::Key, value: Self::Val) -> bool;

    fn remove(&mut self, key: &Self::Key, value: &Self::Val) -> bool;

    fn contains(&self, key: &Self::Key, value: &Self::Val) -> bool;

    fn contains_key(&self, key: &Self::Key) -> bool;

    fn get(&self, key: &Self::Key) -> Option<&Self::ValSet>;

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::ValSet>;

    fn keys(&'a self) -> Self::KeyIter;

    fn values(&'a self) -> Self::ValIter;

    fn iter(&'a self) -> Self::Iter;

    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;
}

struct MultiMapImpl<M> {
    data: M,
    len: usize,
}

impl<'a, M, > MultiMap<'a> for MultiMapImpl<M> where M: Map<'a> + 'a, M::Val: Set<'a> + Default + 'a {
    type Key = M::Key;
    type Val = <<M as Map<'a>>::Val as Set<'a>>::Elem;
    type ValSet = M::Val;
    type Iter = impl Iterator<Item=(&'a Self::Key, &'a Self::Val)>;
    type KeyIter = impl Iterator<Item=&'a Self::Key>;
    type ValIter = impl Iterator<Item=&'a Self::Val>;

    fn new() -> Self {
        MultiMapImpl {
            data: M::new(),
            len: 0,
        }
    }

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

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::ValSet> {
        self.data.get_mut(key)
    }

    fn keys(&'a self) -> Self::KeyIter {
        self.data.iter().map(|(k, _)| k)
    }

    fn values(&'a self) -> Self::ValIter {
        self.data.iter().flat_map(|(_, s)| s.iter())
    }

    fn iter(&'a self) -> Self::Iter {
        self.data.iter().flat_map(|(k, s)| s.iter().map(move |v| (k, v)))
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.len
    }
}

type SortedMultiMap<K, V> = MultiMapImpl<TreeMap<K, TreeSet<V>>>;
type HashMultiMap<K, V> = MultiMapImpl<HashTableMap<K, HashTableSet<V>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_multi_map_insert() {
        let mut map = HashMultiMap::<i32, i32>::new();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 2), false);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.insert(2, 3), false);
    }

    #[test]
    fn test_hash_multi_map_remove() {
        let mut map = HashMultiMap::<i32, i32>::new();
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
        let mut map = HashMultiMap::<i32, i32>::new();
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
        let mut map = HashMultiMap::<i32, i32>::new();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), true);
        assert_eq!(map.contains_key(&3), false);
    }

    #[test]
    fn test_hash_multi_map_get() {
        let mut map = HashMultiMap::<i32, i32>::new();
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
    fn test_hash_multi_map_get_mut() {
        let mut map = HashMultiMap::<i32, i32>::new();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.get_mut(&1).unwrap().insert(4), true);
        assert_eq!(map.get_mut(&1).unwrap().insert(4), false);
        assert_eq!(map.get_mut(&1).unwrap().remove(&4), true);
        assert_eq!(map.get_mut(&1).unwrap().remove(&4), false);
        assert_eq!(map.get_mut(&2).unwrap().insert(4), true);
        assert_eq!(map.get_mut(&2).unwrap().insert(4), false);
        assert_eq!(map.get_mut(&2).unwrap().remove(&4), true);
        assert_eq!(map.get_mut(&2).unwrap().remove(&4), false);
        assert_eq!(map.get_mut(&3), None);
    }

    #[test]
    fn test_hash_multi_map_keys() {
        let mut map = HashMultiMap::<i32, i32>::new();
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
        let mut map = HashMultiMap::<i32, i32>::new();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![&2, &3, &3];
        let mut actual = map.values().collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hash_multi_map_iter() {
        let mut map = HashMultiMap::<i32, i32>::new();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![(&1, &2), (&1, &3), (&2, &3)];
        let mut actual = map.iter().collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hash_multi_map_is_empty() {
        let mut map = HashMultiMap::<i32, i32>::new();
        assert_eq!(map.is_empty(), true);
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.is_empty(), false);
        assert_eq!(map.remove(&1, &2), true);
        assert_eq!(map.is_empty(), true);
    }

    #[test]
    fn test_hash_multi_map_len() {
        let mut map = HashMultiMap::<i32, i32>::new();
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
        let mut map = SortedMultiMap::<i32, i32>::new();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 2), false);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.insert(2, 3), false);
    }

    #[test]
    fn test_sorted_multi_map_remove() {
        let mut map = SortedMultiMap::<i32, i32>::new();
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
        let mut map = SortedMultiMap::<i32, i32>::new();
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
        let mut map = SortedMultiMap::<i32, i32>::new();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), true);
        assert_eq!(map.contains_key(&3), false);
    }

    #[test]
    fn test_sorted_multi_map_get() {
        let mut map = SortedMultiMap::<i32, i32>::new();
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
    fn test_sorted_multi_map_get_mut() {
        let mut map = SortedMultiMap::<i32, i32>::new();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        assert_eq!(map.get_mut(&1).unwrap().insert(4), true);
        assert_eq!(map.get_mut(&1).unwrap().insert(4), false);
        assert_eq!(map.get_mut(&1).unwrap().remove(&4), true);
        assert_eq!(map.get_mut(&1).unwrap().remove(&4), false);
        assert_eq!(map.get_mut(&2).unwrap().insert(4), true);
        assert_eq!(map.get_mut(&2).unwrap().insert(4), false);
        assert_eq!(map.get_mut(&2).unwrap().remove(&4), true);
        assert_eq!(map.get_mut(&2).unwrap().remove(&4), false);
        assert_eq!(map.get_mut(&3), None);
    }

    #[test]
    fn test_sorted_multi_map_keys() {
        let mut map = SortedMultiMap::<i32, i32>::new();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![&1, &2];
        let actual = map.keys().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_values() {
        let mut map = SortedMultiMap::<i32, i32>::new();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![&2, &3, &3];
        let actual = map.values().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_iter() {
        let mut map = SortedMultiMap::<i32, i32>::new();
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.insert(1, 3), true);
        assert_eq!(map.insert(2, 3), true);
        let expected = vec![(&1, &2), (&1, &3), (&2, &3)];
        let actual = map.iter().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_is_empty() {
        let mut map = SortedMultiMap::<i32, i32>::new();
        assert_eq!(map.is_empty(), true);
        assert_eq!(map.insert(1, 2), true);
        assert_eq!(map.is_empty(), false);
        assert_eq!(map.remove(&1, &2), true);
        assert_eq!(map.is_empty(), true);
    }

    #[test]
    fn test_sorted_multi_map_len() {
        let mut map = SortedMultiMap::<i32, i32>::new();
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
}
