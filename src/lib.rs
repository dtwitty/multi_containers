#![warn(missing_docs)]

//! This crate implements containers that can have duplicate values.
//! If you have ever written a `HashMap<K, HashSet<V>>`, this crate is for you.
//! This crate is unstable and its API is subject to change at any time until 1.0.0.
//!
//! This crate is comparable in spirit to these containers in other languages:
//! - Java's `Guava` library's `Multimap` and `Multiset` (which heavily inspired this crate).
//! - Python's `collections.defaultdict(set)` and `collections.Counter`.
//! - C++'s `std::(unordered_)mutlimap` and `std::(unordered_)multiset`.
//!
//! ## Usage
//! The primary containers are `MultiMap` and `MultiSet`. See `examples.rs` for more examples.
//!
//! ### MultiMap
//! `MultiMap` is a wrapper around `Map<K, Set<V>>`.
//! You can either use the provided `HashMultiMap` or `BTreeMultiMap`, or provide your own types with `MultiMapBuilder`.
//! The API is similar to what you would expect from `HashMap<K, HashSet<V>>`, with some additional methods related to the multiple values.
//! For bookkeeping reasons, the inner sets are not exposed mutably.
//! `MultiMap` also provides ways of iterating over `(&K, &Set<V>)`, or over `(&K, &V)`.
//!
//! ```rust
//! use multi_containers::HashMultiMap;
//! let mut map = HashMultiMap::new();
//! map.insert("a".to_string(), 1);
//! map.insert("a".to_string(), 2);
//! map.insert("b".to_string(), 3);
//! assert_eq!(map.get("a").len(), 2);
//! assert_eq!(map.get("b").len(), 1);
//! ```
//!
//! ### MultiSet
//! `MultiSet` is a wrapper around `Map<V, usize>`. It offers the semantics of a set, but allows for duplicate values.
//! It offers iterators over unique `(&V, usize)`, and non-unique `&V`.
//!
//! ```rust
//! use multi_containers::HashMultiSet;
//! let mut set = HashMultiSet::new();
//! set.insert(1);
//! set.insert(1);
//! set.insert_some(2, 3);
//! assert_eq!(set.count(&1), 2);
//! assert_eq!(set.count(&2), 3);
//! ```
//!
//!
//! ## To-Do
//! - [ ] Add doctests and usage examples.
//! - [ ] Implement common traits like `Extend`. This is blocked on `impl_trait_in_assoc_type` being stabilized.
//! - [ ] Collect user feedback and improve the API before 1.0.0.
//! - [ ] Explore concurrency options.

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
