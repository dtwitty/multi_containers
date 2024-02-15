use super::Set;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct HashTableSet<T> {
    data: HashSet<T>,
}

impl <T: Hash + Eq> HashTableSet<T> {
    pub fn new() -> Self {
        HashTableSet {
            data: HashSet::new(),
        }
    }
}

impl<'a, T: Hash + Eq + 'a> Set<'a> for HashTableSet<T> {
    type Elem = T;

    type Iter = impl Iterator<Item=&'a T> + 'a;

    fn insert(&mut self, value: Self::Elem) -> bool {
        self.data.insert(value)
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

impl <T: Hash + Eq> Default for HashTableSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl <T: Hash + Eq> PartialEq for HashTableSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl <T: Hash + Eq> Eq for HashTableSet<T> {}


