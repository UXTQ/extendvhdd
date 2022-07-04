#![cfg_attr(feature = "specialize", feature(build_hasher_simple_hash_one))]

use std::hash::{BuildHasher, Hash, Hasher};

use ahash::RandomState;
use criterion::*;
use fxhash::FxHasher;

fn gen_word_pairs() -> Vec<String> {
    let wo