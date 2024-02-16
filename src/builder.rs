use std::fmt::Debug;
use std::hash::Hash;
use crate::sets::{HashTableSet, Set, SortedSet, TreeSet};
use crate::maps::{HashTableMap, Map, TreeMap};
use crate::MultiMapImpl;


pub struct MultiMapBuilder {}

impl MultiMapBuilder {
    pub fn new() -> Self {
        MultiMapBuilder {}
    }

    pub fn hash_values<V: Hash + Eq + Clone + Debug>(self) -> MultiMapBuilderWithVals<impl Set<Elem=V>> {
        MultiMapBuilderWithVals::<HashTableSet<V>>::new()
    }

    pub fn sorted_values<V: Ord + Debug + Clone>(self) -> MultiMapBuilderWithVals<impl SortedSet<Elem=V>> {
        MultiMapBuilderWithVals::<TreeSet<V>>::new()
    }
}

pub struct MultiMapBuilderWithVals<S> {
    _s: std::marker::PhantomData<S>,
}

impl<S: Eq + Debug + Clone> MultiMapBuilderWithVals<S> {
    pub fn new() -> Self {
        MultiMapBuilderWithVals {
            _s: std::marker::PhantomData,
        }
    }

    pub fn hash_keys<K: Hash + Eq + Clone + Debug>(self) -> MultiMapBuilderWithKeysAndVals<HashTableMap<K, S>> {
        MultiMapBuilderWithKeysAndVals::<HashTableMap<K, S>>::new()
    }

    pub fn sorted_keys<K: Ord + Eq + Clone + Debug>(self) -> MultiMapBuilderWithKeysAndVals<TreeMap<K, S>> {
        MultiMapBuilderWithKeysAndVals::new()
    }
}

pub struct MultiMapBuilderWithKeysAndVals<M> {
    _m: std::marker::PhantomData<M>,
}

impl<M: Map> MultiMapBuilderWithKeysAndVals<M> {
    pub fn new() -> Self {
        MultiMapBuilderWithKeysAndVals {
            _m: std::marker::PhantomData,
        }
    }

    pub fn build(self) -> MultiMapImpl<M> {
        MultiMapImpl::new()
    }
}