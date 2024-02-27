use crate::maps::{Lookup, Map, SortedMap};
use std::borrow::Borrow;
use std::mem::replace;
use std::ops::RangeBounds;

/// A set that allows duplicate elements.
/// The set is implemented as a map from elements to their counts.
/// However, the API is designed to be more like a set,
/// including the ability to iterate over duplicate elements multiple times.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct MultiSet<M> {
    map: M,
    length: usize,
}

impl<M> MultiSet<M>
where
    M: Default,
{
    /// Creates a new empty multi-set.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiSet;
    /// let mut set = HashMultiSet::new();
    /// assert!(set.is_empty());
    /// set.insert(1);
    /// assert!(!set.is_empty());
    /// ```
    pub fn new() -> Self {
        MultiSet {
            map: M::default(),
            length: 0,
        }
    }
}

impl<M> MultiSet<M>
where
    M: Map<Val = usize>,
{
    /// Inserts a value into the multi-set.
    /// Returns the previous count of the value.
    /// If the value was not present, the previous count is 0.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiSet;
    /// let mut set = HashMultiSet::new();
    /// assert_eq!(set.insert(1), 0);
    /// assert_eq!(set.insert(1), 1);
    /// assert_eq!(set.insert(2), 0);
    /// assert_eq!(set.insert(2), 1);
    /// assert_eq!(set.insert(2), 2);
    /// ```
    pub fn insert(&mut self, value: M::Key) -> usize {
        self.insert_some(value, 1)
    }

    /// Inserts a value into the multi-set `count` times.
    /// Returns the previous count of the value.
    /// If the value was not present, the previous count is 0.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiSet;
    /// let mut set = HashMultiSet::new();
    /// assert_eq!(set.insert_some(1, 2), 0);
    /// assert_eq!(set.insert_some(1, 2), 2);
    /// assert_eq!(set.insert_some(2, 3), 0);
    /// assert_eq!(set.insert_some(2, 3), 3);
    /// assert_eq!(set.insert_some(2, 3), 6);
    /// ```
    pub fn insert_some(&mut self, value: M::Key, count: usize) -> usize {
        self.length += count;
        let have = self.map.get_or_insert(value, || 0_usize);
        replace(have, *have + count)
    }

    /// Sets the count of a value in the multi-set.
    /// Returns the previous count of the value.
    /// If the count is set to 0, the value is removed from the multi-set.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiSet;
    /// let mut set = HashMultiSet::new();
    /// assert_eq!(set.set_count(1, 2), 0);
    /// assert_eq!(set.set_count(1, 2), 2);
    /// assert_eq!(set.set_count(2, 3), 0);
    /// assert_eq!(set.set_count(2, 3), 3);
    /// assert_eq!(set.set_count(2, 0), 3);
    /// assert_eq!(set.set_count(2, 0), 0);
    /// ```
    pub fn set_count(&mut self, value: M::Key, count: usize) -> usize
    where
        M: Lookup<<M as Map>::Key>,
    {
        if count == 0 {
            return self.remove_all(&value);
        }

        let have = self.map.get_or_insert(value, || 0_usize);
        let prev = replace(have, count);
        self.length += *have;
        self.length -= prev;
        prev
    }

    /// Removes a value from the multi-set. Returns the previous count of the value.
    /// If the value was not present, the previous count is 0.
    /// If the count of a value is reduced to 0, the value is removed from the multi-set.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiSet;
    /// let mut set = HashMultiSet::new();
    /// set.insert_some(1, 2);
    /// set.insert_some(2, 3);
    /// assert_eq!(set.remove(&1), 2);
    /// assert_eq!(set.remove(&1), 1);
    /// assert_eq!(set.remove(&1), 0);
    /// assert_eq!(set.remove(&2), 3);
    /// assert_eq!(set.remove(&2), 2);
    /// assert_eq!(set.remove(&2), 1);
    /// ```
    pub fn remove<Q>(&mut self, value: &Q) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.remove_at_most(value, 1)
    }

    /// Removes at most `max` occurrences of a value from the multi-set.
    /// Returns the previous count of the value.
    /// If the value was not present, the previous count is 0.
    /// If the count of a value is reduced to 0, the value is removed from the multi-set.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiSet;
    /// let mut set = HashMultiSet::new();
    /// set.insert_some(1, 2);
    /// set.insert_some(2, 3);
    /// assert_eq!(set.remove_at_most(&1, 1), 2);
    /// assert_eq!(set.remove_at_most(&1, 1), 1);
    /// assert_eq!(set.remove_at_most(&2, 2), 3);
    /// assert_eq!(set.remove_at_most(&2, 2), 1);
    /// assert_eq!(set.remove_at_most(&2, 2), 0);
    /// ```
    pub fn remove_at_most<Q>(&mut self, value: &Q, max: usize) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        match self.map.get_mut(value) {
            Some(count) => {
                let prev = *count;
                let removed = (*count).min(max);
                *count -= removed;
                self.length -= removed;
                if *count == 0 {
                    self.map.remove(value);
                }
                prev
            }
            None => 0,
        }
    }

    /// Removes all occurrences of a value from the multi-set.
    /// Returns the previous count of the value.
    /// If the value was not present, the previous count is 0.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiSet;
    /// let mut set = HashMultiSet::new();
    /// set.insert_some(1, 2);
    /// set.insert_some(2, 3);
    /// assert_eq!(set.remove_all(&1), 2);
    /// assert_eq!(set.remove_all(&1), 0);
    /// assert_eq!(set.remove_all(&2), 3);
    /// assert_eq!(set.remove_all(&2), 0);
    /// ```
    pub fn remove_all<Q>(&mut self, value: &Q) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        match self.map.remove(value) {
            Some(count) => {
                self.length -= count;
                count
            }
            None => 0,
        }
    }

    /// Returns `true` if the multi-set contains the given value.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiSet;
    /// let mut set = HashMultiSet::new();
    /// set.insert_some(1, 2);
    /// set.insert_some(2, 3);
    /// assert_eq!(set.contains(&1), true);
    /// assert_eq!(set.contains(&2), true);
    /// assert_eq!(set.contains(&3), false);
    /// ```
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.contains_key(value)
    }

    /// Returns the number of occurrences of a value in the multi-set.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiSet;
    /// let mut set = HashMultiSet::new();
    /// set.insert_some(1, 2);
    /// set.insert_some(2, 3);
    /// assert_eq!(set.count(&1), 2);
    /// assert_eq!(set.count(&2), 3);
    /// assert_eq!(set.count(&3), 0);
    /// ```
    pub fn count<Q>(&self, value: &Q) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.get(value).copied().unwrap_or(0_usize)
    }

    /// Returns `true` if the multi-set is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiSet;
    /// let mut set = HashMultiSet::new();
    /// assert_eq!(set.is_empty(), true);
    /// set.insert(1);
    /// assert_eq!(set.is_empty(), false);
    /// set.remove(&1);
    /// assert_eq!(set.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the number of values in the multi-set, including duplicates.
    /// This is equal to the sum of the counts of all unique values.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiSet;
    /// let mut set = HashMultiSet::new();
    /// assert_eq!(set.len(), 0);
    /// set.insert(1);
    /// assert_eq!(set.len(), 1);
    /// set.insert(1);
    /// assert_eq!(set.len(), 2);
    /// set.insert(2);
    /// assert_eq!(set.len(), 3);
    /// set.remove(&1);
    /// assert_eq!(set.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns an iterator over the values of the multi-set, including duplicates.
    /// The iterator yields each value `count` times, where `count` is the number of occurrences of the value in the multi-set.
    /// The order of the values depends on the underlying map implementation.
    ///
    /// # Example
    ///
    /// ```
    /// // `BTreeMultiSet` is used here so that iterators are sorted.
    /// use multi_containers::BTreeMultiSet;
    /// let mut set = BTreeMultiSet::new();
    /// set.insert_some(1, 2);
    /// set.insert_some(2, 3);
    /// assert_eq!(set.iter().collect::<Vec<_>>(), vec![&1, &1, &2, &2, &2]);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &M::Key> {
        self.map
            .iter()
            .flat_map(|(k, &v)| std::iter::repeat(k).take(v))
    }

    /// Returns an iterator over the unique values of the multi-set, with their counts.
    /// The order of the values depends on the underlying map implementation.
    ///
    /// # Example
    ///
    /// ```
    /// // `BTreeMultiSet` is used here so that iterators are sorted.
    /// use multi_containers::BTreeMultiSet;
    /// let mut set = BTreeMultiSet::new();
    /// set.insert_some(1, 2);
    /// set.insert_some(2, 3);
    /// assert_eq!(set.counts().collect::<Vec<_>>(), vec![(&1, &2), (&2, &3)]);
    /// ```
    pub fn counts(&self) -> M::Iter<'_> {
        self.map.iter()
    }

    /// Returns an iterator over the entries of the multi-set within a given range, including duplicates.
    /// The iterator yields each value `count` times, where `count` is the number of occurrences of the value in the multi-set.
    /// Values will be yielded in sorted order, as this method is only available for sorted multisets.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::BTreeMultiSet;
    /// let mut set = BTreeMultiSet::new();
    /// set.insert_some(1, 2);
    /// set.insert_some(2, 3);
    /// set.insert_some(3, 4);
    /// set.insert_some(5, 6);
    /// assert_eq!(set.range(2..4).collect::<Vec<_>>(), vec![&2, &2, &2, &3, &3, &3, &3]);
    /// ```
    pub fn range<Q, R>(&self, range: R) -> impl Iterator<Item = &M::Key>
    where
        M: SortedMap<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
        R: RangeBounds<Q>,
    {
        self.map
            .range(range)
            .flat_map(|(k, &v)| std::iter::repeat(k).take(v))
    }

    /// Returns an iterator over the unique values of the multi-set within a given range, with their counts.
    /// Values will be yielded in sorted order, as this method is only available for sorted multisets.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::BTreeMultiSet;
    /// let mut set = BTreeMultiSet::new();
    /// set.insert_some(1, 2);
    /// set.insert_some(2, 3);
    /// set.insert_some(3, 4);
    /// set.insert_some(5, 6);
    /// assert_eq!(set.range_counts(2..4).collect::<Vec<_>>(), vec![(&2, &3), (&3, &4)]);
    /// ```
    pub fn range_counts<Q, R>(&self, range: R) -> M::RangeIter<'_>
    where
        M: SortedMap<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
        R: RangeBounds<Q>,
    {
        self.map.range(range)
    }
}

impl <T, M> Extend<T> for MultiSet<M>
where
    M: Map<Key = T, Val = usize>,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for value in iter {
            self.insert(value);
        }
    }
}

impl<T, M> FromIterator<T> for MultiSet<M>
where
    M: Map<Key = T, Val = usize> + Default,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut set = MultiSet::default();
        set.extend(iter);
        set
    }
}

impl<M, const N: usize> From<[M::Key; N]> for MultiSet<M>
where
    M: Map<Val = usize> + Default,
    M::Key: Clone,
{
    fn from(array: [M::Key; N]) -> Self {
        array.into_iter().collect::<Self>()
    }
}

#[cfg(test)]
mod tests {
    macro_rules! base_test_suite {
        ($mod_name:ident, $set_maker:expr) => {
            mod $mod_name {
                use crate::test_utils::unordered_elements_are;
                use crate::MultiSet;
                use crate::MultiSetBuilder;

                #[test]
                fn insert() {
                    let mut set = $set_maker;
                    assert_eq!(set.insert(1), 0);
                    assert_eq!(set.insert(1), 1);
                    assert_eq!(set.insert(2), 0);
                    assert_eq!(set.insert(2), 1);
                    assert_eq!(set.insert(2), 2);
                }

                #[test]
                fn insert_some() {
                    let mut set = $set_maker;
                    assert_eq!(set.insert_some(1, 2), 0);
                    assert_eq!(set.insert_some(1, 2), 2);
                    assert_eq!(set.insert_some(2, 3), 0);
                    assert_eq!(set.insert_some(2, 3), 3);
                    assert_eq!(set.insert_some(2, 3), 6);
                }

                #[test]
                fn set_count() {
                    let mut set = $set_maker;
                    assert_eq!(set.set_count(1, 2), 0);
                    assert_eq!(set.set_count(1, 2), 2);
                    assert_eq!(set.set_count(2, 3), 0);
                    assert_eq!(set.set_count(2, 3), 3);
                    assert_eq!(set.set_count(2, 3), 3);
                }

                #[test]
                fn remove() {
                    let mut set = $set_maker;
                    set.insert_some(1, 2);
                    set.insert_some(2, 3);
                    assert_eq!(set.remove(&1), 2);
                    assert_eq!(set.remove(&1), 1);
                    assert_eq!(set.remove(&1), 0);
                    assert_eq!(set.remove(&2), 3);
                    assert_eq!(set.remove(&2), 2);
                    assert_eq!(set.remove(&2), 1);
                }

                #[test]
                fn remove_at_most() {
                    let mut set = $set_maker;
                    set.insert_some(1, 2);
                    set.insert_some(2, 3);
                    assert_eq!(set.remove_at_most(&1, 1), 2);
                    assert_eq!(set.remove_at_most(&1, 1), 1);
                    assert_eq!(set.remove_at_most(&2, 2), 3);
                    assert_eq!(set.remove_at_most(&2, 2), 1);
                    assert_eq!(set.remove_at_most(&2, 2), 0);
                }

                #[test]
                fn remove_all() {
                    let mut set = $set_maker;
                    set.insert_some(1, 2);
                    set.insert_some(2, 3);
                    assert_eq!(set.remove_all(&1), 2);
                    assert_eq!(set.remove_all(&1), 0);
                    assert_eq!(set.remove_all(&2), 3);
                    assert_eq!(set.remove_all(&2), 0);
                }

                #[test]
                fn contains() {
                    let mut set = $set_maker;
                    set.insert_some(1, 2);
                    set.insert_some(2, 3);
                    assert!(set.contains(&1));
                    assert!(set.contains(&2));
                    assert!(!set.contains(&3));
                }

                #[test]
                fn count() {
                    let mut set = $set_maker;
                    set.insert_some(1, 2);
                    set.insert_some(2, 3);
                    assert_eq!(set.count(&1), 2);
                    assert_eq!(set.count(&2), 3);
                    assert_eq!(set.count(&3), 0);
                }

                #[test]
                fn is_empty() {
                    let mut set = $set_maker;
                    assert!(set.is_empty());
                    set.insert(1);
                    assert!(!set.is_empty());
                    set.remove(&1);
                    assert!(set.is_empty());
                }

                #[test]
                fn len() {
                    let mut set = $set_maker;
                    assert_eq!(set.len(), 0);
                    set.insert(1);
                    assert_eq!(set.len(), 1);
                    set.insert(1);
                    assert_eq!(set.len(), 2);
                    set.remove_all(&1);
                    assert_eq!(set.len(), 0);
                }

                #[test]
                fn iter() {
                    let mut set = $set_maker;
                    set.insert_some(1, 2);
                    set.insert_some(2, 3);
                    assert!(unordered_elements_are(
                        set.iter().cloned(),
                        vec![1, 1, 2, 2, 2]
                    ));
                }

                #[test]
                fn iter_counts() {
                    let mut set = $set_maker;
                    set.insert_some(1, 2);
                    set.insert_some(2, 3);
                    assert!(unordered_elements_are(
                        set.counts().map(|(k, v)| (k.clone(), v.clone())),
                        vec![(1, 2), (2, 3)]
                    ));
                }

                #[test]
                fn from_iter() {
                    let mut set = $set_maker;
                    set.insert_some(1, 2); // This line does nothing, but makes clippy happy.
                                           // Because we are in a macro, we don't actually know the type of the set.
                                           // So, we let the compiler infer it using the mut variable.
                    set = vec![1, 1, 2, 2, 2].into_iter().collect::<MultiSet<_>>();
                    assert!(unordered_elements_are(
                        set.iter().cloned(),
                        vec![1, 1, 2, 2, 2]
                    ));
                }

                #[test]
                fn from_array() {
                    let mut set = $set_maker;
                    set.insert_some(1, 2); // This line does nothing, but makes clippy happy.
                                           // Because we are in a macro, we don't actually know the type of the set.
                                           // So, we let the compiler infer it using the mut variable.
                    set = MultiSet::from([1, 1, 2, 2, 2]);
                    assert!(unordered_elements_are(
                        set.iter().cloned(),
                        vec![1, 1, 2, 2, 2]
                    ));
                }
            }
        };
    }

    macro_rules! sorted_test_suite {
        ($mod_name:ident, $set_maker:expr) => {
            mod $mod_name {
                use crate::test_utils::is_sorted;
                use crate::MultiSetBuilder;

                #[test]
                fn range() {
                    let mut set = $set_maker;
                    set.insert_some(1, 2);
                    set.insert_some(2, 3);
                    assert!(is_sorted(set.range(..).cloned()));
                    assert!(is_sorted(set.range(1..).cloned()));
                    assert!(is_sorted(set.range(..2).cloned()));
                    assert!(is_sorted(set.range(1..2).cloned()));
                }

                #[test]
                fn range_counts() {
                    let mut set = $set_maker;
                    set.insert_some(1, 2);
                    set.insert_some(2, 3);
                    assert!(is_sorted(
                        set.range_counts(..).map(|(k, v)| (k.clone(), v.clone()))
                    ));
                    assert!(is_sorted(
                        set.range_counts(1..).map(|(k, v)| (k.clone(), v.clone()))
                    ));
                    assert!(is_sorted(
                        set.range_counts(..2).map(|(k, v)| (k.clone(), v.clone()))
                    ));
                    assert!(is_sorted(
                        set.range_counts(1..2).map(|(k, v)| (k.clone(), v.clone()))
                    ));
                }
            }
        };
    }

    base_test_suite!(hash_values, MultiSetBuilder::hash_values().build());

    base_test_suite!(sorted_values, MultiSetBuilder::sorted_values().build());

    sorted_test_suite!(
        sorted_values_sorted,
        MultiSetBuilder::sorted_values().build()
    );
}
