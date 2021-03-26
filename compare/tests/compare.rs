use ahash::{CallHasher, RandomState};
use criterion::*;
use farmhash::FarmHasher;
use fnv::{FnvBuildHasher};
use fxhash::FxBuildHasher;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};

fn ahash<K: Hash>(k: &K, builder: &RandomState) -> u64 {
    let hasher = builder.build_hasher();
    k.get_hash(hasher)
}

fn generic_hash<K: Hash, B: BuildHasher>(key: &K, builder: &B) -> u64 {
    let mut hasher = builder.build_hasher();
    key.hash(&mut hasher);
    hasher.finish()
}

fn create_string(len: usize) -> String {
    let mut string = String::default();
    for pos in 1..=len {
        let c = (48 + (pos % 10) as u8) as char;
        string.push(c);
    }
    string
}

fn compare_ahash(c: &mut Criterion) {
    let builder = RandomState::new();
    let test = "compare_ahash";
    for num in &[1,3,7,15,31,63,127,255,511,1023] {
        let name = "string".to_owned() + &num.to_string();
        let string = create_string(*num);
        c.bench_with_input(BenchmarkId::new(test, &name), &string, |bencher, s| {
            bencher.iter(|| {
                black_box(ahash(s, &builder))
            });
        });
    }
}

fn compare_other<B: BuildHasher>(c: &mut Criterion, test: &str, builder: B) {
    for num in &[1,3,7,15,31,63,127,255,511,1023] {
        let name = "string".to_owned() + &num.to_string();
        let string = create_string(*num);
        c.bench_with_input(BenchmarkId::new(test, &name), &string, |bencher, s| {
            bencher.iter(|| {
                black_box(generic_hash(&s, &builder))
            });
        });
    }
}

fn compare_farmhash(c: &mut Criterion) {
    let int: u64 = 1234;
    let string = create_string(1024);
    let builder = BuildHasherDefault::<FarmHasher>::default();
    compare_other(c, "compare_farmhash", builder)
}

fn compare_fnvhash(c: &mut Criterion) {
    let int: u64 = 1234;
    let string = create_string(1024);
    let builder = FnvBuildHasher::default();
    compare_other(c, "