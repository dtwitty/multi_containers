mod hash_table_map;
mod tree_map;

use std::ops::RangeBounds;
pub use self::hash_table_map::HashTableMap;
pub use self::tree_map::TreeMap;

/// A map from keys to values.
pub trait Map<'a> {

    /// The type of keys in the map.
    type Key: 'a;

    /// The type of values in the map.
    type Val: 'a;

    /// The type of iterator over the entries of the map.
    type Iter: Iterator<Item=(&'a Self::Key, &'a Self::Val)>;

    /// The type of iterator over the entries of the map, with mutable references to the values.
    type IterMut: Iterator<Item=(&'a Self::Key, &'a mut Self::Val)>;

    /// The type of iterator over the keys of the map.
    type KeyIter: Iterator<Item=&'a Self::Key> + 'a;

    /// The type of iterator over the values of the map.
    type ValIter: Iterator<Item=&'a Self::Val> + 'a;

    fn new() -> Self;
    fn insert(&mut self, key: Self::Key, value: Self::Val) -> Option<Self::Val>;
    fn get(&self, key: &Self::Key) -> Option<&Self::Val>;
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Val>;
    fn get_or_insert<F: FnOnce() -> Self::Val>(&mut self, key: Self::Key, make_value: F) -> &mut Self::Val;
    fn remove(&mut self, key: &Self::Key) -> bool;
    fn contains(&self, key: &Self::Key) -> bool;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn iter(&'a self) -> Self::Iter;
    fn iter_mut(&'a mut self) -> Self::IterMut;
    fn keys(&'a self) -> Self::KeyIter;
    fn values(&'a self) -> Self::ValIter;
}

pub trait SortedMap<'a>: Map<'a> {
    type RangeIter: Iterator<Item=(&'a Self::Key, &'a Self::Val)> + 'a where Self::Key: 'a, Self::Val: 'a;
    type RangeIterMut: Iterator<Item=(&'a Self::Key, &'a mut Self::Val)> + 'a where Self::Key: 'a, Self::Val: 'a;
    fn range<R: RangeBounds<Self::Key>>(&'a self, range: R) -> Self::RangeIter;
    fn range_mut<R: RangeBounds<Self::Key>>(&'a mut self, range: R) -> Self::RangeIterMut;
}
