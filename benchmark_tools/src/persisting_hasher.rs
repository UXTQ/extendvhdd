use std::convert::TryInto;
use std::fs::File;
use std::hash::{Hasher, BuildHasher};
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use once_cell::sync::OnceCell;
use std::process::id;

static GLOBAL_COUNT: AtomicU64 = AtomicU64::new(0);
st