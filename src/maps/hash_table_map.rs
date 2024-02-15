use std::collections::HashMap;
use std::hash::Hash;
use crate::maps::Map;

#[derive(Debug)]
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

impl <K: Hash, V> Default for HashTableMap<K, V> {
    fn default() -> Self {
        HashTableMap::new()
    }
}

impl <K: Hash + Eq, V: PartialEq> PartialEq for HashTableMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl <K: Hash + Eq, V: Eq> Eq for HashTableMap<K, V> {}


impl<'a, K: Hash + Eq + 'a, V: 'a> Map<'a> for HashTableMap<K, V> {
    type Key = K;
    type Val = V;
    type Iter = impl Iterator<Item=(&'a K, &'a V)> + 'a;
    type IterMut = impl Iterator<Item=(&'a K, &'a mut V)> + 'a;
    type KeyIter = impl Iterator<Item=&'a K> + 'a;
    type ValIter = impl Iterator<Item=&'a V> + 'a;

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

    fn iter(&'a self) -> Self::Iter {
        self.data.iter()
    }

    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.data.iter_mut()
    }

    fn keys(&'a self) -> Self::KeyIter {
        self.data.keys()
    }

    fn values(&'a self) -> Self::ValIter {
        self.data.values()
    }
}
