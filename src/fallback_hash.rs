
use crate::convert::*;
use crate::operations::folded_multiply;
use crate::operations::read_small;
use crate::operations::MULTIPLE;
use crate::random_state::PI;
use crate::RandomState;
use core::hash::Hasher;

const ROT: u32 = 23; //17

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
    buffer: u64,
    pad: u64,
    extra_keys: [u64; 2],
}

impl AHasher {
    /// Creates a new hasher keyed to the provided key.
    #[inline]
    #[allow(dead_code)] // Is not called if non-fallback hash is used.
    pub(crate) fn new_with_keys(key1: u128, key2: u128) -> AHasher {
        let pi: [u128; 2] = PI.convert();
        let key1: [u64; 2] = (key1 ^ pi[0]).convert();
        let key2: [u64; 2] = (key2 ^ pi[1]).convert();
        AHasher {
            buffer: key1[0],
            pad: key1[1],
            extra_keys: key2,
        }
    }

    #[allow(unused)] // False positive
    pub(crate) fn test_with_keys(key1: u128, key2: u128) -> Self {
        let key1: [u64; 2] = key1.convert();
        let key2: [u64; 2] = key2.convert();