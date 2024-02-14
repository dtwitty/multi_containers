use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;

trait Set {
    type Elem;
    fn new() -> Self;
    fn insert(&mut self, value: Self::Elem);
    fn remove(&mut self, value: &Self::Elem) -> bool;
    fn contains(&self, value: &Self::Elem) -> bool;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

struct HashTableSet<T> {
    data: HashSet<T>,
}

impl<T: Hash + Eq> Set for HashTableSet<T> {
    type Elem = T;
    fn new() -> Self {
        HashTableSet {
            data: HashSet::new(),
        }
    }

    fn insert(&mut self, value: Self::Elem) {
        self.data.insert(value);
    }

    fn remove(&mut self, value: &Self::Elem) -> bool {
        self.data.remove(value)
    }

    fn contains(&self, value: &Self::Elem) -> bool {
        self.data.contains(value)
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

struct TreeSet<T> {
    data: BTreeSet<T>,
}

impl<T: Ord> Set for TreeSet<T> {
    type Elem = T;
    fn new() -> Self {
        TreeSet {
            data: BTreeSet::new(),
        }
    }

    fn insert(&mut self, value: Self::Elem) {
        self.data.insert(value);
    }

    fn remove(&mut self, value: &Self::Elem) -> bool {
        self.data.remove(value)
    }

    fn contains(&self, value: &Self::Elem) -> bool {
        self.data.contains(value)
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}


trait Map<> {
    type Key;
    type Val;
    fn new() -> Self;
    fn insert(&mut self, key: Self::Key, value: Self::Val);
    fn get(&self, key: &Self::Key) -> Option<&Self::Val>;
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Val>;
    fn get_or_insert<F: FnOnce() -> Self::Val>(&mut self, key: Self::Key, make_value: F) -> &mut Self::Val;
    fn remove(&mut self, key: &Self::Key) -> bool;
    fn contains(&self, key: &Self::Key) -> bool;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

struct HashTableMap<K, V> {
    data: HashMap<K, V>,
}

impl<K: Hash + Eq, V> Map for HashTableMap<K, V> {
    type Key = K;
    type Val = V;
    
    fn new() -> Self {
        HashTableMap {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, key: Self::Key, value: Self::Val) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Val> {
        self.data.get(key)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Val> {
        self.data.get_mut(key)
    }

    fn get_or_insert<F: FnOnce() -> Self::Val>(&mut self, key: Self::Key, make_value: F) -> &mut Self::Val {
        self.data.entry(key).or_insert_with(make_value)
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.data.remove(key).is_some()
    }

    fn contains(&self, key: &Self::Key) -> bool {
        self.data.contains_key(key)
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

struct TreeMap<K, V> {
    data: BTreeMap<K, V>,
}

impl<K: Ord, V> Map for TreeMap<K, V> {
    type Key = K;
    type Val = V;
    
    fn new() -> Self {
        TreeMap {
            data: BTreeMap::new(),
        }
    }

    fn insert(&mut self, key: Self::Key, value: Self::Val) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Val> {
        self.data.get(key)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Val> {
        self.data.get_mut(key)
    }

    fn get_or_insert<F: FnOnce() -> Self::Val>(&mut self, key: Self::Key, make_value: F) -> &mut Self::Val {
        self.data.entry(key).or_insert_with(make_value)
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.data.remove(key).is_some()
    }

    fn contains(&self, key: &Self::Key) -> bool {
        self.data.contains_key(key)
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

struct MultiMap<M> {
    data: M,
}

impl<K, V, S, M> MultiMap<M> where M: Map<Key=K, Val=S>, S: Set<Elem=V> {
    fn new() -> Self {
        MultiMap {
            data: M::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let set = self.data.get_or_insert(key, || S::new());
        set.insert(value);
    }

    fn remove(&mut self, key: &K, value: &V) -> bool {
        if let Some(set) = self.data.get_mut(key) {
            if set.remove(value) {
                if set.is_empty() {
                    self.data.remove(key);
                }
                return true;
            }
        }
        false
    }

    fn contains(&self, key: &K, value: &V) -> bool {
        if let Some(set) = self.data.get(key) {
            set.contains(value)
        } else {
            false
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_multi_map() {
        let mut multi_map: MultiMap<HashTableMap<_, HashTableSet<_>>> = MultiMap::new();
        multi_map.insert(1, 1);
        multi_map.insert(1, 2);
        multi_map.insert(2, 1);
        multi_map.insert(2, 2);
        assert_eq!(multi_map.len(), 2);
        assert_eq!(multi_map.contains(&1, &1), true);
        assert_eq!(multi_map.contains(&1, &2), true);
        assert_eq!(multi_map.contains(&2, &1), true);
        assert_eq!(multi_map.contains(&2, &2), true);
        assert_eq!(multi_map.contains(&3, &1), false);
        assert_eq!(multi_map.contains(&3, &2), false);
        assert_eq!(multi_map.contains(&4, &1), false);
        assert_eq!(multi_map.contains(&4, &2), false);
        multi_map.remove(&1, &1);
        multi_map.remove(&1, &2);
        multi_map.remove(&2, &1);
        multi_map.remove(&2, &2);
        assert_eq!(multi_map.len(), 0);
    }

    #[test]
    fn test_sorted_multi_map() {
        let mut multi_map: MultiMap<TreeMap<_, TreeSet<_>>> = MultiMap::new();
        multi_map.insert(1, 1);
        multi_map.insert(1, 2);
        multi_map.insert(2, 1);
        multi_map.insert(2, 2);
        assert_eq!(multi_map.len(), 2);
        assert_eq!(multi_map.contains(&1, &1), true);
        assert_eq!(multi_map.contains(&1, &2), true);
        assert_eq!(multi_map.contains(&2, &1), true);
        assert_eq!(multi_map.contains(&2, &2), true);
        assert_eq!(multi_map.contains(&3, &1), false);
        assert_eq!(multi_map.contains(&3, &2), false);
        assert_eq!(multi_map.contains(&4, &1), false);
        assert_eq!(multi_map.contains(&4, &2), false);
        multi_map.remove(&1, &1);
        multi_map.remove(&1, &2);
        multi_map.remove(&2, &1);
        multi_map.remove(&2, &2);
        assert_eq!(multi_map.len(), 0);
    }
}
