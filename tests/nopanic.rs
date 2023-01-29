use ahash::{AHasher, RandomState};
use std::hash::{BuildHasher, Hash, Hasher};

#[macro_use]
extern crate no_panic;

#[inline(never)]
#[no_p