use crate::MultiSet;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

/// A builder for a multi-set. This struct does nothing by itself, but it is used to chain method calls to
/// configure the multi-set before building it.
pub struct MultiSetBuilder {}

impl MultiSetBuilder {
    /// Configures the multi-set to use a hashmap.
    pub fn hash_values<K, S>() -> MultiSetBuilderWithVals<HashMap<K, usize, S>>
    where
        K: Hash + Eq,
    {
        Self::with_map_type()
    }

    /// Configures the multi-set to use a sorted map.
    pub fn sorted_values<K>() -> MultiSetBuilderWithVals<BTreeMap<K, usize>>
    where
        K: Ord,
    {
        Self::with_map_type()
    }

    pub fn with_map_type<M>() -> MultiSetBuilderWithVals<M> {
        MultiSetBuilderWithVals {
            _m: std::marker::PhantomData,
        }
    }
}

/// A builder for a multi-set that has a known type for the map.
pub struct MultiSetBuilderWithVals<M> {
    _m: std::marker::PhantomData<M>,
}

impl<M> MultiSetBuilderWithVals<M>
where
    M: Default,
{
    /// Builds the multi-set.
    pub fn build(self) -> MultiSet<M> {
        Default::default()
    }
}
