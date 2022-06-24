
#![cfg_attr(feature = "specialize", feature(build_hasher_simple_hash_one))]

use ahash::{AHasher, RandomState};
use criterion::*;
use fxhash::FxHasher;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasherDefault, Hash, Hasher};

// Needs to be in sync with `src/lib.rs`
const AHASH_IMPL: &str = if cfg!(any(
    all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "aes",
        not(miri),
    ),
    all(
        any(target_arch = "arm", target_arch = "aarch64"),
        any(target_feature = "aes", target_feature = "crypto"),
        not(miri),
        feature = "stdsimd",
    ),
)) {
    "aeshash"
} else {
    "fallbackhash"
};

fn ahash<H: Hash>(b: &H) -> u64 {