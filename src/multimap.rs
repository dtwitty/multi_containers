use crate::maps::*;
use crate::sets::*;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::RangeBounds;

/// A multi-map from keys to values.
/// This can be thought of as an ergonomic wrapper around `Map<K, Set<V>>`.
/// The semantics of a multi-map is that it can contain multiple values for a single key.
/// The multi-map is implemented as a managed map from keys to sets of values. For bookkeeping, the
/// value sets are queryable, but not modifiable except through the multi-map API.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct MultiMap<M> {
    map: M,
    length: usize,
}

impl<M> MultiMap<M>
where
    M: Default,
{
    /// Creates a new, empty multi-map.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiMap;
    /// let mut map: HashMultiMap<&str, i32> = HashMultiMap::new();
    /// assert_eq!(map.num_keys(), 0);
    /// assert_eq!(map.num_mappings(), 0);
    /// ```
    ///
    ///
    pub fn new() -> Self {
        MultiMap {
            map: Default::default(),
            length: 0,
        }
    }
}

impl<M> MultiMap<M>
where
    M: Map,
    M::Val: Set,
{
    /// Inserts a (key, value) mapping into the multi-map.
    /// Returns `true` if it was not already present.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiMap;
    /// let mut map = HashMultiMap::new();
    /// assert_eq!(map.insert("a", 1), true);
    /// assert_eq!(map.insert("a", 1), false);
    /// assert_eq!(map.insert("a", 2), true);
    /// assert_eq!(map.insert("b", 2), true);
    /// assert_eq!(map.num_keys(), 2);
    /// assert_eq!(map.num_mappings(), 3);
    /// ```
    pub fn insert(&mut self, key: M::Key, value: <<M as Map>::Val as Set>::Elem) -> bool
    where
        M::Val: Default,
    {
        if self.map.get_or_insert(key, Default::default).insert(value) {
            self.length += 1;
            true
        } else {
            false
        }
    }

    /// Returns `true` if the multi-map contains the given (key, value) mapping.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiMap;
    /// let mut map = HashMultiMap::new();
    /// assert_eq!(map.insert("a".to_string(), 1), true);
    /// assert_eq!(map.contains("a", &1), true);
    /// assert_eq!(map.contains("a", &2), false);
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiMap;
    /// let mut map = HashMultiMap::new();
    /// assert_eq!(map.contains_key("a"), false);
    /// assert_eq!(map.insert("a".to_string(), 1), true);
    /// assert_eq!(map.contains_key("a"), true);
    /// ```
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.contains_key(key)
    }

    /// Removes a (key, value) mapping from the multi-map.
    /// Returns `true` if it was present.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiMap;
    /// let mut map = HashMultiMap::new();
    /// assert_eq!(map.insert("a".to_string(), 1), true);
    /// assert_eq!(map.num_keys(), 1);
    /// assert_eq!(map.num_mappings(), 1);
    /// assert_eq!(map.remove("a", &1), true);
    /// assert_eq!(map.remove("a", &1), false);
    /// assert_eq!(map.num_keys(), 0);
    /// assert_eq!(map.num_mappings(), 0);
    /// ```
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

    /// Removes all mappings with the given key from the multi-map.
    /// Returns the set of values that were removed, if any.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiMap;
    /// let mut map = HashMultiMap::new();
    /// assert_eq!(map.insert("a".to_string(), 1), true);
    /// assert_eq!(map.insert("a".to_string(), 2), true);
    /// assert_eq!(map.remove_key("a").unwrap().contains(&1), true);
    /// assert_eq!(map.remove_key("a").is_some(), false);
    /// ```
    pub fn remove_key<Q>(&mut self, key: &Q) -> Option<M::Val>
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.remove(key)
    }

    /// Returns a reference to the set of values for the given key, if there are any.
    /// If the returned value is `Some`, it is guaranteed to be non-empty.
    /// If the returned value is `None`, the key is not present in the multi-map.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiMap;
    /// let mut map = HashMultiMap::new();
    /// assert_eq!(map.get("a"), None);
    /// assert_eq!(map.insert("a".to_string(), 1), true);
    /// assert_eq!(map.get("a").unwrap().contains(&1), true);
    /// assert_eq!(map.get("a").unwrap().contains(&2), false);
    /// ```
    pub fn get<Q>(&self, key: &Q) -> Option<&M::Val>
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.map.get(key)
    }

    /// Returns an iterator over the keys of the multi-map.
    /// The keys are returned in the order specified by the underlying `Map` implementation.
    ///
    /// # Example
    ///
    /// ```
    /// // `BTreeMultiMap` is used here to ensure that the keys are returned in sorted order.
    /// use multi_containers::BTreeMultiMap;
    /// let mut map = BTreeMultiMap::<u32, u32>::new();
    /// map.insert(1, 2);
    /// map.insert(2, 3);
    /// map.insert(2, 4);
    /// assert_eq!(map.keys().collect::<Vec<_>>(), vec![&1, &2]);
    /// ```
    pub fn keys(&self) -> M::KeyIter<'_> {
        self.map.keys()
    }

    /// Returns an iterator over the keys and value sets in the multi-map.
    /// The keys are returned in the order specified by the underlying `Map` implementation.
    ///
    /// # Example
    ///
    /// ```
    /// // `BTreeMultiMap` is used here to ensure that the keys are returned in sorted order.
    /// use multi_containers::BTreeMultiMap;
    /// let mut map = BTreeMultiMap::<u32, u32>::new();
    /// map.insert(1, 2);
    /// map.insert(2, 3);
    /// map.insert(2, 4);
    /// assert_eq!(
    ///     map.value_sets()
    ///         .map(|(k, v)| (k, v.iter().collect::<Vec<_>>()))
    ///         .collect::<Vec<_>>(),
    ///     vec![(&1, vec![&2]), (&2, vec![&3, &4])]
    /// );
    /// ```
    pub fn value_sets(&self) -> M::Iter<'_> {
        self.map.iter()
    }

    /// Returns an iterator over the keys and values in the multi-map.
    /// The keys are returned in the order specified by the underlying `Map` implementation.
    /// The values are returned in the order specified by the underlying `Set` implementation.
    ///
    /// # Example
    ///
    /// ```
    /// // `BTreeMultiMap` is used here to ensure that the keys are returned in sorted order.
    /// use multi_containers::BTreeMultiMap;
    /// let mut map = BTreeMultiMap::<u32, u32>::new();
    /// map.insert(1, 2);
    /// map.insert(1, 3);
    /// map.insert(2, 3);
    /// map.insert(2, 4);
    /// assert_eq!(map.mappings().collect::<Vec<_>>(), vec![(&1, &2), (&1, &3), (&2, &3), (&2, &4)]);
    /// ```
    pub fn mappings(&self) -> impl Iterator<Item = (&M::Key, &<<M as Map>::Val as Set>::Elem)> {
        self.value_sets()
            .flat_map(|(k, s)| s.iter().map(move |v| (k, v)))
    }

    /// Returns `true` if the multi-map is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiMap;
    /// let mut map = HashMultiMap::new();
    /// assert_eq!(map.is_empty(), true);
    /// assert_eq!(map.insert("a".to_string(), 1), true);
    /// assert_eq!(map.is_empty(), false);
    /// assert_eq!(map.remove("a", &1), true);
    /// assert_eq!(map.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the number of keys in the multi-map.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiMap;
    /// let mut map = HashMultiMap::new();
    /// assert_eq!(map.num_keys(), 0);
    /// assert_eq!(map.insert("a".to_string(), 1), true);
    /// assert_eq!(map.num_keys(), 1);
    /// assert_eq!(map.insert("a".to_string(), 2), true);
    /// assert_eq!(map.num_keys(), 1);
    /// assert_eq!(map.insert("b".to_string(), 2), true);
    /// assert_eq!(map.num_keys(), 2);
    /// assert_eq!(map.remove_key("a").is_some(), true);
    /// assert_eq!(map.num_keys(), 1);
    /// ```
    pub fn num_keys(&self) -> usize {
        self.map.len()
    }

    /// Returns the number of (key, value) mappings in the multi-map.
    /// This is the total number of values across all keys.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::HashMultiMap;
    /// let mut map = HashMultiMap::new();
    /// assert_eq!(map.num_mappings(), 0);
    /// assert_eq!(map.insert("a".to_string(), 1), true);
    /// assert_eq!(map.num_mappings(), 1);
    /// assert_eq!(map.insert("a".to_string(), 2), true);
    /// assert_eq!(map.num_mappings(), 2);
    /// assert_eq!(map.insert("b".to_string(), 2), true);
    /// assert_eq!(map.num_mappings(), 3);
    /// assert_eq!(map.remove("a", &1), true);
    /// assert_eq!(map.num_mappings(), 2);
    /// ```
    pub fn num_mappings(&self) -> usize {
        self.length
    }

    /// Returns an iterator over the keys and value sets in the multi-map within a range of keys.
    /// The keys are returned in sorted order, as this method is only available for multi-maps with sorted keys.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::BTreeMultiMap;
    /// let mut map = BTreeMultiMap::<u32, u32>::new();
    /// map.insert(1, 2);
    /// map.insert(2, 3);
    /// map.insert(2, 4);
    /// assert_eq!(map.value_sets_in_range(1..3).map(|(k, _v)| k).collect::<Vec<_>>(), vec![&1, &2]);
    /// ```
    pub fn value_sets_in_range<Q, R>(&self, range: R) -> M::RangeIter<'_>
    where
        M: SortedMap<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
        R: RangeBounds<Q>,
    {
        self.map.range(range)
    }

    /// Returns an iterator over the keys and values in the multi-map within a range of keys.
    /// The keys are returned in sorted order, as this method is only available for multi-maps with sorted keys.
    ///
    /// # Example
    ///
    /// ```
    /// use multi_containers::BTreeMultiMap;
    /// let mut map = BTreeMultiMap::<u32, u32>::new();
    /// map.insert(1, 2);
    /// map.insert(1, 3);
    /// map.insert(2, 3);
    /// map.insert(2, 4);
    /// assert_eq!(map.mappings_in_range(1..3).map(|(k, _v)| k).collect::<Vec<_>>(), vec![&1, &1, &2, &2]);
    /// ```
    pub fn mappings_in_range<Q, R>(
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

impl <T, M> Extend<T> for MultiMap<M>
where M: Map,
      M::Val: Set + Default,
      T: Into<(M::Key, <<M as Map>::Val as Set>::Elem)> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for t in iter {
            let (k, v) = t.into();
            self.insert(k, v);
        }
    }
}

impl<T, M> FromIterator<T> for MultiMap<M>
where
    M: Map + Default,
    M::Val: Set + Default,
    T: Into<(M::Key, <<M as Map>::Val as Set>::Elem)>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut map = MultiMap::new();
        map.extend(iter);
        map
    }
}

impl<M, const N: usize> From<[(M::Key, <<M as Map>::Val as Set>::Elem); N]> for MultiMap<M>
where
    M: Map + Default,
    M::Val: Set + Default,
{
    fn from(arr: [(M::Key, <<M as Map>::Val as Set>::Elem); N]) -> Self {
        arr.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    macro_rules! base_test_suite {
        ($mod_name:ident, $map_maker:expr) => {
            mod $mod_name {
                use crate::test_utils::unordered_elements_are;
                use crate::MultiMapBuilder;

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
                    assert_eq!(map.remove_key(&1).unwrap().contains(&2), true);
                    assert_eq!(map.remove_key(&1), None);
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
                fn test_value_sets() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert!(unordered_elements_are(
                        map.value_sets()
                            .map(|(k, v)| (k.clone(), Vec::from_iter(v.iter().cloned()))),
                        vec![(1, vec![2]), (2, vec![3])]
                    ));
                }

                #[test]
                fn test_mappings() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(1, 3), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert!(unordered_elements_are(
                        map.mappings().map(|(k, v)| (k.clone(), v.clone())),
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
                    assert!(map.remove_key(&1).is_some());
                    assert_eq!(map.num_keys(), 1);
                    assert!(map.remove_key(&1).is_none());
                    assert_eq!(map.num_keys(), 1);
                }

                #[test]
                fn test_num_mappings() {
                    let mut map = $map_maker;
                    assert_eq!(map.num_mappings(), 0);
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.num_mappings(), 1);
                    assert_eq!(map.insert(1, 3), true);
                    assert_eq!(map.num_mappings(), 2);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.num_mappings(), 3);
                    assert_eq!(map.insert(2, 4), true);
                    assert_eq!(map.num_mappings(), 4);
                    assert_eq!(map.remove(&1, &2), true);
                    assert_eq!(map.num_mappings(), 3);
                    assert_eq!(map.remove(&1, &3), true);
                    assert_eq!(map.num_mappings(), 2);
                    assert_eq!(map.remove(&2, &3), true);
                    assert_eq!(map.num_mappings(), 1);
                    assert_eq!(map.remove(&2, &4), true);
                    assert_eq!(map.num_mappings(), 0);
                }
            }
        };
    }

    macro_rules! sorted_keys_test_suite {
        ($mod_name:ident, $map_maker:expr) => {
            mod $mod_name {
                use crate::test_utils::is_sorted;
                use crate::MultiMapBuilder;

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
                fn test_value_sets_sorted() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert_eq!(map.insert(3, 5), true);
                    assert!(is_sorted(map.value_sets().map(|(k, _v)| k)));
                }

                #[test]
                fn test_mappings_sorted() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert_eq!(map.insert(3, 5), true);
                    assert!(is_sorted(map.mappings().map(|(k, _v)| k)));
                }

                #[test]
                fn test_value_sets_in_range_keys() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert_eq!(map.insert(3, 5), true);
                    assert!(is_sorted(map.value_sets_in_range(1..3).map(|(k, _v)| k)));
                }

                #[test]
                fn test_mappings_in_range_keys() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert_eq!(map.insert(3, 5), true);
                    assert!(is_sorted(map.mappings_in_range(1..3).map(|(k, _v)| k)));
                }
            }
        };
    }

    macro_rules! sorted_values_test_suite {
        ($map_name:ident, $map_maker:expr) => {
            mod $map_name {
                use crate::test_utils::is_sorted;
                use crate::MultiMapBuilder;

                #[test]
                fn test_each_set_sorted() {
                    let mut map = $map_maker;
                    assert_eq!(map.insert(1, 2), true);
                    assert_eq!(map.insert(2, 3), true);
                    assert_eq!(map.insert(2, 4), true);
                    assert_eq!(map.insert(3, 5), true);
                    for (_, set) in map.value_sets() {
                        assert!(is_sorted(set.iter()));
                    }
                }
            }
        };
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
