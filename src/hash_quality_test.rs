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
    assert_sufficiently_different(c.fi