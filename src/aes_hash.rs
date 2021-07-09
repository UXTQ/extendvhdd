
use crate::convert::*;
use crate::operations::*;
use crate::random_state::PI;
use crate::RandomState;
use core::hash::Hasher;

/// A `Hasher` for hashing an arbitrary stream of bytes.
///
/// Instances of [`AHasher`] represent state that is updated while hashing data.
///
/// Each method updates the internal state based on the new data provided. Once
/// all of the data has been provided, the resulting hash can be obtained by calling
/// `finish()`
///
/// [Clone] is also provided in case you wish to calculate hashes for two different items that
/// start with the same data.
///
#[derive(Debug, Clone)]
pub struct AHasher {
    enc: u128,
    sum: u128,
    key: u128,
}

impl AHasher {
    /// Creates a new hasher keyed to the provided keys.
    ///
    /// Normally hashers are created via `AHasher::default()` for fixed keys or `RandomState::new()` for randomly
    /// generated keys and `RandomState::with_seeds(a,b)` for seeds that are set and can be reused. All of these work at
    /// map creation time (and hence don't have any overhead on a per-item bais).
    ///
    /// This method directly creates the hasher instance and performs no transformation on the provided seeds. This may
    /// be useful where a HashBuilder is not desired, such as for testing purposes.
    ///
    /// # Example
    ///
    /// ```
    /// use std::hash::Hasher;
    /// use ahash::AHasher;
    ///
    /// let mut hasher = AHasher::new_with_keys(1234, 5678);
    ///
    /// hasher.write_u32(1989);
    /// hasher.write_u8(11);
    /// hasher.write_u8(9);
    /// hasher.write(b"Huh?");
    ///
    /// println!("Hash is {:x}!", hasher.finish());
    /// ```
    #[inline]
    pub(crate) fn new_with_keys(key1: u128, key2: u128) -> Self {
        let pi: [u128; 2] = PI.convert();
        let key1 = key1 ^ pi[0];
        let key2 = key2 ^ pi[1];
        Self {
            enc: key1,
            sum: key2,
            key: key1 ^ key2,
        }
    }

    #[allow(unused)] // False positive
    pub(crate) fn test_with_keys(key1: u128, key2: u128) -> Self {
        Self {
            enc: key1,
            sum: key2,
            key: key1 ^ key2,
        }
    }

    #[inline]
    pub(crate) fn from_random_state(rand_state: &RandomState) -> Self {
        let key1 = [rand_state.k0, rand_state.k1].convert();
        let key2 = [rand_state.k2, rand_state.k3].convert();
        Self {
            enc: key1,
            sum: key2,
            key: key1 ^ key2,
        }
    }

    #[inline(always)]
    fn hash_in(&mut self, new_value: u128) {
        self.enc = aesenc(self.enc, new_value);
        self.sum = shuffle_and_add(self.sum, new_value);
    }

    #[inline(always)]
    fn hash_in_2(&mut self, v1: u128, v2: u128) {
        self.enc = aesenc(self.enc, v1);
        self.sum = shuffle_and_add(self.sum, v1);
        self.enc = aesenc(self.enc, v2);
        self.sum = shuffle_and_add(self.sum, v2);
    }

    #[inline]
    #[cfg(feature = "specialize")]
    fn short_finish(&self) -> u64 {
        let combined = aesdec(self.sum, self.enc);
        let result: [u64; 2] = aesenc(combined, combined).convert();
        result[0]
    }
}

/// Provides [Hasher] methods to hash all of the primitive types.
///
/// [Hasher]: core::hash::Hasher
impl Hasher for AHasher {
    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.hash_in(i);
    }

    #[inline]
    #[cfg(any(
        target_pointer_width = "64",
        target_pointer_width = "32",
        target_pointer_width = "16"
    ))]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }

    #[inline]
    #[cfg(target_pointer_width = "128")]
    fn write_usize(&mut self, i: usize) {
        self.write_u128(i as u128);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.write_u128(i as u128);
    }

    #[inline]
    #[allow(clippy::collapsible_if)]
    fn write(&mut self, input: &[u8]) {
        let mut data = input;
        let length = data.len();
        add_in_length(&mut self.enc, length as u64);

        //A 'binary search' on sizes reduces the number of comparisons.
        if data.len() <= 8 {
            let value = read_small(data);
            self.hash_in(value.convert());
        } else {
            if data.len() > 32 {
                if data.len() > 64 {
                    let tail = data.read_last_u128x4();
                    let mut current: [u128; 4] = [self.key; 4];
                    current[0] = aesenc(current[0], tail[0]);
                    current[1] = aesenc(current[1], tail[1]);
                    current[2] = aesenc(current[2], tail[2]);
                    current[3] = aesenc(current[3], tail[3]);
                    let mut sum: [u128; 2] = [self.key, self.key];
                    sum[0] = add_by_64s(sum[0].convert(), tail[0].convert()).convert();
                    sum[1] = add_by_64s(sum[1].convert(), tail[1].convert()).convert();
                    sum[0] = shuffle_and_add(sum[0], tail[2]);
                    sum[1] = shuffle_and_add(sum[1], tail[3]);
                    while data.len() > 64 {
                        let (blocks, rest) = data.read_u128x4();
                        current[0] = aesenc(current[0], blocks[0]);