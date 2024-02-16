mod hash_table_map;
mod tree_map;

use std::fmt::Debug;
use std::ops::RangeBounds;
pub use self::hash_table_map::HashTableMap;
pub use self::tree_map::TreeMap;

/// A map from keys to values.
pub trait Map: Eq + Debug + Default + Clone {
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
    fn get(&self, key: &Self::Key) -> Option<&Self::Val>;
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Val>;
    fn get_or_insert<F: FnOnce() -> Self::Val>(&mut self, key: Self::Key, make_value: F) -> &mut Self::Val;
    fn remove(&mut self, key: &Self::Key) -> bool;
    fn contains(&self, key: &Self::Key) -> bool;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn iter(&self) -> Self::Iter<'_>;
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
    fn keys(&self) -> Self::KeyIter<'_>;
    fn values(&self) -> Self::ValIter<'_>;
}

pub trait SortedMap: Map {
    type RangeIter<'a>: Iterator<Item=(&'a Self::Key, &'a Self::Val)> where Self: 'a;
    type RangeIterMut<'a>: Iterator<Item=(&'a Self::Key, &'a mut Self::Val)> where Self: 'a;
    fn range<R: RangeBounds<Self::Key>>(&self, range: R) -> Self::RangeIter<'_>;
    fn range_mut<R: RangeBounds<Self::Key>>(&mut self, range: R) -> Self::RangeIterMut<'_>;
}
