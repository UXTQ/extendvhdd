
use std::borrow::Borrow;
use std::collections::hash_map::{IntoKeys, IntoValues};
use std::collections::{hash_map, HashMap};
use std::fmt::{self, Debug};
use std::hash::{BuildHasher, Hash};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut, Index};
use std::panic::UnwindSafe;

#[cfg(feature = "serde")]
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

use crate::RandomState;

/// A [`HashMap`](std::collections::HashMap) using [`RandomState`](crate::RandomState) to hash the items.
/// (Requires the `std` feature to be enabled.)
#[derive(Clone)]
pub struct AHashMap<K, V, S = crate::RandomState>(HashMap<K, V, S>);

impl<K, V> From<HashMap<K, V, crate::RandomState>> for AHashMap<K, V> {
    fn from(item: HashMap<K, V, crate::RandomState>) -> Self {
        AHashMap(item)
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for AHashMap<K, V>
where
    K: Eq + Hash,
{
    /// # Examples
    ///
    /// ```
    /// use ahash::AHashMap;
    ///
    /// let map1 = AHashMap::from([(1, 2), (3, 4)]);
    /// let map2: AHashMap<_, _> = [(1, 2), (3, 4)].into();
    /// assert_eq!(map1, map2);
    /// ```
    fn from(arr: [(K, V); N]) -> Self {
        Self::from_iter(arr)
    }
}

impl<K, V> Into<HashMap<K, V, crate::RandomState>> for AHashMap<K, V> {
    fn into(self) -> HashMap<K, V, crate::RandomState> {
        self.0
    }
}

impl<K, V> AHashMap<K, V, RandomState> {
    /// This crates a hashmap using [RandomState::new] which obtains its keys from [RandomSource].
    /// See the documentation in [RandomSource] for notes about key strength.
    pub fn new() -> Self {
        AHashMap(HashMap::with_hasher(RandomState::new()))
    }

    /// This crates a hashmap with the specified capacity using [RandomState::new].
    /// See the documentation in [RandomSource] for notes about key strength.
    pub fn with_capacity(capacity: usize) -> Self {
        AHashMap(HashMap::with_capacity_and_hasher(capacity, RandomState::new()))
    }