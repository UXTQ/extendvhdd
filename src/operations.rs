
use crate::convert::*;

///This constant comes from Kunth's prng (Empirically it works better than those from splitmix32).
pub(crate) const MULTIPLE: u64 = 6364136223846793005;

/// This is a constant with a lot of special properties found by automated search.
/// See the unit tests below. (Below are alternative values)
#[cfg(all(target_feature = "ssse3", not(miri)))]
const SHUFFLE_MASK: u128 = 0x020a0700_0c01030e_050f0d08_06090b04_u128;
//const SHUFFLE_MASK: u128 = 0x000d0702_0a040301_05080f0c_0e0b0609_u128;
//const SHUFFLE_MASK: u128 = 0x040A0700_030E0106_0D050F08_020B0C09_u128;

#[inline(always)]
#[cfg(feature = "folded_multiply")]
pub(crate) const fn folded_multiply(s: u64, by: u64) -> u64 {
    let result = (s as u128).wrapping_mul(by as u128);
    ((result & 0xffff_ffff_ffff_ffff) as u64) ^ ((result >> 64) as u64)
}

#[inline(always)]
#[cfg(not(feature = "folded_multiply"))]
pub(crate) const fn folded_multiply(s: u64, by: u64) -> u64 {
    let b1 = s.wrapping_mul(by.swap_bytes());
    let b2 = s.swap_bytes().wrapping_mul(!by);
    b1 ^ b2.swap_bytes()
}

/// Given a small (less than 8 byte slice) returns the same data stored in two u32s.
/// (order of and non-duplication of bytes is NOT guaranteed)
#[inline(always)]
pub(crate) fn read_small(data: &[u8]) -> [u64; 2] {
    debug_assert!(data.len() <= 8);
    if data.len() >= 2 {
        if data.len() >= 4 {
            //len 4-8