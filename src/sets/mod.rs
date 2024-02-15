mod hash_table_set;

pub use self::hash_table_set::HashTableSet;

pub trait Set<'a> {
    /// The type of elements in the set.
    type Elem;

    /// The type of iterator over the elements of the set.
    type Iter: Iterator<Item=&'a Self::Elem> where Self::Elem: 'a;

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
    fn iter(&'a self) -> Self::Iter;
}
