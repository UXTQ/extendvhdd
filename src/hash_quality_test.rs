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