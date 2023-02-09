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
    hasher2.write(string.as_bytes());
    (hasher1.finish(), hasher2.finish())
}

#[inline(never)]
fn hash_test_final_wrapper(num: i32, string: &str) {
    hash_test_final(num, string);
}

struct SimpleBuildHasher {
    hasher: AHasher,
}

impl SimpleBuildHasher {
    fn hash_one<T: Hash>(&self, x: T) -> u64
    where
        Self: Sized,
    {
        let mut hasher = self.build_hasher();
        x.hash(&mut hasher);
        hasher.finish()
    }
}

impl BuildHasher for SimpleBuildHasher {
    type Hasher = AHasher;

    fn build_hasher(&se