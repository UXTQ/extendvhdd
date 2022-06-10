
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
            [data.read_u32().0 as u64, data.read_last_u32() as u64]
        } else {
            //len 2-3
            [data.read_u16().0 as u64, data[data.len() - 1] as u64]
        }
    } else {
        if data.len() > 0 {
            [data[0] as u64, data[0] as u64]
        } else {
            [0, 0]
        }
    }
}

#[inline(always)]
pub(crate) fn shuffle(a: u128) -> u128 {
    #[cfg(all(target_feature = "ssse3", not(miri)))]
    {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;
        use core::mem::transmute;
        unsafe { transmute(_mm_shuffle_epi8(transmute(a), transmute(SHUFFLE_MASK))) }
    }
    #[cfg(not(all(target_feature = "ssse3", not(miri))))]
    {
        a.swap_bytes()
    }
}

#[allow(unused)] //not used by fallback
#[inline(always)]
pub(crate) fn add_and_shuffle(a: u128, b: u128) -> u128 {
    let sum = add_by_64s(a.convert(), b.convert());
    shuffle(sum.convert())
}

#[allow(unused)] //not used by fallback
#[inline(always)]
pub(crate) fn shuffle_and_add(base: u128, to_add: u128) -> u128 {
    let shuffled: [u64; 2] = shuffle(base).convert();
    add_by_64s(shuffled, to_add.convert()).convert()
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(miri)))]
#[inline(always)]
pub(crate) fn add_by_64s(a: [u64; 2], b: [u64; 2]) -> [u64; 2] {
    use core::mem::transmute;
    unsafe {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;
        transmute(_mm_add_epi64(transmute(a), transmute(b)))
    }
}

#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(miri))))]
#[inline(always)]
pub(crate) fn add_by_64s(a: [u64; 2], b: [u64; 2]) -> [u64; 2] {
    [a[0].wrapping_add(b[0]), a[1].wrapping_add(b[1])]
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri)))]
#[allow(unused)]
#[inline(always)]
pub(crate) fn aesenc(value: u128, xor: u128) -> u128 {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    use core::mem::transmute;
    unsafe {
        let value = transmute(value);
        transmute(_mm_aesenc_si128(value, transmute(xor)))
    }
}

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    any(target_feature = "aes", target_feature = "crypto"),
    not(miri),
    feature = "stdsimd"
))]
#[allow(unused)]
#[inline(always)]
pub(crate) fn aesenc(value: u128, xor: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    use core::arch::aarch64::*;
    #[cfg(target_arch = "arm")]
    use core::arch::arm::*;
    use core::mem::transmute;
    unsafe {
        let value = transmute(value);
        transmute(vaesmcq_u8(vaeseq_u8(value, transmute(xor))))
    }
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri)))]
#[allow(unused)]
#[inline(always)]
pub(crate) fn aesdec(value: u128, xor: u128) -> u128 {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    use core::mem::transmute;
    unsafe {
        let value = transmute(value);
        transmute(_mm_aesdec_si128(value, transmute(xor)))
    }
}

#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64"),
    any(target_feature = "aes", target_feature = "crypto"),
    not(miri),
    feature = "stdsimd"
))]