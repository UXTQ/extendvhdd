
//! AHash is a high performance keyed hash function.
//!
//! It quickly provides a high quality hash where the result is not predictable without knowing the Key.
//! AHash works with `HashMap` to hash keys, but without allowing for the possibility that an malicious user can
//! induce a collision.
//!
//! # How aHash works
//!
//! When it is available aHash uses the hardware AES instructions to provide a keyed hash function.
//! When it is not, aHash falls back on a slightly slower alternative algorithm.
//!
//! Because aHash does not have a fixed standard for its output, it is able to improve over time.
//! But this also means that different computers or computers using different versions of ahash may observe different
//! hash values for the same input.
#![cfg_attr(
    all(feature = "std", any(feature = "compile-time-rng", feature = "runtime-rng", feature = "no-rng")),
    doc = r##"
# Basic Usage
AHash provides an implementation of the [Hasher] trait.
To construct a HashMap using aHash as its hasher do the following:
```
use ahash::{AHasher, RandomState};
use std::collections::HashMap;

let mut map: HashMap<i32, i32, RandomState> = HashMap::default();
map.insert(12, 34);
```

### Randomness

The above requires a source of randomness to generate keys for the hashmap. By default this obtained from the OS.
It is also possible to have randomness supplied via the `compile-time-rng` flag, or manually.

### If randomess is not available

[AHasher::default()] can be used to hash using fixed keys. This works with
[BuildHasherDefault](std::hash::BuildHasherDefault). For example:

```
use std::hash::BuildHasherDefault;
use std::collections::HashMap;
use ahash::AHasher;

let mut m: HashMap<_, _, BuildHasherDefault<AHasher>> = HashMap::default();
 # m.insert(12, 34);
```
It is also possible to instantiate [RandomState] directly:

```
use ahash::HashMap;
use ahash::RandomState;

let mut m = HashMap::with_hasher(RandomState::with_seed(42));
 # m.insert(1, 2);
```
Or for uses besides a hashhmap:
```
use std::hash::BuildHasher;
use ahash::RandomState;

let hash_builder = RandomState::with_seed(42);
let hash = hash_builder.hash_one("Some Data");
```
There are several constructors for [RandomState] with different ways to supply seeds.

# Convenience wrappers

For convenience, both new-type wrappers and type aliases are provided.

The new type wrappers are called called `AHashMap` and `AHashSet`.
```
use ahash::AHashMap;

let mut map: AHashMap<i32, i32> = AHashMap::new();
map.insert(12, 34);
```
This avoids the need to type "RandomState". (For convience `From`, `Into`, and `Deref` are provided).

# Aliases

For even less typing and better interop with existing libraries (such as rayon) which require a `std::collection::HashMap` ,
the type aliases [HashMap], [HashSet] are provided.

```
use ahash::{HashMap, HashMapExt};

let mut map: HashMap<i32, i32> = HashMap::new();
map.insert(12, 34);
```
Note the import of [HashMapExt]. This is needed for the constructor.

"##
)]
#![deny(clippy::correctness, clippy::complexity, clippy::perf)]
#![allow(clippy::pedantic, clippy::cast_lossless, clippy::unreadable_literal)]
#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![cfg_attr(feature = "specialize", feature(min_specialization))]
#![cfg_attr(feature = "specialize", feature(build_hasher_simple_hash_one))]
#![cfg_attr(feature = "stdsimd", feature(stdsimd))]

#[macro_use]
mod convert;

mod fallback_hash;

cfg_if::cfg_if! {
    if #[cfg(any(
            all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri)),
            all(any(target_arch = "arm", target_arch = "aarch64"),
                any(target_feature = "aes", target_feature = "crypto"),
                not(miri),
                feature = "stdsimd")
            ))] {
        mod aes_hash;
        pub use crate::aes_hash::AHasher;
    } else {
        pub use crate::fallback_hash::AHasher;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        mod hash_map;
        mod hash_set;

        pub use crate::hash_map::AHashMap;
        pub use crate::hash_set::AHashSet;

        /// [Hasher]: std::hash::Hasher
        /// [HashMap]: std::collections::HashMap
        /// Type alias for [HashMap]<K, V, ahash::RandomState>
        pub type HashMap<K, V> = std::collections::HashMap<K, V, crate::RandomState>;

        /// Type alias for [HashSet]<K, ahash::RandomState>
        pub type HashSet<K> = std::collections::HashSet<K, crate::RandomState>;
    }
}

#[cfg(test)]
mod hash_quality_test;

mod operations;
pub mod random_state;
mod specialize;

pub use crate::random_state::RandomState;

use core::hash::BuildHasher;
use core::hash::Hash;
use core::hash::Hasher;

#[cfg(feature = "std")]
/// A convenience trait that can be used together with the type aliases defined to
/// get access to the `new()` and `with_capacity()` methods for the HashMap type alias.
pub trait HashMapExt {
    /// Constructs a new HashMap
    fn new() -> Self;
    /// Constructs a new HashMap with a given initial capacity
    fn with_capacity(capacity: usize) -> Self;
}

#[cfg(feature = "std")]
/// A convenience trait that can be used together with the type aliases defined to
/// get access to the `new()` and `with_capacity()` methods for the HashSet type aliases.
pub trait HashSetExt {