
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