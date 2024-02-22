use crate::maps::Map;
use crate::sets::Set;
use crate::MultiMap;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

/// A builder for a multi-map. This struct does nothing by itself, but it is used to chain method calls to
/// configure the multi-map before building it.
pub struct MultiMapBuilder {}

impl MultiMapBuilder {
    /// Configures the multi-map to use a hashmap.
    pub fn hash_keys<K, S>() -> MultiMapBuilderWithKeys<HashMap<K, S>>
    where
        K: Hash + Eq,
    {
        Self::with_map_type()
    }

    /// Configures the multi-map to use a sorted map.
    pub fn sorted_keys<K, S>() -> MultiMapBuilderWithKeys<BTreeMap<K, S>>
    where
        K: Ord,
    {
        Self::with_map_type()
    }

    pub fn with_map_type<M>() -> MultiMapBuilderWithKeys<M>
    where
        M: Map,
    {
        MultiMapBuilderWithKeys {
            _m: std::marker::PhantomData,
        }
    }
}

/// A builder for a multi-map that has a known type for the map.
pub struct MultiMapBuilderWithKeys<M>
where
    M: Map,
{
    _m: std::marker::PhantomData<M>,
}

impl<M> MultiMapBuilderWithKeys<M>
where
    M: Map,
{
    /// Configures the multi-map to use a hash set for values.
    pub fn hash_values<V>(self) -> MultiMapBuilderWithKeysAndVals<M>
    where
        M: Map<Val = HashSet<V>>,
        V: Hash + Eq,
    {
        self.with_set_type()
    }

    /// Configures the multi-map to use a sorted set for values.
    pub fn sorted_values<V>(self) -> MultiMapBuilderWithKeysAndVals<M>
    where
        M: Map<Val = BTreeSet<V>>,
        V: Ord,
    {
        self.with_set_type()
    }

    pub fn with_set_type(self) -> MultiMapBuilderWithKeysAndVals<M>
    where
        M: Map,
        M::Val: Set,
    {
        MultiMapBuilderWithKeysAndVals {
            _m: std::marker::PhantomData,
        }
    }
}

/// A builder for a multi-map that has a known type for keys and values.
pub struct MultiMapBuilderWithKeysAndVals<M> {
    _m: std::marker::PhantomData<M>,
}

impl<M> MultiMapBuilderWithKeysAndVals<M>
where
    M: Default,
{
    /// Builds a multi-map.
    pub fn build(self) -> MultiMap<M> {
        Default::default()
    }
}
