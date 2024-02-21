use crate::MultiMap;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

/// A builder for a multi-map. This struct does nothing by itself, but it is used to chain method calls to
/// configure the multi-map before building it.
pub struct MultiMapBuilder {}

impl MultiMapBuilder {
    /// Configures the multi-map to use a hash set for the values.
    pub fn hash_values<V>() -> MultiMapBuilderWithVals<impl Fn() -> HashSet<V>>
    where
        V: Hash + Eq,
    {
        Self::with_value_set_factory(HashSet::new)
    }

    /// Configures the multi-map to use a sorted set for the values.
    pub fn sorted_values<V>() -> MultiMapBuilderWithVals<impl Fn() -> BTreeSet<V>>
    where
        V: Ord,
    {
        Self::with_value_set_factory(BTreeSet::new)
    }

    /// An advanced method for configuring the multi-map to use a custom value set factory.
    /// This is useful if you want to use your own custom set type.
    /// To do anything useful, your output type should implement the `Set` trait.
    pub fn with_value_set_factory<F>(value_set_factory: F) -> MultiMapBuilderWithVals<F> {
        MultiMapBuilderWithVals { value_set_factory }
    }
}

/// A builder for a multi-map that has a known type for value sets.
pub struct MultiMapBuilderWithVals<F> {
    value_set_factory: F,
}

impl<F, O> MultiMapBuilderWithVals<F>
where
    F: Fn() -> O,
{
    /// Configures the multi-map to use a hash map for the keys.
    pub fn hash_keys<K>(
        self,
    ) -> MultiMapBuilderWithKeysAndVals<impl Fn() -> HashMap<K, F::Output>, F>
    where
        K: Hash + Eq,
    {
        self.with_map_factory(HashMap::new)
    }

    /// Configures the multi-map to use a sorted map for the keys.
    pub fn sorted_keys<K>(
        self,
    ) -> MultiMapBuilderWithKeysAndVals<impl Fn() -> BTreeMap<K, F::Output>, F>
    where
        K: Ord,
    {
        self.with_map_factory(BTreeMap::new)
    }

    /// An advanced method for configuring the multi-map to use a custom map factory.
    /// This is useful if you want to use your own custom map type.
    /// To do anything useful, your output type should implement the `Map` trait.
    pub fn with_map_factory<G>(self, map_factory: G) -> MultiMapBuilderWithKeysAndVals<G, F> {
        MultiMapBuilderWithKeysAndVals {
            map_factory,
            value_set_factory: self.value_set_factory,
        }
    }
}

/// A builder for a multi-map that has a known type for keys and values.
pub struct MultiMapBuilderWithKeysAndVals<F, G> {
    map_factory: F,
    value_set_factory: G,
}

impl<M, S, F, G> MultiMapBuilderWithKeysAndVals<F, G>
where
    F: Fn() -> M,
    G: Fn() -> S,
{
    /// Builds a multi-map. This is the only usable method on this struct.
    pub fn build(self) -> MultiMap<M, G> {
        let map = (self.map_factory)();
        MultiMap::from_parts(map, self.value_set_factory)
    }
}
