use crate::MultiMap;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

/// A builder for a multi-map. This struct does nothing by itself, but it is used to chain method calls to
/// configure the multi-map before building it.
pub struct MultiMapBuilder {}

impl MultiMapBuilder {
    /// Configures the multi-map to use a hashmap.
    pub fn hash_keys<K, S>() -> MultiMapBuilderWithKeys<impl Fn() -> HashMap<K, S>>
    where
        K: Hash + Eq,
    {
        Self::with_map_factory(HashMap::new)
    }

    /// Configures the multi-map to use a sorted map.
    pub fn sorted_keys<K, S>() -> MultiMapBuilderWithKeys<impl Fn() -> BTreeMap<K, S>>
    where
        K: Ord,
    {
        Self::with_map_factory(BTreeMap::new)
    }

    /// An advanced method for configuring the multi-map to use a custom map factory.
    /// This is useful if you want to use your own custom map type.
    /// To do anything useful, your output type should implement the `Map` trait, and the value type should implement the `Set` trait.
    /// For correctness, the output of `map_factory` should be an empty map.
    pub fn with_map_factory<F>(map_factory: F) -> MultiMapBuilderWithKeys<F> {
        MultiMapBuilderWithKeys { map_factory }
    }
}

/// A builder for a multi-map that has a known type for the map.
pub struct MultiMapBuilderWithKeys<F> {
    map_factory: F,
}

impl<F, O> MultiMapBuilderWithKeys<F>
where
    F: Fn() -> O,
{
    /// Configures the multi-map to use a hash set for values.
    pub fn hash_values<V>(self) -> MultiMapBuilderWithKeysAndVals<F, impl Fn() -> HashSet<V>>
    where
        V: Hash + Eq,
    {
        self.with_value_set_factory(HashSet::new)
    }

    /// Configures the multi-map to use a sorted set for values.
    pub fn sorted_values<V>(self) -> MultiMapBuilderWithKeysAndVals<F, impl Fn() -> BTreeSet<V>>
    where
        V: Ord,
    {
        self.with_value_set_factory(BTreeSet::new)
    }

    /// An advanced method for configuring the multi-map to use a custom value set factory.
    /// This is useful if you want to use your own custom set type.
    /// To do anything useful, your output type should implement the `Set` trait.
    pub fn with_value_set_factory<G>(
        self,
        value_set_factory: G,
    ) -> MultiMapBuilderWithKeysAndVals<F, G> {
        MultiMapBuilderWithKeysAndVals {
            map_factory: self.map_factory,
            value_set_factory,
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
    /// Builds a multi-map.
    pub fn build(self) -> MultiMap<M, G> {
        let map = (self.map_factory)();
        MultiMap::from_parts(map, self.value_set_factory)
    }
}
