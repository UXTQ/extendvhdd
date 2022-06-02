
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
    pub fn with_capacity(capacity: usize) -> Self {
        AHashSet(HashSet::with_capacity_and_hasher(capacity, RandomState::new()))
    }
}

impl<T, S> AHashSet<T, S>
where
    S: BuildHasher,
{
    pub fn with_hasher(hash_builder: S) -> Self {
        AHashSet(HashSet::with_hasher(hash_builder))
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        AHashSet(HashSet::with_capacity_and_hasher(capacity, hash_builder))
    }
}

impl<T, S> Deref for AHashSet<T, S> {
    type Target = HashSet<T, S>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> DerefMut for AHashSet<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, S> PartialEq for AHashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn eq(&self, other: &AHashSet<T, S>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T, S> Eq for AHashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
}

impl<T, S> BitOr<&AHashSet<T, S>> for &AHashSet<T, S>
where
    T: Eq + Hash + Clone,
    S: BuildHasher + Default,
{
    type Output = AHashSet<T, S>;

    /// Returns the union of `self` and `rhs` as a new `AHashSet<T, S>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ahash::AHashSet;
    ///
    /// let a: AHashSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: AHashSet<_> = vec![3, 4, 5].into_iter().collect();
    ///
    /// let set = &a | &b;
    ///
    /// let mut i = 0;
    /// let expected = [1, 2, 3, 4, 5];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitor(self, rhs: &AHashSet<T, S>) -> AHashSet<T, S> {
        AHashSet(self.0.bitor(&rhs.0))
    }
}

impl<T, S> BitAnd<&AHashSet<T, S>> for &AHashSet<T, S>
where
    T: Eq + Hash + Clone,
    S: BuildHasher + Default,
{
    type Output = AHashSet<T, S>;

    /// Returns the intersection of `self` and `rhs` as a new `AHashSet<T, S>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ahash::AHashSet;
    ///
    /// let a: AHashSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: AHashSet<_> = vec![2, 3, 4].into_iter().collect();
    ///
    /// let set = &a & &b;
    ///
    /// let mut i = 0;
    /// let expected = [2, 3];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitand(self, rhs: &AHashSet<T, S>) -> AHashSet<T, S> {
        AHashSet(self.0.bitand(&rhs.0))
    }
}

impl<T, S> BitXor<&AHashSet<T, S>> for &AHashSet<T, S>