#![feature(build_hasher_simple_hash_one)]

use ahash::*;
use core::slice;
use std::hash::{BuildHasher};

#[no_mangle]
pub extern "C" fn ah