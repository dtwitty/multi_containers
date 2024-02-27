# multi_containers

[![crates.io](https://img.shields.io/crates/v/multi_containers.svg)](https://crates.io/crates/multi_containers)
[![docs](https://docs.rs/multi_containers/badge.svg)](https://docs.rs/multi_containers)

This crate implements containers that can have duplicate values.
If you have ever written a `HashMap<K, HashSet<V>>`, this crate is for you.

**This crate is unstable and its API is subject to change at any time until 1.0.0**.

This crate is comparable in spirit to these containers in other languages:
- Java's `Guava` library's `Multimap` and `Multiset` (which heavily inspired this crate).
- Python's `collections.defaultdict(set)` and `collections.Counter`.
- C++'s `std::(unordered_)mutlimap` and `std::(unordered_)multiset`.

### Usage
The primary containers are `MultiMap` and `MultiSet`. See `examples.rs` for more examples.

#### MultiMap
`MultiMap` is a wrapper around `Map<K, Set<V>>`.
You can either use the provided `HashMultiMap` or `BTreeMultiMap`, or provide your own types with `MultiMapBuilder`.
The API is similar to what you would expect from `HashMap<K, HashSet<V>>`, with some additional methods related to the multiple values.
For bookkeeping reasons, the inner sets are not exposed mutably.
`MultiMap` also provides ways of iterating over `(&K, &Set<V>)`, or over `(&K, &V)`.

```rust
use multi_containers::HashMultiMap;
let mut map = HashMultiMap::new();
map.insert("a".to_string(), 1);
map.insert("a".to_string(), 2);
map.insert("b".to_string(), 3);
assert_eq!(map.get("a").unwrap().len(), 2);
assert_eq!(map.get("b").unwrap().len(), 1);
```

#### MultiSet
`MultiSet` is a wrapper around `Map<V, usize>`. It offers the semantics of a set, but allows for duplicate values.
It offers iterators over unique `(&V, usize)`, and non-unique `&V`.

```rust
use multi_containers::HashMultiSet;
let mut set = HashMultiSet::new();
set.insert(1);
set.insert(1);
set.insert_some(2, 3);
assert_eq!(set.count(&1), 2);
assert_eq!(set.count(&2), 3);
```

License: MIT
