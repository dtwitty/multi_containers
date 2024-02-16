use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};
use std::ops::RangeBounds;
use crate::sets::{Set, SortedSet};

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


impl<T: Ord + Clone + Debug> Set for TreeSet<T> {
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

impl<T: Ord + Clone + Debug> SortedSet for TreeSet<T> {
    type RangeIter<'a> = impl Iterator<Item=&'a T> where Self: 'a;

    fn range<R: RangeBounds<Self::Elem>>(&self, range: R) -> Self::RangeIter<'_> {
        self.data.range(range)
    }
}
