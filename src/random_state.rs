
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