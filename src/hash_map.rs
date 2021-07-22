
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
}

impl<K, V, S> AHashMap<K, V, S>
where
    S: BuildHasher,
{
    pub fn with_hasher(hash_builder: S) -> Self {
        AHashMap(HashMap::with_hasher(hash_builder))
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        AHashMap(HashMap::with_capacity_and_hasher(capacity, hash_builder))
    }
}

impl<K, V, S> AHashMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get(&1), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    #[inline]
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get(k)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
    /// assert_eq!(map.get_key_value(&2), None);
    /// ```
    #[inline]
    pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get_key_value(k)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// if let Some(x) = map.get_mut(&1) {
    ///     *x = "b";
    /// }
    /// assert_eq!(map[&1], "b");
    /// ```
    #[inline]
    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get_mut(k)
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical. See the [module-level
    /// documentation] for more.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// assert_eq!(map.insert(37, "a"), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b");
    /// assert_eq!(map.insert(37, "c"), Some("b"));
    /// assert_eq!(map[&37], "c");
    /// ```
    #[inline]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.0.insert(k, v)
    }

    /// Creates a consuming iterator visiting all the keys in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `K`.
    ///