use std::borrow::Borrow;
use std::collections::{btree_map, BTreeMap, hash_map, HashMap};
use std::hash::Hash;
use std::ops::RangeBounds;

/// A map from keys to values.
pub trait Map {
    /// The type of keys in the map.
    type Key;

    /// The type of values in the map.
    type Val;

    /// The type of iterator over the entries of the map.
    type Iter<'a>: Iterator<Item=(&'a Self::Key, &'a Self::Val)> where Self: 'a;

    /// The type of iterator over the entries of the map, with mutable references to the values.
    type IterMut<'a>: Iterator<Item=(&'a Self::Key, &'a mut Self::Val)> where Self: 'a;

    /// The type of iterator over the keys of the map.
    type KeyIter<'a>: Iterator<Item=&'a Self::Key> where Self: 'a;

    /// The type of iterator over the values of the map.
    type ValIter<'a>: Iterator<Item=&'a Self::Val> where Self: 'a;

    fn insert(&mut self, key: Self::Key, value: Self::Val) -> Option<Self::Val>;
    fn get_or_insert<F: FnOnce() -> Self::Val>(&mut self, key: Self::Key, make_value: F) -> &mut Self::Val;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn iter(&self) -> Self::Iter<'_>;
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
    fn keys(&self) -> Self::KeyIter<'_>;
    fn values(&self) -> Self::ValIter<'_>;
}

pub trait Lookup<Q>: Map where Q: ?Sized, Self::Key: Borrow<Q> {
    fn contains_key(&self, key: &Q) -> bool;
    fn get(&self, key: &Q) -> Option<&Self::Val>;
    fn get_mut(&mut self, key: &Q) -> Option<&mut Self::Val>;
    fn remove(&mut self, key: &Q) -> bool;
}

pub trait SortedMap<Q>: Map where Q: ?Sized, Self::Key: Borrow<Q> {
    type RangeIter<'a>: Iterator<Item=(&'a Self::Key, &'a Self::Val)> where Self: 'a;
    type RangeIterMut<'a>: Iterator<Item=(&'a Self::Key, &'a mut Self::Val)> where Self: 'a;
    fn range<R>(&self, range: R) -> Self::RangeIter<'_> where R: RangeBounds<Q>;
    fn range_mut<R>(&mut self, range: R) -> Self::RangeIterMut<'_> where R: RangeBounds<Q>;
}


impl<K: Hash + Eq, V> Map for HashMap<K, V> {
    type Key = K;
    type Val = V;
    type Iter<'a> = hash_map::Iter<'a, K, V> where Self: 'a;
    type IterMut<'a> = hash_map::IterMut<'a, K, V> where Self: 'a;
    type KeyIter<'a> = hash_map::Keys<'a, K, V> where Self: 'a;
    type ValIter<'a> = hash_map::Values<'a, K, V> where Self: 'a;

    fn insert(&mut self, key: Self::Key, value: Self::Val) -> Option<Self::Val> {
        self.insert(key, value)
    }

    fn get_or_insert<F: FnOnce() -> Self::Val>(&mut self, key: Self::Key, make_value: F) -> &mut Self::Val {
        self.entry(key).or_insert_with(make_value)
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }

    fn keys(&self) -> Self::KeyIter<'_> {
        self.keys()
    }

    fn values(&self) -> Self::ValIter<'_> {
        self.values()
    }
}

impl<K, V, Q> Lookup<Q> for HashMap<K, V> where K: Eq + Hash + Borrow<Q>, Q: Hash + Eq + ?Sized {
    fn contains_key(&self, key: &Q) -> bool {
        self.contains_key(key)
    }

    fn get(&self, key: &Q) -> Option<&V> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
        self.get_mut(key)
    }

    fn remove(&mut self, key: &Q) -> bool {
        self.remove(key).is_some()
    }
}

impl<K: Ord, V> Map for BTreeMap<K, V> {
    type Key = K;
    type Val = V;
    type Iter<'a> = btree_map::Iter<'a, K, V> where Self: 'a;
    type IterMut<'a> = btree_map::IterMut<'a, K, V> where Self: 'a;
    type KeyIter<'a> = btree_map::Keys<'a, K, V> where Self: 'a;
    type ValIter<'a> = btree_map::Values<'a, K, V> where Self: 'a;


    fn insert(&mut self, key: Self::Key, value: Self::Val) -> Option<Self::Val> {
        self.insert(key, value)
    }

    fn get_or_insert<F: FnOnce() -> Self::Val>(&mut self, key: Self::Key, make_value: F) -> &mut Self::Val {
        self.entry(key).or_insert_with(make_value)
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }

    fn keys(&self) -> Self::KeyIter<'_> {
        self.keys()
    }

    fn values(&self) -> Self::ValIter<'_> {
        self.values()
    }
}

impl<K, V, Q> SortedMap<Q> for BTreeMap<K, V> where
    K: Ord + Borrow<Q>,
    Q: Ord + ?Sized
{
    type RangeIter<'a> = btree_map::Range<'a, K, V> where Self: 'a;
    type RangeIterMut<'a> = btree_map::RangeMut<'a, K, V> where Self: 'a;

    fn range<R>(&self, range: R) -> Self::RangeIter<'_> where R: RangeBounds<Q> {
        self.range(range)
    }

    fn range_mut<R>(&mut self, range: R) -> Self::RangeIterMut<'_> where R: RangeBounds<Q> {
        self.range_mut(range)
    }
}

impl<K, V, Q> Lookup<Q> for BTreeMap<K, V> where K: Ord + Borrow<Q>, Q: Ord + ?Sized {
    fn contains_key(&self, key: &Q) -> bool {
        self.contains_key(key)
    }

    fn get(&self, key: &Q) -> Option<&V> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
        self.get_mut(key)
    }

    fn remove(&mut self, key: &Q) -> bool {
        self.remove(key).is_some()
    }
}
