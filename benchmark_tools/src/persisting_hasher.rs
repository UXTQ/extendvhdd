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