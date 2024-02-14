use std::collections::{btree_map, btree_set, BTreeMap, BTreeSet, hash_map, hash_set, HashMap, HashSet};
use std::hash::Hash;


trait Set<'a> {
    type Elem;
    type Iter: Iterator<Item=&'a Self::Elem> where Self::Elem: 'a;
    fn new() -> Self;
    fn insert(&mut self, value: Self::Elem);
    fn remove(&mut self, value: &Self::Elem) -> bool;
    fn contains(&self, value: &Self::Elem) -> bool;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn iter(&'a self) -> Self::Iter;
}

struct HashTableSet<T> {
    data: HashSet<T>,
}

impl<'a, T: Hash + Eq + 'a> Set<'a> for HashTableSet<T> {
    type Elem = T;
    type Iter = hash_set::Iter<'a, T>;
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

    fn iter(&'a self) -> Self::Iter {
        self.data.iter()
    }
}

struct TreeSet<T> {
    data: BTreeSet<T>,
}

impl<'a, T: Ord + 'a> Set<'a> for TreeSet<T> {
    type Elem = T;
    type Iter = btree_set::Iter<'a, T>;
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

    fn iter(&'a self) -> Self::Iter {
        self.data.iter()
    }
}


trait Map<'a> {
    type Key;
    type Val;
    type Iter: Iterator<Item=(&'a Self::Key, &'a Self::Val)> where Self::Key: 'a, Self::Val: 'a;
    fn new() -> Self;
    fn insert(&mut self, key: Self::Key, value: Self::Val);
    fn get(&self, key: &Self::Key) -> Option<&Self::Val>;
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Val>;
    fn get_or_insert<F: FnOnce() -> Self::Val>(&mut self, key: Self::Key, make_value: F) -> &mut Self::Val;
    fn remove(&mut self, key: &Self::Key) -> bool;
    fn contains(&self, key: &Self::Key) -> bool;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn iter(&'a self) -> Self::Iter;
}

struct HashTableMap<K, V> {
    data: HashMap<K, V>,
}

impl<'a, K: Hash + Eq + 'a, V: 'a> Map<'a> for HashTableMap<K, V> {
    type Key = K;
    type Val = V;
    type Iter = hash_map::Iter<'a, K, V>;

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

    fn iter(&'a self) -> Self::Iter {
        self.data.iter()
    }
}

struct TreeMap<K, V> {
    data: BTreeMap<K, V>,
}

impl<'a, K: Ord + 'a, V: 'a> Map<'a> for TreeMap<K, V> {
    type Key = K;
    type Val = V;
    type Iter = btree_map::Iter<'a, K, V>;

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

    fn iter(&'a self) -> Self::Iter {
        self.data.iter()
    }
}

struct MultiMap<M> {
    data: M,
}

impl<'a, M> MultiMap<M> where M: Map<'a> + 'a, M::Val: Set<'a> + 'a {
    fn new() -> Self {
        MultiMap {
            data: M::new(),
        }
    }

    fn insert(&mut self, key: M::Key, value: <<M as Map<'a>>::Val as Set<'a>>::Elem) {
        let set = self.data.get_or_insert(key, || M::Val::new());
        set.insert(value);
    }

    fn remove(&mut self, key: &M::Key, value: &<<M as Map<'a>>::Val as Set<'a>>::Elem) -> bool {
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

    fn contains(&self, key: &M::Key, value: &<<M as Map<'a>>::Val as Set<'a>>::Elem) -> bool {
        self.data.get(key).map_or(false, |set| set.contains(value))
    }

    fn contains_key(&self, key: &M::Key) -> bool {
        self.data.contains(key)
    }

    fn get(&self, key: &M::Key) -> Option<&M::Val> {
        self.data.get(key)
    }

    fn get_mut(&mut self, key: &M::Key) -> Option<&mut M::Val> {
        self.data.get_mut(key)
    }

    fn keys(&'a self) -> impl Iterator<Item=&'a M::Key> {
        self.data.iter().map(|(k, _)| k)
    }

    fn values(&'a self) -> impl Iterator<Item=&'a <<M as Map<'a>>::Val as Set<'a>>::Elem> {
        self.data.iter().flat_map(|(_, s)| s.iter())
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn iter(&'a self) -> impl Iterator<Item=(&'a M::Key, &'a <<M as Map<'a>>::Val as Set<'a>>::Elem)> {
        self.data.iter().flat_map(|(k, s)| s.iter().map(move |v| (k, v)))
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
