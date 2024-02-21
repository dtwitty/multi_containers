use crate::maps::*;
use crate::sets::*;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::RangeBounds;

/// A multi-map from keys to values.
/// The primary semantics of a multi-map is that it can contain multiple values for a single key.
/// The multi-map is implemented as a managed map from keys to sets of values. For bookkeeping, the
/// value sets are queryable, but not modifiable except through the multi-map API.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct MultiMap<M, F> {
    map: M,
    value_set_factory: F,
    length: usize,
}

impl<M, F> MultiMap<M, F> {
    /// Creates a new multi-map with the given map and value set factory.
    pub fn from_parts(map: M, value_set_factory: F) -> Self {
        MultiMap {
            map,
            value_set_factory,
            length: 0,
        }
    }
}

impl<M, F> MultiMap<M, F>
where
    M: Map,
    M::Val: Set,
    F: Fn() -> M::Val,
{
    /// Inserts a (key, value) mapping into the multi-map. Returns `true` if it was not already present.
    pub fn insert(&mut self, key: M::Key, value: <<M as Map>::Val as Set>::Elem) -> bool {
        if self
            .map
            .get_or_insert(key, || (self.value_set_factory)())
            .insert(value)
        {
            self.length += 1;
            true
        } else {
            false
        }
    }

    /// Returns `true` if the multi-map contains the given (key, value) mapping.
    pub fn contains<Q, R>(&mut self, key: &Q, value: &R) -> bool
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
        M::Val: Container<R>,
        <<M as Map>::Val as Set>::Elem: Borrow<R>,
        R: ?Sized,
    {
        self.map.get(key).map_or(false, |set| set.contains(value))
    }

    /// Returns `true` if the multi-map contains any mapping with the given key.
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.contains_key(key)
    }

    /// Removes a (key, value) mapping from the multi-map. Returns `true` if it was present.
    pub fn remove<Q, R>(&mut self, key: &Q, value: &R) -> bool
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
        M::Val: Container<R>,
        <<M as Map>::Val as Set>::Elem: Borrow<R>,
        R: ?Sized,
    {
        if let Some(set) = self.map.get_mut(key) {
            if set.remove(value) {
                self.length -= 1;
                if set.is_empty() {
                    self.map.remove(key);
                }
                return true;
            }
        }
        false
    }

    /// Removes all mappings with the given key from the multi-map. Returns `true` if any mappings were present.
    pub fn remove_key<Q>(&mut self, key: &Q) -> bool
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.remove(key)
    }

    /// Returns a reference to the set of values for the given key, if there are any.
    pub fn get<Q>(&self, key: &Q) -> Option<&M::Val>
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.get(key)
    }

    /// Returns an iterator over the keys of the multi-map.
    pub fn keys(&self) -> M::KeyIter<'_> {
        self.map.keys()
    }

    /// Returns an iterator over the values of the multi-map.
    pub fn values(&self) -> impl Iterator<Item = &<<M as Map>::Val as Set>::Elem> {
        self.map.values().flat_map(|s| s.iter())
    }

    /// Returns an iterator over the keys and value sets in the multi-map.
    pub fn iter(&self) -> M::Iter<'_> {
        self.map.iter()
    }

    /// Returns an iterator over the keys and values in the multi-map.
    pub fn flat_iter(&self) -> impl Iterator<Item = (&M::Key, &<<M as Map>::Val as Set>::Elem)> {
        self.iter().flat_map(|(k, s)| s.iter().map(move |v| (k, v)))
    }

    /// Returns `true` if the multi-map is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the number of keys in the multi-map.
    pub fn num_keys(&self) -> usize {
        self.map.len()
    }

    /// Returns the number of (key, value) mappings in the multi-map.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns an iterator over the entries of the multi-map within a range of keys.
    pub fn range<Q, R>(&self, range: R) -> M::RangeIter<'_>
    where
        M: SortedMap<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
        R: RangeBounds<Q>,
    {
        self.map.range(range)
    }

    /// Returns an iterator over the entries of the multi-map within a range of keys, with mutable references to the values.
    pub fn flat_range<Q, R>(
        &self,
        range: R,
    ) -> impl Iterator<Item = (&M::Key, &<<M as Map>::Val as Set>::Elem)>
    where
        M: SortedMap<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
        R: RangeBounds<Q>,
    {
        self.map
            .range(range)
            .flat_map(|(k, s)| s.iter().map(move |v| (k, v)))
    }
}

#[cfg(test)]
mod tests {
    macro_rules! base_test_suite {
        ($mod_name:ident, $map_maker:expr) => {
            mod $mod_name {
                use super::unordered_elements_are;
                use crate::builder::MultiMapBuilder;

                #[test]
                fn test_insert() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(1, 2), false);
                    assert_eq!(map.insert(1, 3), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 3), false);
                }

                #[test]
                fn test_contains() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.contains(&1, &2), true);
                    assert_eq!(map.contains(&1, &3), false);
                    assert_eq!(map.contains(&2, &2), false);
                    assert_eq!(map.insert(1, 3), true);
                    assert_eq!(map.contains(&1, &3), true);
                }

                #[test]
                fn test_contains_key() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.contains_key(&1), true);
                    assert_eq!(map.contains_key(&2), false);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.contains_key(&2), true);
                }

                #[test]
                fn test_remove() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.remove(&1, &2), true);
                    assert_eq!(map.remove(&1, &2), false);
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(1, 3), true);
                    assert_eq!(map.remove(&1, &2), true);
                    assert_eq!(map.remove(&1, &3), true);
                    assert_eq!(map.remove(&1, &2), false);
                }

                #[test]
                fn test_remove_key() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.remove_key(&1), true);
                    assert_eq!(map.remove_key(&1), false);
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(1, 3), true);
                    assert_eq!(map.remove_key(&1), true);
                    assert_eq!(map.remove_key(&1), false);
                }

                #[test]
                fn test_get() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.get(&1).unwrap().contains(&2), true);
                    assert_eq!(map.get(&1).unwrap().contains(&3), false);
                    assert_eq!(map.get(&2), None);
                    assert_eq!(map.insert(1, 3), true);
                    assert_eq!(map.get(&1).unwrap().contains(&3), true);
                }

                #[test]
                fn test_keys() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert!(unordered_elements_are(map.keys().cloned(), vec![1, 2]))
                }

                #[test]
                fn test_values() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(1, 3), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert!(unordered_elements_are(
                        map.values().cloned(),
                        vec![2, 3, 3, 4]
                    ))
                }

                #[test]
                fn test_iter() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert!(unordered_elements_are(
                        map.iter()
                            .map(|(k, v)| (k.clone(), Vec::from_iter(v.iter().cloned()))),
                        vec![(1, vec![2]), (2, vec![3])]
                    ));
                }

                #[test]
                fn test_flat_iter() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(1, 3), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert!(unordered_elements_are(
                        map.flat_iter().map(|(k, v)| (k.clone(), v.clone())),
                        vec![(1, 2), (1, 3), (2, 3), (2, 4)]
                    ));
                }

                #[test]
                fn test_is_empty() {
                    let mut map = $map_maker;
                    assert_eq!(map.is_empty(), true);
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.is_empty(), false);
                    assert_eq!(map.remove(&1, &2), true);
                    assert_eq!(map.is_empty(), true);
                }

                #[test]
                fn test_num_keys() {
                    let mut map = $map_maker;
                    assert_eq!(map.num_keys(), 0);
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.num_keys(), 1);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.num_keys(), 2);
                    assert_eq!(map.insert(2, 4), true);
                    assert_eq!(map.num_keys(), 2);
                    assert_eq!(map.remove_key(&1), true);
                    assert_eq!(map.num_keys(), 1);
                    assert_eq!(map.remove_key(&1), false);
                    assert_eq!(map.num_keys(), 1);
                }

                #[test]
                fn test_len() {
                    let mut map = $map_maker;
                    assert_eq!(map.len(), 0);
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.len(), 1);
                    assert_eq!(map.insert(1, 3), true);
                    assert_eq!(map.len(), 2);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.len(), 3);
                    assert_eq!(map.insert(2, 4), true);
                    assert_eq!(map.len(), 4);
                    assert_eq!(map.remove(&1, &2), true);
                    assert_eq!(map.len(), 3);
                    assert_eq!(map.remove(&1, &3), true);
                    assert_eq!(map.len(), 2);
                    assert_eq!(map.remove(&2, &3), true);
                    assert_eq!(map.len(), 1);
                    assert_eq!(map.remove(&2, &4), true);
                    assert_eq!(map.len(), 0);
                }
            }
        };
    }

    macro_rules! sorted_keys_test_suite {
        ($mod_name:ident, $map_maker:expr) => {
            mod $mod_name {
                use super::is_sorted;
                use crate::builder::MultiMapBuilder;

                #[test]
                fn test_keys_sorted() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert_eq!(map.insert(3, 5), true);
                    assert!(is_sorted(map.keys()));
                }

                #[test]
                fn test_iter_sorted() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert_eq!(map.insert(3, 5), true);
                    assert!(is_sorted(map.iter().map(|(k, _v)| k)));
                }

                #[test]
                fn test_range_keys() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert_eq!(map.insert(3, 5), true);
                    assert!(is_sorted(map.range(1..3).map(|(k, _v)| k)));
                }

                #[test]
                fn test_flat_range_keys() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert_eq!(map.insert(3, 5), true);
                    assert!(is_sorted(map.flat_range(1..3).map(|(k, _v)| k)));
                }
            }
        };
    }

    macro_rules! sorted_values_test_suite {
        ($map_name:ident, $map_maker:expr) => {
            mod $map_name {
                use super::is_sorted;
                use crate::builder::MultiMapBuilder;

                #[test]
                fn test_each_set_sorted() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert_eq!(map.insert(3, 5), true);
                    for (_, set) in map.iter() {
                        assert!(is_sorted(set.iter()));
                    }
                }
            }
        };
    }

    fn unordered_elements_are<T, I>(i: I, v: Vec<T>) -> bool
    where
        T: Eq + Clone,
        I: IntoIterator<Item = T>,
    {
        let mut v = v.clone();
        for x in i {
            if let Some(pos) = v.iter().position(|y| *y == x) {
                v.remove(pos);
            } else {
                return false;
            }
        }
        v.is_empty()
    }

    fn is_sorted<I>(i: I) -> bool
    where
        I: IntoIterator,
        I::Item: Ord,
    {
        let mut i = i.into_iter();
        if let Some(mut prev) = i.next() {
            for next in i {
                if prev > next {
                    return false;
                }
                prev = next;
            }
        }
        true
    }

    base_test_suite!(
        hash_values_hash_keys,
        MultiMapBuilder::hash_keys().hash_values().build()
    );

    base_test_suite!(
        hash_values_sorted_keys,
        MultiMapBuilder::sorted_keys().hash_values().build()
    );

    sorted_keys_test_suite!(
        hash_values_sorted_keys_sorted_key_tests,
        MultiMapBuilder::sorted_keys().hash_values().build()
    );

    base_test_suite!(
        sorted_values_hash_keys,
        MultiMapBuilder::hash_keys().sorted_values().build()
    );

    sorted_values_test_suite!(
        sorted_values_hashs_keys_sorted_values_tests,
        MultiMapBuilder::hash_keys().sorted_values().build()
    );

    base_test_suite!(
        sorted_values_sorted_keys,
        MultiMapBuilder::sorted_keys().sorted_values().build()
    );

    sorted_keys_test_suite!(
        sorted_values_sorted_keys_sorted_key_tests,
        MultiMapBuilder::sorted_keys().sorted_values().build()
    );

    sorted_values_test_suite!(
        sorted_values_sorted_keys_sorted_values_tests,
        MultiMapBuilder::sorted_keys().sorted_values().build()
    );
}
