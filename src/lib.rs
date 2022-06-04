
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