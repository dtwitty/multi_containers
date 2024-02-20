use crate::maps::Map;
use crate::MultiMap;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

/// A builder for a multi-map. This struct does nothing by itself, but it is used to chain method calls to
/// configure the multi-map before building it.
pub struct MultiMapBuilder {}

impl MultiMapBuilder {
    /// Configures the multi-map to use a hash set for the values.
    pub fn hash_values<V>() -> MultiMapBuilderWithVals<HashSet<V>>
    where
        V: Hash + Eq,
    {
        MultiMapBuilderWithVals::new()
    }

    /// Configures the multi-map to use a sorted set for the values.
    pub fn sorted_values<V>() -> MultiMapBuilderWithVals<BTreeSet<V>>
    where
        V: Ord,
    {
        MultiMapBuilderWithVals::new()
    }
}

/// A builder for a multi-map that has a known type for value sets.
pub struct MultiMapBuilderWithVals<S> {
    _s: std::marker::PhantomData<S>,
}

impl<S> MultiMapBuilderWithVals<S> {
    fn new() -> Self {
        MultiMapBuilderWithVals {
            _s: std::marker::PhantomData,
        }
    }

    /// Configures the multi-map to use a hash map for the keys.
    pub fn hash_keys<K>(self) -> MultiMapBuilderWithKeysAndVals<HashMap<K, S>>
    where
        K: Hash + Eq,
    {
        MultiMapBuilderWithKeysAndVals::new()
    }

    /// Configures the multi-map to use a sorted map for the keys.
    pub fn sorted_keys<K>(self) -> MultiMapBuilderWithKeysAndVals<BTreeMap<K, S>>
    where
        K: Ord,
    {
        MultiMapBuilderWithKeysAndVals::new()
    }
}

/// A builder for a multi-map that has a known type for keys and values. Can be used to vend multi-maps.
pub struct MultiMapBuilderWithKeysAndVals<M> {
    _m: std::marker::PhantomData<M>,
}

impl<M> MultiMapBuilderWithKeysAndVals<M>
where
    M: Map + Default,
{
    fn new() -> Self {
        MultiMapBuilderWithKeysAndVals {
            _m: std::marker::PhantomData,
        }
    }

    /// Builds a multi-map. This is the only usable method on this struct.
    pub fn build(self) -> MultiMap<M> {
        MultiMap::new()
    }
}
