
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
    let build_hasher = RandomState::with_seeds(1, 2, 3, 4);
    build_hasher.hash_one(b)
}

fn fnvhash<H: Hash>(b: &H) -> u64 {
    let mut hasher = fnv::FnvHasher::default();
    b.hash(&mut hasher);
    hasher.finish()
}

fn siphash<H: Hash>(b: &H) -> u64 {
    let mut hasher = DefaultHasher::default();
    b.hash(&mut hasher);
    hasher.finish()
}

fn fxhash<H: Hash>(b: &H) -> u64 {
    let mut hasher = FxHasher::default();
    b.hash(&mut hasher);
    hasher.finish()
}

fn seahash<H: Hash>(b: &H) -> u64 {
    let mut hasher = seahash::SeaHasher::default();
    b.hash(&mut hasher);
    hasher.finish()
}

const STRING_LENGTHS: [u32; 12] = [1, 3, 4, 7, 8, 15, 16, 24, 33, 68, 132, 1024];

fn gen_strings() -> Vec<String> {
    STRING_LENGTHS
        .iter()
        .map(|len| {
            let mut string = String::default();
            for pos in 1..=*len {
                let c = (48 + (pos % 10) as u8) as char;
                string.push(c);
            }
            string
        })
        .collect()
}

macro_rules! bench_inputs {
    ($group:ident, $hash:ident) => {
        // Number of iterations per batch should be high enough to hide timing overhead.
        let size = BatchSize::NumIterations(2_000);

        let mut rng = rand::thread_rng();
        $group.bench_function("u8", |b| b.iter_batched(|| rng.gen::<u8>(), |v| $hash(&v), size));
        $group.bench_function("u16", |b| b.iter_batched(|| rng.gen::<u16>(), |v| $hash(&v), size));
        $group.bench_function("u32", |b| b.iter_batched(|| rng.gen::<u32>(), |v| $hash(&v), size));
        $group.bench_function("u64", |b| b.iter_batched(|| rng.gen::<u64>(), |v| $hash(&v), size));
        $group.bench_function("u128", |b| b.iter_batched(|| rng.gen::<u128>(), |v| $hash(&v), size));
        $group.bench_with_input("strings", &gen_strings(), |b, s| b.iter(|| $hash(black_box(s))));
    };
}

fn bench_ahash(c: &mut Criterion) {
    let mut group = c.benchmark_group(AHASH_IMPL);
    bench_inputs!(group, ahash);
}

fn bench_fx(c: &mut Criterion) {
    let mut group = c.benchmark_group("fx");
    bench_inputs!(group, fxhash);