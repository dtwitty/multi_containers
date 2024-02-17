use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use crate::maps::Map;
use crate::MultiMap;


pub struct MultiMapBuilder {}

impl MultiMapBuilder {
    pub fn new() -> Self {
        MultiMapBuilder {}
    }

    pub fn hash_values<V: Hash + Eq>(self) -> MultiMapBuilderWithVals<HashSet<V>> {
        MultiMapBuilderWithVals::new()
    }

    pub fn sorted_values<V: Ord>(self) -> MultiMapBuilderWithVals<BTreeSet<V>> {
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

    pub fn hash_keys<K: Hash + Eq>(self) -> MultiMapBuilderWithKeysAndVals<HashMap<K, S>> {
        MultiMapBuilderWithKeysAndVals::new()
    }

    pub fn sorted_keys<K: Ord + Eq>(self) -> MultiMapBuilderWithKeysAndVals<BTreeMap<K, S>> {
        MultiMapBuilderWithKeysAndVals::new()
    }
}

pub struct MultiMapBuilderWithKeysAndVals<M> {
    _m: std::marker::PhantomData<M>,
}

impl<M: Map + Default> MultiMapBuilderWithKeysAndVals<M> {
    pub fn new() -> Self {
        MultiMapBuilderWithKeysAndVals {
            _m: std::marker::PhantomData,
        }
    }

    pub fn build(self) -> MultiMap<M> {
        MultiMap::new()
    }
}