#![feature(impl_trait_in_assoc_type)]
#![warn(missing_docs)]

/// Provides a convenient way to construct multi-maps.
pub mod builder;

/// Defines the MultiMap type.
pub mod multimap;

/// Traits for working with maps.
pub mod maps;

/// Traits for working with sets.
pub mod sets;

pub use crate::multimap::MultiMap;
