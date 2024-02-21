use crate::MultiSet;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

/// A builder for a multi-set. This struct does nothing by itself, but it is used to chain method calls to
/// configure the multi-set before building it.
pub struct MultiSetBuilder {}

impl MultiSetBuilder {
    /// Configures the multi-set to use a hashmap.
    pub fn hash_values<K>() -> MultiSetBuilderWithVals<impl Fn() -> HashMap<K, usize>>
    where
        K: Hash + Eq,
    {
        Self::with_map_factory(HashMap::new)
    }

    /// Configures the multi-set to use a sorted map.
    pub fn sorted_values<K>() -> MultiSetBuilderWithVals<impl Fn() -> BTreeMap<K, usize>>
    where
        K: Ord,
    {
        Self::with_map_factory(BTreeMap::new)
    }

    /// An advanced method for configuring the multi-set to use a custom map factory.
    /// This is useful if you want to use your own custom map type.
    /// To do anything useful, your output type should implement the `Map` trait, and the value type should implement the `Set` trait.
    /// For correctness, the output of `map_factory` should be an empty map.
    pub fn with_map_factory<F>(map_factory: F) -> MultiSetBuilderWithVals<F> {
        MultiSetBuilderWithVals { map_factory }
    }
}

/// A builder for a multi-set that has a known type for the map.
pub struct MultiSetBuilderWithVals<F> {
    map_factory: F,
}

impl<M, F> MultiSetBuilderWithVals<F>
where
    F: Fn() -> M,
{
    /// Builds the multi-set.
    pub fn build(self) -> MultiSet<M> {
        let map = (self.map_factory)();
        MultiSet::from_parts(map)
    }
}
