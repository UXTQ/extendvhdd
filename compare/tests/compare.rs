use ahash::{CallHasher, RandomState};
use criterion::*;
use farmhash::FarmHasher;
use fnv::{FnvBuildHasher};
use fxhash::FxBuildHasher;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};

fn ahash<K: Hash>(k: &K, builder: &RandomState) -> u64 {
    let hasher = builder.build_hasher();
    k.get_hash(hasher)
}

fn generic_hash<K: Hash, B: BuildHasher>(key: &K, builder: &B) -> u64 {
    let mut hasher = builder.build_hasher();
    key.h