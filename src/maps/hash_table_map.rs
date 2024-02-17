use std::borrow::Borrow;
use std::collections::{hash_map, HashMap};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use crate::maps::{Lookup, Map};

pub struct HashTableMap<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> HashTableMap<K, V> {
    pub fn new() -> Self {
        HashTableMap {
            data: HashMap::new(),
        }
    }
}

impl<K: Hash, V> Default for HashTableMap<K, V> {
    fn default() -> Self {
        HashTableMap::new()
    }
}

impl<K: Hash + Eq, V: PartialEq> PartialEq for HashTableMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl<K: Hash + Eq, V: Eq> Eq for HashTableMap<K, V> {}

impl<'a, K: Debug, V: Debug> Debug for HashTableMap<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<'a, K: Clone, V: Clone> Clone for HashTableMap<K, V> {
    fn clone(&self) -> Self {
        HashTableMap {
            data: self.data.clone(),
        }
    }
}


impl<K: Hash + Eq, V> Map for HashTableMap<K, V> {
    type Key = K;
    type Val = V;
    type Iter<'a> = hash_map::Iter<'a, K, V> where Self: 'a;
    type IterMut<'a> = hash_map::IterMut<'a, K, V> where Self: 'a;
    type KeyIter<'a> = hash_map::Keys<'a, K, V> where Self: 'a;
    type ValIter<'a> = hash_map::Values<'a, K, V> where Self: 'a;

    fn insert(&mut self, key: Self::Key, value: Self::Val) -> Option<Self::Val> {
        self.data.insert(key, value)
    }

    fn get_or_insert<F: FnOnce() -> Self::Val>(&mut self, key: Self::Key, make_value: F) -> &mut Self::Val {
        self.data.entry(key).or_insert_with(make_value)
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.data.iter_mut()
    }

    fn keys(&self) -> Self::KeyIter<'_> {
        self.data.keys()
    }

    fn values(&self) -> Self::ValIter<'_> {
        self.data.values()
    }
}

impl<K, V, Q> Lookup<Q> for HashTableMap<K, V> where K: Eq + Hash + Borrow<Q>, Q: Hash + Eq + ?Sized {
    fn contains_key(&self, key: &Q) -> bool {
        self.data.contains_key(key)
    }

    fn get(&self, key: &Q) -> Option<&V> {
        self.data.get(key)
    }

    fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
        self.data.get_mut(key)
    }

    fn remove(&mut self, key: &Q) -> bool {
        self.data.remove(key).is_some()
    }
}
