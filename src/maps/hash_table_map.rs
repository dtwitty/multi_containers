use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use crate::maps::Map;

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


impl<K: Hash + Eq + Debug + Clone, V: Eq + Debug + Clone> Map for HashTableMap<K, V> {
    type Key = K;
    type Val = V;
    type Iter<'a> = impl Iterator<Item=(&'a K, &'a V)> where Self: 'a;
    type IterMut<'a> = impl Iterator<Item=(&'a K, &'a mut V)> where Self: 'a;
    type KeyIter<'a> = impl Iterator<Item=&'a K> where Self: 'a;
    type ValIter<'a> = impl Iterator<Item=&'a V> where Self: 'a;

    fn insert(&mut self, key: Self::Key, value: Self::Val) -> Option<Self::Val> {
        self.data.insert(key, value)
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
