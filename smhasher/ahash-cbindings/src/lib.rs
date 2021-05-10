#![feature(build_hasher_simple_hash_one)]

use ahash::*;
use core::slice;
use std::hash::{BuildHasher};

#[no_mangle]
pub extern "C" fn ahash64(buf: *const (), len: usize, seed: u64) -> u64 {
    let buf: &[u8] = unsafe { slice::from_raw_parts(buf as *const