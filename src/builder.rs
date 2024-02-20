use crate::maps::Map;
use crate::MultiMap;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

pub struct MultiMapBuilder {}

impl MultiMapBuilder {
    pub fn new() -> Self {
        MultiMapBuilder {}
    }

    pub fn hash_values<V>(self) -> MultiMapBuilderWithVals<HashSet<V>>
    where
        V: Hash + Eq,
    {
        MultiMapBuilderWithVals::new()
    }

    pub fn sorted_values<V>(self) -> MultiMapBuilderWithVals<BTreeSet<V>>
    where
        V: Ord,
    {
        MultiMapBuilderWithVals::new()
    }
}

pub struct MultiMapBuilderWithVals<S> {
    _s: std::marker::PhantomData<S>,
}

impl<S> MultiMapBuilderWithVals<S> {
    pub fn new() -> Self {
        MultiMapBuilderWithVals {
            _s: std::marker::PhantomData,
        }
    }

    pub fn hash_keys<K>(self) -> MultiMapBuilderWithKeysAndVals<HashMap<K, S>>
    where
        K: Hash + Eq,
    {
        MultiMapBuilderWithKeysAndVals::new()
    }

    pub fn sorted_keys<K>(self) -> MultiMapBuilderWithKeysAndVals<BTreeMap<K, S>>
    where
        K: Ord,
    {
        MultiMapBuilderWithKeysAndVals::new()
    }
}

pub struct MultiMapBuilderWithKeysAndVals<M> {
    _m: std::marker::PhantomData<M>,
}

impl<M> MultiMapBuilderWithKeysAndVals<M>
where
    M: Map + Default,
{
    pub fn new() -> Self {
        MultiMapBuilderWithKeysAndVals {
            _m: std::marker::PhantomData,
        }
    }

    pub fn build(self) -> MultiMap<M> {
        MultiMap::new()
    }
}
