#![feature(impl_trait_in_assoc_type)]
#![warn(missing_docs)]

//! Containers for working with non-unique keys and values.

/// Provides a convenient way to construct multi-maps.
pub mod multimap_builder;

/// Defines the MultiMap type.
pub mod multimap;

/// Traits for working with maps.
pub mod maps;

/// Traits for working with sets.
pub mod sets;

pub use crate::multimap_builder::MultiMapBuilder;
pub use crate::multimap::MultiMap;