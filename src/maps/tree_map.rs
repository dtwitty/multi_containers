use std::collections::BTreeMap;
use crate::maps::{Map, SortedMap};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeMap<K, V> {
    data: BTreeMap<K, V>,
}

impl<'a, K: Ord + 'a, V: 'a> Map<'a> for TreeMap<K, V> {
    type Key = K;
    type Val = V;
    type Iter = impl Iterator<Item=(&'a K, &'a V)> + 'a;
    type IterMut = impl Iterator<Item=(&'a K, &'a mut V)> + 'a;

    fn new() -> Self {
        TreeMap {
            data: BTreeMap::new(),
        }
    }

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
}

impl<'a, K: Ord + 'a, V: 'a> SortedMap<'a> for TreeMap<K, V> {
    type RangeIter = impl Iterator<Item=(&'a Self::Key, &'a Self::Val)> + 'a;
    type RangeIterMut = impl Iterator<Item=(&'a Self::Key, &'a mut Self::Val)> + 'a;

    fn range<R: std::ops::RangeBounds<Self::Key>>(&'a self, range: R) -> Self::RangeIter {
        self.data.range(range)
    }

    fn range_mut<R: std::ops::RangeBounds<Self::Key>>(&'a mut self, range: R) -> Self::RangeIterMut {
        self.data.range_mut(range)
    }
}
