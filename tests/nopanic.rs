use ahash::{AHasher, RandomState};
use std::hash::{BuildHasher, Hash, Hasher};

#[macro_use]
extern crate no_panic;

#[inline(never)]
#[no_panic]
fn hash_test_final(num: i32, string: &str) -> (u64, u64) {
    use core::hash::Hasher;
    let mut hasher1 = RandomState::with_seeds(1, 2, 3, 4).build_hasher();
    let mut hasher2 = RandomState::with_seeds(3, 4, 5, 6).build_hasher();
    hasher1.write_i32(num);
