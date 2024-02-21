#![feature(impl_trait_in_assoc_type)]
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

mod test_utils;
mod examples;

pub use crate::multimap::MultiMap;
pub use crate::multimap_builder::MultiMapBuilder;
pub use crate::multiset::MultiSet;
pub use crate::multiset_builder::MultiSetBuilder;
