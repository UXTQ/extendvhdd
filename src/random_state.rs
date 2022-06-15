
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
    } else {
        #[inline]
        fn get_fixed_seeds() -> &'static [[u64; 4]; 2] {
            &[PI, PI2]
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(not(all(target_arch = "arm", target_os = "none")))] {
        use once_cell::race::OnceBox;

        static RAND_SOURCE: OnceBox<Box<dyn RandomSource + Send + Sync>> = OnceBox::new();
    }
}
/// A supplier of Randomness used for different hashers.
/// See [set_random_source].
///
/// If [set_random_source] aHash will default to the best available source of randomness.
/// In order this is:
/// 1. OS provided random number generator (available if the `runtime-rng` flag is enabled which it is by default) - This should be very strong.
/// 2. Strong compile time random numbers used to permute a static "counter". (available if `compile-time-rng` is enabled.
/// __Enabling this is recommended if `runtime-rng` is not possible__)
/// 3. A static counter that adds the memory address of each [RandomState] created permuted with fixed constants.
/// (Similar to above but with fixed keys) - This is the weakest option. The strength of this heavily depends on whether or not ASLR is enabled.
/// (Rust enables ASLR by default)
pub trait RandomSource {
    fn gen_hasher_seed(&self) -> usize;
}

struct DefaultRandomSource {
    counter: AtomicUsize,
}

impl DefaultRandomSource {
    fn new() -> DefaultRandomSource {
        DefaultRandomSource {
            counter: AtomicUsize::new(&PI as *const _ as usize),
        }
    }

    #[cfg(all(target_arch = "arm", target_os = "none"))]
    const fn default() -> DefaultRandomSource {
        DefaultRandomSource {
            counter: AtomicUsize::new(PI[3] as usize),
        }
    }
}

impl RandomSource for DefaultRandomSource {
    cfg_if::cfg_if! {
        if #[cfg(all(target_arch = "arm", target_os = "none"))] {
            fn gen_hasher_seed(&self) -> usize {
                let stack = self as *const _ as usize;
                let previous = self.counter.load(Ordering::Relaxed);