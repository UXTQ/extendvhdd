
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
                let new = previous.wrapping_add(stack);
                self.counter.store(new, Ordering::Relaxed);
                new
            }
        } else {
            fn gen_hasher_seed(&self) -> usize {
                let stack = self as *const _ as usize;
                self.counter.fetch_add(stack, Ordering::Relaxed)
            }
        }
    }
}

cfg_if::cfg_if! {
        if #[cfg(all(target_arch = "arm", target_os = "none"))] {
            #[inline]
            fn get_src() -> &'static dyn RandomSource {
                static RAND_SOURCE: DefaultRandomSource = DefaultRandomSource::default();
                &RAND_SOURCE
            }
        } else {
            /// Provides an optional way to manually supply a source of randomness for Hasher keys.
            ///
            /// The provided [RandomSource] will be used to be used as a source of randomness by [RandomState] to generate new states.
            /// If this method is not invoked the standard source of randomness is used as described in the Readme.
            ///
            /// The source of randomness can only be set once, and must be set before the first RandomState is created.
            /// If the source has already been specified `Err` is returned with a `bool` indicating if the set failed because
            /// method was previously invoked (true) or if the default source is already being used (false).
            #[cfg(not(all(target_arch = "arm", target_os = "none")))]
            pub fn set_random_source(source: impl RandomSource + Send + Sync + 'static) -> Result<(), bool> {
                RAND_SOURCE.set(Box::new(Box::new(source))).map_err(|s| s.as_ref().type_id() != TypeId::of::<&DefaultRandomSource>())
            }

            #[inline]
            fn get_src() -> &'static dyn RandomSource {
                RAND_SOURCE.get_or_init(|| Box::new(Box::new(DefaultRandomSource::new()))).as_ref()
            }
        }
}

/// Provides a [Hasher] factory. This is typically used (e.g. by [HashMap]) to create
/// [AHasher]s in order to hash the keys of the map. See `build_hasher` below.
///
/// [build_hasher]: ahash::
/// [Hasher]: std::hash::Hasher
/// [BuildHasher]: std::hash::BuildHasher
/// [HashMap]: std::collections::HashMap
///
/// There are multiple constructors each is documented in more detail below:
///
/// | Constructor   | Dynamically random? | Seed |
/// |---------------|---------------------|------|
/// |`new`          | Each instance unique|_[RandomSource]_|
/// |`generate_with`| Each instance unique|`u64` x 4 + [RandomSource]|
/// |`with_seed`    | Fixed per process   |`u64` + static random number|
/// |`with_seeds`   | Fixed               |`u64` x 4|
///
#[derive(Clone)]
pub struct RandomState {
    pub(crate) k0: u64,
    pub(crate) k1: u64,
    pub(crate) k2: u64,
    pub(crate) k3: u64,
}

impl fmt::Debug for RandomState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("RandomState { .. }")
    }
}

impl RandomState {

    /// Create a new `RandomState` `BuildHasher` using random keys.
    ///
    /// Each instance will have a unique set of keys derived from [RandomSource].
    ///
    #[inline]
    pub fn new() -> RandomState {
        let src = get_src();
        let fixed = get_fixed_seeds();
        Self::from_keys(&fixed[0], &fixed[1], src.gen_hasher_seed())
    }

    /// Create a new `RandomState` `BuildHasher` based on the provided seeds, but in such a way
    /// that each time it is called the resulting state will be different and of high quality.
    /// This allows fixed constant or poor quality seeds to be provided without the problem of different
    /// `BuildHasher`s being identical or weak.
    ///
    /// This is done via permuting the provided values with the value of a static counter and memory address.
    /// (This makes this method somewhat more expensive than `with_seeds` below which does not do this).
    ///
    /// The provided values (k0-k3) do not need to be of high quality but they should not all be the same value.
    #[inline]
    pub fn generate_with(k0: u64, k1: u64, k2: u64, k3: u64) -> RandomState {
        let src = get_src();
        let fixed = get_fixed_seeds();
        RandomState::from_keys(&fixed[0], &[k0, k1, k2, k3], src.gen_hasher_seed())
    }

    fn from_keys(a: &[u64; 4], b: &[u64; 4], c: usize) -> RandomState {
        let &[k0, k1, k2, k3] = a;
        let mut hasher = AHasher::from_random_state(&RandomState { k0, k1, k2, k3 });
        hasher.write_usize(c);
        let mix = |l: u64, r: u64| {
            let mut h = hasher.clone();
            h.write_u64(l);
            h.write_u64(r);
            h.finish()
        };
        RandomState {
            k0: mix(b[0], b[2]),
            k1: mix(b[1], b[3]),
            k2: mix(b[2], b[1]),
            k3: mix(b[3], b[0]),
        }
    }

    /// Internal. Used by Default.
    #[inline]
    pub(crate) fn with_fixed_keys() -> RandomState {
        let [k0, k1, k2, k3] = get_fixed_seeds()[0];
        RandomState { k0, k1, k2, k3 }
    }

    /// Build a `RandomState` from a single key. The provided key does not need to be of high quality,
    /// but all `RandomState`s created from the same key will produce identical hashers.
    /// (In contrast to `generate_with` above)