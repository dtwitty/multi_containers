# Multi-Containers
This crate implements containers that can have duplicate values.
It is heavily inspired by Guava's `MultiSet` and `MultiMap`.

## Usage
See `exmaples.rs` for usage examples.

## To-Do
Given the trait-heavy nature of this crate, it is difficult to implement certain traits like `IntoIterator`.
These will be implemented when `impl_trait_in_assoc_type` is stabilized.