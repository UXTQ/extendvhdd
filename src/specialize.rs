
use core::hash::BuildHasher;
use core::hash::Hash;
use core::hash::Hasher;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

#[cfg(feature = "specialize")]
use crate::BuildHasherExt;
#[cfg(feature = "specialize")]
use alloc::string::String;
#[cfg(feature = "specialize")]
use alloc::vec::Vec;

/// Provides a way to get an optimized hasher for a given data type.
/// Rather than using a Hasher generically which can hash any value, this provides a way to get a specialized hash
/// for a specific type. So this may be faster for primitive types.
pub(crate) trait CallHasher {
    fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64;
}

#[cfg(not(feature = "specialize"))]
impl<T> CallHasher for T
where
    T: Hash + ?Sized,
{
    #[inline]
    fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
        let mut hasher = build_hasher.build_hasher();
        value.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(feature = "specialize")]
impl<T> CallHasher for T
where
    T: Hash + ?Sized,
{
    #[inline]
    default fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
        let mut hasher = build_hasher.build_hasher();
        value.hash(&mut hasher);
        hasher.finish()
    }
}

macro_rules! call_hasher_impl {
    ($typ:ty) => {
        #[cfg(feature = "specialize")]
        impl CallHasher for $typ {
            #[inline]
            fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
                build_hasher.hash_as_u64(value)
            }
        }
    };
}
call_hasher_impl!(u8);