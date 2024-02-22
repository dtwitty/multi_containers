#![warn(missing_docs)]

//! Containers for working with non-unique keys and values.

/// Defines the `MultiMap` type.
pub mod multimap;

/// Provides a convenient way to construct multi-maps.
pub mod multimap_builder;

/// Defines the `MultiSet` type.
pub mod multiset;

/// Provides a convenient way to construct multi-sets.
pub mod multiset_builder;

/// Traits for working with maps.
pub mod maps;

/// Traits for working with sets.
pub mod sets;

mod examples;
mod test_utils;

pub use crate::multimap::MultiMap;
pub use crate::multimap_builder::MultiMapBuilder;
pub use crate::multiset::MultiSet;
pub use crate::multiset_builder::MultiSetBuilder;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// A multi-map that uses `HashMap` for the keys and `HashSet` for the values.
pub type HashMultiMap<K, V, S> = MultiMap<HashMap<K, HashSet<V, S>, S>>;

/// A multi-set that uses `HashMap` for the keys.
pub type HashMultiSet<K, S> = MultiSet<HashMap<K, usize, S>>;

/// A multi-map that uses `BTreeMap` for the keys and `BTreeSet` for the values.
pub type BTreeMultiMap<K, V> = MultiMap<BTreeMap<K, BTreeSet<V>>>;

/// A multi-set that uses `BTreeMap` for the keys.
pub type BTreeMultiSet<K> = MultiSet<BTreeMap<K, usize>>;
