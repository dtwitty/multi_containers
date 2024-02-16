mod hash_table_set;
mod tree_set;

use std::fmt::Debug;
use std::ops::RangeBounds;
pub use self::hash_table_set::HashTableSet;
pub use self::tree_set::TreeSet;

/// A set of elements.
pub trait Set : Eq + Debug + Clone + Default {
    /// The type of elements in the set.
    type Elem;

    /// The type of iterator over the elements of the set.
    type Iter<'a>: Iterator<Item=&'a Self::Elem> where Self: 'a;

    /// Inserts a value into the set. Returns `true` if the value was not already present.
    fn insert(&mut self, value: Self::Elem) -> bool;

    /// Removes a value from the set. Returns `true` if the value was present.
    fn remove(&mut self, value: &Self::Elem) -> bool;

    /// Returns `true` if the set contains the given value.
    fn contains(&self, value: &Self::Elem) -> bool;

    /// Returns `true` if the set is empty.
    fn is_empty(&self) -> bool;

    /// Returns the number of elements in the set.
    fn len(&self) -> usize;

    /// Returns an iterator over the elements of the set.
    fn iter(&self) -> Self::Iter<'_>;
}

pub trait SortedSet: Set {
    /// The type of iterator over the elements of the set within a range.
    type RangeIter<'a>: Iterator<Item=&'a Self::Elem> where Self: 'a;

    /// Returns an iterator over the elements of the set within the given range.
    fn range<R: RangeBounds<Self::Elem>>(&self, range: R) -> Self::RangeIter<'_>;
}
