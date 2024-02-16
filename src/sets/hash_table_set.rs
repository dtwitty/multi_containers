use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use crate::sets::Set;

pub struct HashTableSet<T> {
    data: HashSet<T>,
}

impl<T> HashTableSet<T> {
    pub fn new() -> Self {
        HashTableSet {
            data: HashSet::new(),
        }
    }
}

impl<T> Default for HashTableSet<T> {
    fn default() -> Self {
        HashTableSet::new()
    }
}

impl<T: Clone> Clone for HashTableSet<T> {
    fn clone(&self) -> Self {
        HashTableSet {
            data: self.data.clone(),
        }
    }
}

impl<T: Hash + Eq> PartialEq for HashTableSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl<T: Hash + Eq> Eq for HashTableSet<T> {}

impl<T: Debug> Debug for HashTableSet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<T: Hash + Eq + Clone + Debug> Set for HashTableSet<T> {
    type Elem = T;

    type Iter<'a> = impl Iterator<Item=&'a T> where Self: 'a;

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

    fn iter(&self) -> Self::Iter<'_> {
        self.data.iter()
    }
}

