use std::borrow::Borrow;
use std::collections::{btree_set, BTreeSet};
use std::fmt::{Debug, Formatter};
use crate::sets::{Container, Set};

pub struct TreeSet<T> {
    data: BTreeSet<T>,
}

impl<T> TreeSet<T> {
    pub fn new() -> Self {
        TreeSet {
            data: BTreeSet::new(),
        }
    }
}

impl<T> Default for TreeSet<T> {
    fn default() -> Self {
        TreeSet::new()
    }
}

impl<T: PartialEq> PartialEq for TreeSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl<T: Eq> Eq for TreeSet<T> {}

impl<T: Debug> Debug for TreeSet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<T: Clone> Clone for TreeSet<T> {
    fn clone(&self) -> Self {
        TreeSet {
            data: self.data.clone(),
        }
    }
}


impl<T: Ord > Set for TreeSet<T> {
    type Elem = T;
    type Iter<'a> = btree_set::Iter<'a, T> where T: 'a;

    fn insert(&mut self, value: Self::Elem) -> bool {
        self.data.insert(value)
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

impl <T, Q> Container<Q> for TreeSet<T> where Q: Ord + ?Sized, T: Ord + Borrow<Q> {
    fn remove(&mut self, value: &Q) -> bool {
        self.data.remove(value)
    }

    fn contains(&self, value: &Q) -> bool {
        self.data.contains(value)
    }
}

