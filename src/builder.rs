use std::hash::Hash;
use crate::sets::{HashTableSet, TreeSet};
use crate::maps::{HashTableMap, Map, TreeMap};
use crate::MultiMapImpl;


pub struct MultiMapBuilder {}

impl MultiMapBuilder {
    pub fn new() -> Self {
        MultiMapBuilder {}
    }

    pub fn hash_values<V: Hash + Eq>(self) -> MultiMapBuilderWithVals<HashTableSet<V>> {
        MultiMapBuilderWithVals::new()
    }

    pub fn sorted_values<V: Ord>(self) -> MultiMapBuilderWithVals<TreeSet<V>> {
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

    pub fn hash_keys<K: Hash + Eq>(self) -> MultiMapBuilderWithKeysAndVals<HashTableMap<K, S>> {
        MultiMapBuilderWithKeysAndVals::<HashTableMap<K, S>>::new()
    }

    pub fn sorted_keys<K: Ord + Eq>(self) -> MultiMapBuilderWithKeysAndVals<TreeMap<K, S>> {
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

    pub fn build(self) -> MultiMapImpl<M> {
        MultiMapImpl::new()
    }
}