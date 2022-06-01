
use crate::RandomState;
use std::collections::{hash_set, HashSet};
use std::fmt::{self, Debug};
use std::hash::{BuildHasher, Hash};
use std::iter::FromIterator;
use std::ops::{BitAnd, BitOr, BitXor, Deref, DerefMut, Sub};

#[cfg(feature = "serde")]
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

/// A [`HashSet`](std::collections::HashSet) using [`RandomState`](crate::RandomState) to hash the items.
/// (Requires the `std` feature to be enabled.)
#[derive(Clone)]
pub struct AHashSet<T, S = RandomState>(HashSet<T, S>);

impl<T> From<HashSet<T, RandomState>> for AHashSet<T> {
    fn from(item: HashSet<T, RandomState>) -> Self {
        AHashSet(item)
    }
}

impl<T, const N: usize> From<[T; N]> for AHashSet<T>
where
    T: Eq + Hash,
{
    /// # Examples
    ///
    /// ```
    /// use ahash::AHashSet;
    ///
    /// let set1 = AHashSet::from([1, 2, 3, 4]);
    /// let set2: AHashSet<_> = [1, 2, 3, 4].into();
    /// assert_eq!(set1, set2);
    /// ```
    fn from(arr: [T; N]) -> Self {
        Self::from_iter(arr)
    }
}

impl<T> Into<HashSet<T, RandomState>> for AHashSet<T> {
    fn into(self) -> HashSet<T, RandomState> {
        self.0
    }
}

impl<T> AHashSet<T, RandomState> {
    /// This crates a hashset using [RandomState::new].
    /// See the documentation in [RandomSource] for notes about key strength.
    pub fn new() -> Self {
        AHashSet(HashSet::with_hasher(RandomState::new()))
    }

    /// This crates a hashset with the specified capacity using [RandomState::new].
    /// See the documentation in [RandomSource] for notes about key strength.