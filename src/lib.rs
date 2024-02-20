#![feature(impl_trait_in_assoc_type)]
#![warn(missing_docs)]

pub mod builder;
pub mod maps;
pub mod multimap;
pub mod sets;

pub use crate::multimap::MultiMap;
