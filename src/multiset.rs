use crate::maps::{Lookup, Map, SortedMap};
use std::borrow::Borrow;
use std::ops::RangeBounds;

/// A set that allows duplicate elements.
/// The set is implemented as a map from elements to their counts. However, the API is designed to be more like a set,
/// including the ability to iterate over duplicate elements multiple times.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct MultiSet<M> {
    map: M,
    length: usize,
}

impl<M> MultiSet<M> {
    /// Creates a new multi-map with the given map and value set factory.
    /// This is an advanced method. If you don't have a good reason to use it, you probably want to use
    /// `MultiSetBuilder` instead.
    pub fn from_parts(map: M) -> Self {
        MultiSet { map, length: 0 }
    }
}

impl<M> MultiSet<M>
where
    M: Map<Val = usize>,
{
    /// Inserts a value into the multi-set. Returns the previous count of the value.
    pub fn insert(&mut self, value: M::Key) -> usize {
        let count = self.map.get_or_insert(value, || 0_usize);
        let prev = *count;
        *count += 1;
        self.length += 1;
        prev
    }

    /// Removes a value from the multi-set. Returns the previous count of the value.
    pub fn remove<Q>(&mut self, value: &Q) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.remove_at_most(value, 1)
    }

    /// Removes at most `max` occurrences of a value from the multi-set. Returns the previous count of the value.
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

    /// Removes all occurrences of a value from the multi-set. Returns the previous count of the value.
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
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.contains_key(value)
    }

    /// Returns the number of occurrences of a value in the multi-set.
    pub fn count<Q>(&self, value: &Q) -> usize
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.get(value).copied().unwrap_or(0_usize)
    }

    /// Returns `true` if the multi-set is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the number of values in the multi-set, including duplicates.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns an iterator over the values of the multi-set, including duplicates.
    pub fn iter(&self) -> impl Iterator<Item = &M::Key> {
        self.map
            .iter()
            .flat_map(|(k, &v)| std::iter::repeat(k).take(v))
    }

    /// Returns an iterator over the unique values of the multi-set, with their counts.
    pub fn iter_counts(&self) -> M::Iter<'_> {
        self.map.iter()
    }

    /// Returns an iterator over the entries of the multi-set within a given range, including duplicates.
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

#[cfg(test)]
mod tests {
    macro_rules! base_test_suite {
        ($mod_name:ident, $map_maker:expr) => {
            mod $mod_name {
                use crate::test_utils::unordered_elements_are;
                use crate::MultiSetBuilder;

                #[test]
                fn insert() {
                    let mut set = $map_maker;
                    assert_eq!(set.insert(1), 0);
                    assert_eq!(set.insert(1), 1);
                    assert_eq!(set.insert(2), 0);
                    assert_eq!(set.insert(2), 1);
                    assert_eq!(set.insert(2), 2);
                }

                #[test]
                fn remove() {
                    let mut set = $map_maker;
                    set.insert(1);
                    set.insert(1);
                    set.insert(2);
                    set.insert(2);
                    set.insert(2);
                    assert_eq!(set.remove(&1), 2);
                    assert_eq!(set.remove(&1), 1);
                    assert_eq!(set.remove(&1), 0);
                    assert_eq!(set.remove(&2), 3);
                    assert_eq!(set.remove(&2), 2);
                    assert_eq!(set.remove(&2), 1);
                }

                #[test]
                fn remove_at_most() {
                    let mut set = $map_maker;
                    set.insert(1);
                    set.insert(1);
                    set.insert(2);
                    set.insert(2);
                    set.insert(2);
                    assert_eq!(set.remove_at_most(&1, 1), 2);
                    assert_eq!(set.remove_at_most(&1, 1), 1);
                    assert_eq!(set.remove_at_most(&2, 2), 3);
                    assert_eq!(set.remove_at_most(&2, 2), 1);
                    assert_eq!(set.remove_at_most(&2, 2), 0);
                }

                #[test]
                fn remove_all() {
                    let mut set = $map_maker;
                    set.insert(1);
                    set.insert(1);
                    set.insert(2);
                    set.insert(2);
                    set.insert(2);
                    assert_eq!(set.remove_all(&1), 2);
                    assert_eq!(set.remove_all(&1), 0);
                    assert_eq!(set.remove_all(&2), 3);
                    assert_eq!(set.remove_all(&2), 0);
                }

                #[test]
                fn contains() {
                    let mut set = $map_maker;
                    set.insert(1);
                    set.insert(1);
                    set.insert(2);
                    set.insert(2);
                    set.insert(2);
                    assert!(set.contains(&1));
                    assert!(set.contains(&2));
                    assert!(!set.contains(&3));
                }

                #[test]
                fn count() {
                    let mut set = $map_maker;
                    set.insert(1);
                    set.insert(1);
                    set.insert(2);
                    set.insert(2);
                    set.insert(2);
                    assert_eq!(set.count(&1), 2);
                    assert_eq!(set.count(&2), 3);
                    assert_eq!(set.count(&3), 0);
                }

                #[test]
                fn is_empty() {
                    let mut set = $map_maker;
                    assert!(set.is_empty());
                    set.insert(1);
                    assert!(!set.is_empty());
                    set.remove(&1);
                    assert!(set.is_empty());
                }

                #[test]
                fn len() {
                    let mut set = $map_maker;
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
                    let mut set = $map_maker;
                    set.insert(1);
                    set.insert(1);
                    set.insert(2);
                    set.insert(2);
                    set.insert(2);
                    assert!(unordered_elements_are(
                        set.iter().cloned(),
                        vec![1, 1, 2, 2, 2]
                    ));
                }

                #[test]
                fn iter_counts() {
                    let mut set = $map_maker;
                    set.insert(1);
                    set.insert(1);
                    set.insert(2);
                    set.insert(2);
                    set.insert(2);
                    assert!(unordered_elements_are(
                        set.iter_counts().map(|(k, v)| (k.clone(), v.clone())),
                        vec![(1, 2), (2, 3)]
                    ));
                }
            }
        };
    }

    macro_rules! sorted_test_suite {
        ($mod_name:ident, $map_maker:expr) => {
            mod $mod_name {
                use crate::test_utils::is_sorted;
                use crate::MultiSetBuilder;

                #[test]
                fn range() {
                    let mut set = $map_maker;
                    set.insert(1);
                    set.insert(1);
                    set.insert(2);
                    set.insert(2);
                    set.insert(2);
                    assert!(is_sorted(set.range(..).cloned()));
                    assert!(is_sorted(set.range(1..).cloned()));
                    assert!(is_sorted(set.range(..2).cloned()));
                    assert!(is_sorted(set.range(1..2).cloned()));
                }

                #[test]
                fn range_counts() {
                    let mut set = $map_maker;
                    set.insert(1);
                    set.insert(1);
                    set.insert(2);
                    set.insert(2);
                    set.insert(2);
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
