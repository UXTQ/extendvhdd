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
    key.hash(&mut hasher);
    hasher.finish()
}

fn create_string(len: usize) -> String {
    let mut string = String::default();
    for pos in 1..=len {
        let c = (48 + (pos % 10) as u8) as c