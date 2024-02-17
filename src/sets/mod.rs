mod hash_table_set;
mod tree_set;

use std::borrow::Borrow;
pub use self::hash_table_set::HashTableSet;
pub use self::tree_set::TreeSet;

/// A set of elements.
pub trait Set {
    /// The type of elements in the set.
    type Elem;

    /// The type of iterator over the elements of the set.
    type Iter<'a>: Iterator<Item=&'a Self::Elem> where Self: 'a;

    /// Inserts a value into the set. Returns `true` if the value was not already present.
    fn insert(&mut self, value: Self::Elem) -> bool;

    /// Returns `true` if the set is empty.
    fn is_empty(&self) -> bool;

    /// Returns the number of elements in the set.
    fn len(&self) -> usize;

    /// Returns an iterator over the elements of the set.
    fn iter(&self) -> Self::Iter<'_>;
}

pub trait Container<Q>: Set where Q: ?Sized, Self::Elem: Borrow<Q> {
    /// Removes a value from the set. Returns `true` if the value was present.
    fn remove(&mut self, value: &Q) -> bool;

    /// Returns `true` if the set contains the given value.
    fn contains(&self, value: &Q) -> bool;
}
