use core::hash::{Hash, Hasher};
use std::collections::HashMap;

fn assert_sufficiently_different(a: u64, b: u64, tolerance: i32) {
    let (same_byte_count, same_nibble_count) = count_same_bytes_and_nibbles(a, b);
    assert!(same_byte_count <= tolerance, "{:x} vs {:x}: {:}", a, b, same_byte_count);
    assert!(
        same_nibble_count <= tolerance * 3,
        "{:x} vs {:x}: {:}",
        a,
        b,
        same_nibble_count
    );
    let flipped_bits = (a ^ b).count_ones();
    assert!(
        flipped_bits > 12 && flipped_bits < 52,
        "{:x} and {:x}: {:}",
        a,
        b,
        flipped_bits
    );
    for rotate in 0..64 {
        let flipped_bits2 = (a ^ (b.rotate_left(rotate))).count_ones();
        assert!(
            flipped_bits2 > 10 && flipped_bits2 < 54,
            "{:x} and {:x}: {:}",
            a,
            b.rotate_left(rotate),
            flipped_bits2
        );
    }
}

fn count_same_bytes_and_nibbles(a: u64, b: u64) -> (i32, i32) {
    let mut same_byte_count = 0;
    let mut same_nibble_count = 0;
    for byte in 0..8 {
        let ba = (a >> (8 * byte)) as u8;
        let bb = (b >> (8 * byte)) as u8;
        if ba == bb {
            same_byte_count += 1;
        }
        if ba & 0xF0u8 == bb & 0xF0u8 {
            same_nibble_count += 1;
        }
        if ba & 0x0Fu8 == bb & 0x0Fu8 {
            same_nibble_count += 1;
        }
    }
    (same_byte_count, same_nibble_count)
}

fn gen_combinations(options: &[u32; 11], depth: u32, so_far: Vec<u32>, combinations: &mut Vec<Vec<u32>>) {
    if depth == 0 {
        return;
    }
    for option in options {
        let mut next = so_far.clone();
        next.push(*option);
        combinations.push(next.clone());
        gen_combinations(options, depth - 1, next, combinations);
    }
}

fn test_no_full_collisions<T: Hasher>(gen_hash: impl Fn() -> T) {
    let options: [u32; 11] = [
        0x00000000, 0x10000000, 0x20000000, 0x40000000, 0x80000000, 0xF0000000,
        1, 2, 4, 8, 15
    ];
    let mut combinations = Vec::new();
    gen_combinations(&options, 7, Vec::new(), &mut combinations);
    let mut map: HashMap<u64, Vec<u8>> = HashMap::new();
    for combination in combinations {
        let array = unsafe {
            let (begin, middle, end) = combination.align_to::<u8>();
            assert_eq!(0, begin.len());
            assert_eq!(0, end.len());
            middle.to_vec()
        };
        let mut hasher = gen_hash();
        hasher.write(&array);
        let hash = hasher.finish();
        if let Some(value) = map.get(&hash) {
            assert_eq!(
                value, &array,
                "Found a collision between {:x?} and {:x?}. Hash: {:x?}",
                value, &array, &hash
            );
        } else {
            map.insert(hash, array);
        }
    }
    assert_eq!(21435887, map.len()); //11^7 + 11^6 ...
}

fn test_keys_change_output<T: Hasher>(constructor: impl Fn(u128, u128) -> T) {
    let mut a = constructor(1, 1);
    let mut b = constructor(1, 2);
    let mut c = constructor(2, 1);
    let mut d = constructor(2, 2);
    "test".hash(&mut a);
    "test".hash(&mut b);
    "test".hash(&mut c);
    "test".hash(&mut d);
    assert_sufficiently_different(a.finish(), b.finish(), 1);
    assert_sufficiently_different(a.finish(), c.finish(), 1);
    assert_sufficiently_different(a.finish(), d.finish(), 1);
    assert_sufficiently_different(b.finish(), c.finish(), 1);
    assert_sufficiently_different(b.finish(), d.finish(), 1);
    assert_sufficiently_different(c.finish(), d.finish(), 1);
}

fn test_input_affect_every_byte<T: Hasher>(constructor: impl Fn(u128, u128) -> T) {
    let base = hash_with(&0, constructor(0, 0));
    for shift in 0..16 {
        let mut alternitives = vec![];
        for v in 0..256 {
            let input = (v as u128) << (shift * 8);
            let hasher = constructor(0, 0);
            alternitives.push(hash_with(&input, hasher));
        }
        assert_each_byte_differs(shift, base, alternitives);
    }
}

///Ensures that for every bit in the output there is some value for each byte in the key that flips it.
fn test_keys_affect_every_byte<H: Hash, T: Hasher>(item: H, constructor: impl Fn(u128, u128) -> T) {
    let base = hash_with(&item, constructor(0, 0));
    for shift in 0..16 {
        let mut alternitives1 = vec![];
        let mut alternitives2 = vec![];
        for v in 0..256 {
            let input = (v as u128) << (shift * 8);
            let hasher1 = constructor(input, 0);
            let hasher2 = constructor(0, input);
            let h1 = hash_with(&item, hasher1);
            let h2 = hash_with(&item, hasher2);
            alternitives1.push(h1);
            alternitives2.push(h2);
        }
        assert_each_byte_differs(shift, base, alternitives1);
        assert_each_byte_differs(shift, base, alternitives2);
    }
}

fn assert_each_byte_differs(num: u64, base: u64, alternitives: Vec<u64>) {
    let mut changed_bits = 0_u64;
    for alternitive in alternitives {
        changed_bits |= base ^ alternitive
    }
    assert_eq!(
        core::u64::MAX,
        changed_bits,
        "Bits changed: {:x} on num: {:?}. base {:x}",
        changed_bits,
        num,
        base
    );
}

fn test_finish_is_consistent<T: Hasher>(constructor: impl Fn(u128, u128) -> T) {
    let mut hasher = constructor(1, 2);
    "Foo".hash(&mut hasher);
    let a = hasher.finish();
    let b = hasher.finish();
    assert_eq!(a, b);
}

fn test_single_key_bit_flip<T: Hasher>(constructor: impl Fn(u128, u128) -> T) {
    for bit in 0..128 {
        let mut a = constructor(0, 0);
        let mut b = constructor(0, 1 << bit);
        let mut c = constructor(1 << bit, 0);
        "1234".hash(&mut a);
        "1234".hash(&mut b);
        "1234".hash(&mut c);
        assert_sufficiently_different(a.finish(), b.finish(), 2);
        assert_sufficiently_different(a.finish(), c.finish(), 2);
        assert_sufficiently_different(b.finish(), c.finish(), 2);
        let mut a = constructor(0, 0);
        let mut b = constructor(0, 1 << bit);
        let mut c = constructor(1 << bit, 0);
        "12345678".hash(&mut a);
        "12345678".hash(&mut b);
        "12345678".hash(&mut c);
        assert_sufficiently_different(a.finish(), b.finish(), 2);
        assert_sufficiently_different(a.finish(), c.finish(), 2);
        assert_sufficiently_different(b.finish(), c.finish(), 2);
        let mut a = constructor(0, 0);
        let mut b = constructor(0, 1 << bit);
        let mut c = constructor(1 << bit, 0);
        "1234567812345678".hash(&mut a);
        "1234567812345678".hash(&mut b);
        "1234567812345678".hash(&mut c);
        assert_sufficiently_different(a.finish(), b.finish(), 2);
        assert_sufficiently_different(a.finish(), c.finish(), 2);
        assert_sufficiently_different(b.finish(), c.finish(), 2);
    }
}

fn test_all_bytes_matter<T: Hasher>(hasher: impl Fn() -> T) {
    let mut item = vec![0; 256];
    let base_hash = hash(&item, &hasher);
    for pos in 0..256 {
        item[pos] = 255;
        let hash = hash(&item, &hasher);
        assert_ne!(base_hash, hash, "Position {} did not affect output", pos);
        item[pos] = 0;
    }
}

fn test_no_pair_collisions<T: Hasher>(hasher: impl Fn() -> T) {
    let base = [0_u64, 0_u64];
    let base_hash = hash(&base, &hasher);
    for bitpos1 in 0..64 {
        let a = 1_u64 << bitpos1;
        for bitpos2 in 0..bitpos1 {
            let b = 1_u64 << bitpos2;
            let aa = hash(&[a, a], &hasher);
            let ab = hash(&[a, b], &hasher);
            let ba = hash(&[b, a], &hasher);
            let bb = hash(&[b, b], &hasher);
            assert_sufficiently_different(base_hash, aa, 3);
            assert_sufficiently_different(base_hash, ab, 3);
            assert_sufficiently_different(base_hash, ba, 3);
            assert_sufficiently_different(base_hash, bb, 3);
            assert_sufficiently_different(aa, ab, 3);
            assert_sufficiently_different(ab, ba, 3);
            assert_sufficiently_different(ba, bb, 3);
            assert_sufficiently_different(aa, ba, 3);
            assert_sufficiently_different(ab, bb, 3);
            assert_sufficiently_differ