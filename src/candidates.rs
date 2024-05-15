//! Implementations of Prime wheels for number factorization
//! https://en.wikipedia.org/wiki/Wheel_factorization
#![deny(unsafe_code)]
#![allow(dead_code)]
use genawaiter::yield_;
use genawaiter::stack::producer_fn;

/// Wheel factorization algorithm with base {2, 3, 5} (30 spokes)
#[producer_fn(u128)]
pub async fn prime_wheel_30() {
    yield_!(2);
    yield_!(3);
    yield_!(5);
    let mut base = 0u128;
    loop {
        for n in [7, 11, 13, 17, 19, 23, 29, 31] {
            yield_!(base + n);
        }
        base += 30;
    }
}

pub const SPOKES: [u128; 48] = [
    11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
    79, 83, 89, 97, 101, 103, 107, 109, 113, 121, 127, 131, 137, 139, 143,
    149, 151, 157, 163, 167, 169, 173, 179, 181, 187, 191, 193, 197, 199,
    209, 211];

/// Wheel factorization algorithm with base {2, 3, 5, 7} (210 spokes)
#[producer_fn(u128)]
pub async fn prime_wheel_210() {
    yield_!(2);
    yield_!(3);
    yield_!(5);
    yield_!(7);
    // Valid spokes after removing multiples of 2, 3, 5, 7
    let mut base = 0u128;
    loop {
        for n in SPOKES {
            yield_!(base + n);
        }
        base += 210;
    }
}

// Bit-map: 0x0200a28828228820a08a08820228a20208828828208a20a08a2802
const PW210_BITMAP: [u8; 27] = [
    0x02, 0x28, 0x8a, 0xa0, 0x20, 0x8a, 0x20, 0x28,
    0x88, 0x82, 0x08, 0x02, 0xa2, 0x28, 0x02, 0x82,
    0x08, 0x8a, 0xa0, 0x20, 0x88, 0x22, 0x28, 0x88,
    0xa2, 0x00, 0x02];

pub fn is_pw210_candidate(num: u128) -> bool {
    if num < 11 {
        match num {
            2 | 3 | 5 | 7 => true,
            _ => false,
        }
    } else {
        let index = (num % 210) as usize; // Calculate bit position (0 to 209)
        let byte_index = index / 8; // Calculate byte index within the array
        let bit_mask = 1 << (index % 8); // Calculate bit-mask within the byte
        PW210_BITMAP[byte_index] & bit_mask > 0
    }
}
