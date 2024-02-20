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
pub struct MultiMap<M> {
    data: M,
    length: usize,
}

impl<M> MultiMap<M>
where
    M: Default,
{
    /// Creates a new empty multi-map.
    pub fn new() -> Self {
        MultiMap {
            data: Default::default(),
            length: 0,
        }
    }
}

impl<M> MultiMap<M>
where
    M: Map,
    M::Val: Set + Default,
{
    /// Inserts a (key, value) mapping into the multi-map. Returns `true` if it was not already present.
    pub fn insert(&mut self, key: M::Key, value: <<M as Map>::Val as Set>::Elem) -> bool {
        if self.data.get_or_insert(key, Default::default).insert(value) {
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
        self.data.get(key).map_or(false, |set| set.contains(value))
    }

    /// Returns `true` if the multi-map contains any mapping with the given key.
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.data.contains_key(key)
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
        if let Some(set) = self.data.get_mut(key) {
            if set.remove(value) {
                self.length -= 1;
                if set.is_empty() {
                    self.data.remove(key);
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
        self.data.remove(key)
    }

    /// Returns a reference to the set of values for the given key, if there are any.
    pub fn get<Q>(&self, key: &Q) -> Option<&M::Val>
    where
        M: Lookup<Q>,
        M::Key: Borrow<Q>,
        Q: ?Sized,
    {
        self.data.get(key)
    }

    /// Returns an iterator over the keys of the multi-map.
    pub fn keys(&self) -> M::KeyIter<'_> {
        self.data.keys()
    }

    /// Returns an iterator over the values of the multi-map.
    pub fn values(&self) -> impl Iterator<Item = &<<M as Map>::Val as Set>::Elem> {
        self.data.values().flat_map(|s| s.iter())
    }

    /// Returns an iterator over the keys and value sets in the multi-map.
    pub fn iter(&self) -> M::Iter<'_> {
        self.data.iter()
    }

    /// Returns an iterator over the keys and values in the multi-map.
    pub fn flat_iter(&self) -> impl Iterator<Item = (&M::Key, &<<M as Map>::Val as Set>::Elem)> {
        self.iter().flat_map(|(k, s)| s.iter().map(move |v| (k, v)))
    }

    /// Returns `true` if the multi-map is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of keys in the multi-map.
    pub fn num_keys(&self) -> usize {
        self.data.len()
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
        self.data.range(range)
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
        self.data
            .range(range)
            .flat_map(|(k, s)| s.iter().map(move |v| (k, v)))
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::MultiMapBuilder;

    #[test]
    fn test_hash_multi_map_insert() {
        let mut map = MultiMapBuilder::hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(!map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(!map.insert(2, 3));
    }

    #[test]
    fn test_hash_multi_map_remove() {
        let mut map = MultiMapBuilder::hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.remove(&1, &2));
        assert!(!map.remove(&1, &2));
        assert!(map.remove(&1, &3));
        assert!(!map.remove(&1, &3));
        assert!(map.remove(&2, &3));
        assert!(!map.remove(&2, &3));
    }

    #[test]
    fn test_hash_multi_map_contains() {
        let mut map = MultiMapBuilder::hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.contains(&1, &2));
        assert!(map.contains(&1, &3));
        assert!(map.contains(&2, &3));
        assert!(!map.contains(&1, &4));
        assert!(!map.contains(&2, &4));
    }

    #[test]
    fn test_hash_multi_map_contains_key() {
        let mut map = MultiMapBuilder::hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.contains_key(&1));
        assert!(map.contains_key(&2));
        assert!(!map.contains_key(&3));
    }

    #[test]
    fn test_hash_multi_map_get() {
        let mut map = MultiMapBuilder::hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.get(&1).unwrap().contains(&2));
        assert!(map.get(&1).unwrap().contains(&3));
        assert!(map.get(&2).unwrap().contains(&3));
        assert!(!map.get(&1).unwrap().contains(&4));
        assert!(!map.get(&2).unwrap().contains(&4));
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_hash_multi_map_keys() {
        let mut map = MultiMapBuilder::hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![&1, &2];
        let mut actual = map.keys().collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hash_multi_map_values() {
        let mut map = MultiMapBuilder::hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![&2, &3, &3];
        let mut actual = map.values().collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hash_multi_map_flat_iter() {
        let mut map = MultiMapBuilder::hash_values().hash_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![(&1, &2), (&1, &3), (&2, &3)];
        let mut actual = map.flat_iter().collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hash_multi_map_is_empty() {
        let mut map = MultiMapBuilder::hash_values().hash_keys().build();
        assert!(map.is_empty());
        assert!(map.insert(1, 2));
        assert!(!map.is_empty());
        assert!(map.remove(&1, &2));
        assert!(map.is_empty());
    }

    #[test]
    fn test_hash_multi_map_num_values() {
        let mut map = MultiMapBuilder::hash_values().hash_keys().build();
        assert_eq!(map.len(), 0);
        assert!(map.insert(1, 2));
        assert_eq!(map.len(), 1);
        assert!(map.insert(1, 3));
        assert_eq!(map.len(), 2);
        assert!(map.insert(2, 3));
        assert_eq!(map.len(), 3);
        assert!(map.remove(&1, &2));
        assert_eq!(map.len(), 2);
        assert!(map.remove(&1, &3));
        assert_eq!(map.len(), 1);
        assert!(map.remove(&2, &3));
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_hash_multi_map_num_keys() {
        let mut map = MultiMapBuilder::hash_values().hash_keys().build();
        assert_eq!(map.num_keys(), 0);
        assert!(map.insert(1, 2));
        assert_eq!(map.num_keys(), 1);
        assert!(map.insert(1, 3));
        assert_eq!(map.num_keys(), 1);
        assert!(map.insert(2, 3));
        assert_eq!(map.num_keys(), 2);
        assert!(map.remove(&1, &2));
        assert_eq!(map.num_keys(), 2);
        assert!(map.remove(&1, &3));
        assert_eq!(map.num_keys(), 1);
        assert!(map.remove(&2, &3));
        assert_eq!(map.num_keys(), 0);
    }

    #[test]
    fn test_sorted_multi_map_insert() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(!map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(!map.insert(2, 3));
    }

    #[test]
    fn test_sorted_multi_map_remove() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.remove(&1, &2));
        assert!(!map.remove(&1, &2));
        assert!(map.remove(&1, &3));
        assert!(!map.remove(&1, &3));
        assert!(map.remove(&2, &3));
        assert!(!map.remove(&2, &3));
    }

    #[test]
    fn test_sorted_multi_map_contains() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.contains(&1, &2));
        assert!(map.contains(&1, &3));
        assert!(map.contains(&2, &3));
        assert!(!map.contains(&1, &4));
        assert!(!map.contains(&2, &4));
    }

    #[test]
    fn test_sorted_multi_map_contains_key() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.contains_key(&1));
        assert!(map.contains_key(&2));
        assert!(!map.contains_key(&3));
    }

    #[test]
    fn test_sorted_multi_map_get() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.get(&1).unwrap().contains(&2));
        assert!(map.get(&1).unwrap().contains(&3));
        assert!(map.get(&2).unwrap().contains(&3));
        assert!(!map.get(&1).unwrap().contains(&4));
        assert!(!map.get(&2).unwrap().contains(&4));
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_sorted_multi_map_keys() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![&1, &2];
        let actual = map.keys().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_values() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![&2, &3, &3];
        let actual = map.values().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_flat_iter() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![(&1, &2), (&1, &3), (&2, &3)];
        let actual = map.flat_iter().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_is_empty() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        assert!(map.is_empty());
        assert!(map.insert(1, 2));
        assert!(!map.is_empty());
        assert!(map.remove(&1, &2));
        assert!(map.is_empty());
    }

    #[test]
    fn test_sorted_multi_map_len() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        assert_eq!(map.len(), 0);
        assert!(map.insert(1, 2));
        assert_eq!(map.len(), 1);
        assert!(map.insert(1, 3));
        assert_eq!(map.len(), 2);
        assert!(map.insert(2, 3));
        assert_eq!(map.len(), 3);
        assert!(map.remove(&1, &2));
        assert_eq!(map.len(), 2);
        assert!(map.remove(&1, &3));
        assert_eq!(map.len(), 1);
        assert!(map.remove(&2, &3));
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_sorted_multi_map_range() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.insert(3, 4));
        assert!(map.insert(4, 5));

        let expected = vec![&3, &4];
        let actual = map
            .range(2..=3)
            .flat_map(|(_, s)| s.iter())
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_flat_range() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        let expected = vec![(&1, &2), (&1, &3)];
        let actual = map.flat_range(1..2).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sorted_multi_map_remove_key() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        assert!(map.insert(1, 2));
        assert!(map.insert(1, 3));
        assert!(map.insert(2, 3));
        assert!(map.remove_key(&1));
        assert!(!map.remove_key(&1));
        assert!(map.remove_key(&2));
        assert!(!map.remove_key(&2));
    }

    #[test]
    fn test_borrowed_lookup_types() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        map.insert("a".to_string(), 1);
        assert!(map.contains_key("a"));
        let a: String = "a".to_string();
        assert!(map.contains_key(&a));
    }

    #[test]
    fn test_borrowed_key_types() {
        let mut map = MultiMapBuilder::sorted_values().sorted_keys().build();
        map.insert("a", 1);
        assert!(map.range("a".."b").next().is_some());
        assert!(map.contains_key(&"a"));
        assert!(map.contains_key(&("a".to_string())[..]));
    }
}
