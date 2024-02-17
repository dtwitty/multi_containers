use std::collections::{btree_map, BTreeMap};
use std::fmt::{Debug, Formatter};
use crate::maps::{Map, SortedMap};

pub struct TreeMap<K, V> {
    data: BTreeMap<K, V>,
}

impl<K: Ord, V> TreeMap<K, V> {
    pub fn new() -> Self {
        TreeMap {
            data: BTreeMap::new(),
        }
    }
}

impl<K: Ord, V> Default for TreeMap<K, V> {
    fn default() -> Self {
        TreeMap::new()
    }
}

impl<K: Ord, V: PartialEq> PartialEq for TreeMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl<K: Ord, V: Eq> Eq for TreeMap<K, V> {}

impl<K: Debug, V: Debug> Debug for TreeMap<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<K: Clone, V: Clone> Clone for TreeMap<K, V> {
    fn clone(&self) -> Self {
        TreeMap {
            data: self.data.clone(),
        }
    }
}

impl<K: Ord, V> Map for TreeMap<K, V> {
    type Key = K;
    type Val = V;
    type Iter<'a> = btree_map::Iter<'a, K, V> where Self: 'a;
    type IterMut<'a> = btree_map::IterMut<'a, K, V> where Self: 'a;
    type KeyIter<'a> = btree_map::Keys<'a, K, V> where Self: 'a;
    type ValIter<'a> = btree_map::Values<'a, K, V> where Self: 'a;


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

impl<K: Ord, V> SortedMap for TreeMap<K, V> {
    type RangeIter<'a> = btree_map::Range<'a, K, V> where Self: 'a;
    type RangeIterMut<'a> = btree_map::RangeMut<'a, K, V> where Self: 'a;

    fn range<R: std::ops::RangeBounds<Self::Key>>(&self, range: R) -> Self::RangeIter<'_> {
        self.data.range(range)
    }

    fn range_mut<R: std::ops::RangeBounds<Self::Key>>(&mut self, range: R) -> Self::RangeIterMut<'_> {
        self.data.range_mut(range)
    }
}
