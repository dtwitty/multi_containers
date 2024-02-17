use std::borrow::Borrow;
use std::collections::{btree_map, BTreeMap};
use std::fmt::{Debug, Formatter};
use std::ops::RangeBounds;
use crate::maps::{Lookup, Map, SortedMap};

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

impl<K, V, Q> SortedMap<Q> for TreeMap<K, V> where
    K: Ord + Borrow<Q>,
    Q: Ord + ?Sized
{
    type RangeIter<'a> = btree_map::Range<'a, K, V> where Self: 'a;
    type RangeIterMut<'a> = btree_map::RangeMut<'a, K, V> where Self: 'a;

    fn range<R>(&self, range: R) -> Self::RangeIter<'_> where R: RangeBounds<Q> {
        self.data.range(range)
    }

    fn range_mut<R>(&mut self, range: R) -> Self::RangeIterMut<'_> where R: RangeBounds<Q> {
        self.data.range_mut(range)
    }
}

impl<K, V, Q> Lookup<Q> for TreeMap<K, V> where K: Ord + Borrow<Q>, Q: Ord + ?Sized {
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
