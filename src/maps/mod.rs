mod hash_table_map;
mod tree_map;

use std::ops::RangeBounds;
pub use self::hash_table_map::HashTableMap;
pub use self::tree_map::TreeMap;

pub trait Map<'a> {
    type Key;
    type Val;
    type Iter: Iterator<Item=(&'a Self::Key, &'a Self::Val)> where Self::Key: 'a, Self::Val: 'a;
    type IterMut: Iterator<Item=(&'a Self::Key, &'a mut Self::Val)> where Self::Key: 'a, Self::Val: 'a;
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
}

pub trait SortedMap<'a>: Map<'a> {
    type RangeIter: Iterator<Item=(&'a Self::Key, &'a Self::Val)> + 'a where Self::Key: 'a, Self::Val: 'a;
    type RangeIterMut: Iterator<Item=(&'a Self::Key, &'a mut Self::Val)> + 'a where Self::Key: 'a, Self::Val: 'a;
    fn range<R: RangeBounds<Self::Key>>(&'a self, range: R) -> Self::RangeIter;
    fn range_mut<R: RangeBounds<Self::Key>>(&'a mut self, range: R) -> Self::RangeIterMut;
}
