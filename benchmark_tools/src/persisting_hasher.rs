use std::convert::TryInto;
use std::fs::File;
use std::hash::{Hasher, BuildHasher};
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use once_cell::sync::OnceCell;
use std::process::id;

static GLOBAL_COUNT: AtomicU64 = AtomicU64::new(0);
static GLOBAL_OUT: OnceCell<Arc<Mutex<BufWriter<File>>>> = OnceCell::new();

pub struct PersistingHasherBuilder {
    id: u64,
    out: Arc<Mutex<BufWriter<File>>>,
}

impl PersistingHasherBuilder {
    pub fn flush(&self) {
        let mut guard = self.out.lock().unwrap();
        guard.flush().unwrap();
    }
}

impl Default for PersistingHasherBuilder {
    fn default() -> Self {
        PersistingHasherBuilder {
            id: GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst),
            out: GLOBAL_OUT.get_or_init(||
                Arc::new(Mutex::new(BufWriter::new(File::create(
                    format!("hash_output-{}", id())).unwrap())))).clone(),
        }
    }
}

impl BuildHasher for PersistingHasherBuilder {
    type Hasher = PersistingHasher;

    fn build_hasher(&self) -> Self::Hasher {
        PersistingHasher {
            hash: self.id,
            out: self.out.clone(),
        }
    }
}

pub struct PersistingHasher {
    /// Used to compute a hash
    hash: u64,
    /// File to write data out to
    out: Arc<Mutex<BufWriter<File>>>,
}

impl PersistingHasher {
    fn add_to_hash(&mut self, i: u64) {
        self.hash = self
            .hash
            .rotate_right(31)
            .wrapping_add(i)
            .wrapping_mul(0xcfee444d8b59a89b);
    }
}

impl Hasher for PersistingHasher {
    fn finish(&self) -> u64 {
        let mut guard = self.out.lock().unwrap();
        write!(guard, "f").unwrap();
        self.hash
    }

    fn write(&mut self, mut bytes: &[u8]) {
        let read_u64 = |bytes: &[u8]| u64::from_ne_bytes(bytes[..8].try_into().unwrap());

        while bytes.len() >= 8 {
            self.add_to_hash(read_u64(bytes));
            bytes = &bytes[8..];
        }
    