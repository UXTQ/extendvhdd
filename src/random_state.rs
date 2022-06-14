
use core::hash::Hash;
cfg_if::cfg_if! {
    if #[cfg(any(
        all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri)),
         all(any(target_arch = "arm", target_arch = "aarch64"), any(target_feature = "aes", target_feature = "crypto"), not(miri), feature = "stdsimd")
    ))] {
        use crate::aes_hash::*;
    } else {
        use crate::fallback_hash::*;
    }
}
cfg_if::cfg_if! {
    if #[cfg(feature = "specialize")]{
        use crate::BuildHasherExt;
    }
}
cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        extern crate std as alloc;
    } else {
        extern crate alloc;
    }
}

#[cfg(feature = "atomic-polyfill")]
use atomic_polyfill as atomic;
#[cfg(not(feature = "atomic-polyfill"))]
use core::sync::atomic;

use alloc::boxed::Box;
use atomic::{AtomicUsize, Ordering};
use core::any::{Any, TypeId};
use core::fmt;
use core::hash::BuildHasher;
use core::hash::Hasher;

pub(crate) const PI: [u64; 4] = [
    0x243f_6a88_85a3_08d3,
    0x1319_8a2e_0370_7344,
    0xa409_3822_299f_31d0,
    0x082e_fa98_ec4e_6c89,
];

pub(crate) const PI2: [u64; 4] = [
    0x4528_21e6_38d0_1377,
    0xbe54_66cf_34e9_0c6c,
    0xc0ac_29b7_c97c_50dd,
    0x3f84_d5b5_b547_0917,
];

cfg_if::cfg_if! {
    if #[cfg(all(feature = "compile-time-rng", any(test, fuzzing)))] {
        #[inline]
        fn get_fixed_seeds() -> &'static [[u64; 4]; 2] {
            use const_random::const_random;

            const RAND: [[u64; 4]; 2] = [
                [
                    const_random!(u64),
                    const_random!(u64),
                    const_random!(u64),
                    const_random!(u64),
                ], [
                    const_random!(u64),
                    const_random!(u64),
                    const_random!(u64),
                    const_random!(u64),
                ]
            ];
            &RAND
        }
    } else if #[cfg(all(feature = "runtime-rng", not(fuzzing)))] {
        #[inline]
        fn get_fixed_seeds() -> &'static [[u64; 4]; 2] {
            use crate::convert::Convert;

            static SEEDS: OnceBox<[[u64; 4]; 2]> = OnceBox::new();

            SEEDS.get_or_init(|| {
                let mut result: [u8; 64] = [0; 64];
                getrandom::getrandom(&mut result).expect("getrandom::getrandom() failed.");
                Box::new(result.convert())
            })
        }
    } else if #[cfg(feature = "compile-time-rng")] {
        #[inline]
        fn get_fixed_seeds() -> &'static [[u64; 4]; 2] {
            use const_random::const_random;

            const RAND: [[u64; 4]; 2] = [
                [