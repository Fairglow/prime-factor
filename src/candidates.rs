//! Implementations of Prime wheels for number factorization
//! https://en.wikipedia.org/wiki/Wheel_factorization
#![allow(dead_code)]

/// Wheel factorization algorithm with base {2, 3, 5} (30 spokes)
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PrimeWheel30 {
    base: u128,
    first: usize,
    index: usize,
}

impl PrimeWheel30 {
    const FIRSTS: [u128; 3] = [2, 3, 5];
    const SPOKES: [u128; 8] = [7, 11, 13, 17, 19, 23, 29, 31];
    pub fn new() -> Self {
        Self::default()
    }
}

impl Iterator for PrimeWheel30 {
    type Item = u128;
    fn next(&mut self) -> Option<Self::Item> {
        if self.first < Self::FIRSTS.len() {
            let n = Self::FIRSTS[self.first];
            self.first += 1;
            Some(n)
        } else if self.base == 87841638446235960 && self.index > 2 {
            None
        } else if self.index < Self::SPOKES.len() {
            let n = self.base + Self::SPOKES[self.index];
            self.index += 1;
            Some(n)
        } else {
            self.base += 30;
            self.index = 1;
            Some(self.base + Self::SPOKES[0])
        }
    }
}

/// Wheel factorization algorithm with base {2, 3, 5, 7} (210 spokes)
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PrimeWheel210 {
    base: u128,
    first: usize,
    index: usize,
}

impl PrimeWheel210 {
    const FIRSTS: [u128; 4] = [2, 3, 5, 7];
    const SPOKES: [u128; 48] = [
        11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
        79, 83, 89, 97, 101, 103, 107, 109, 113, 121, 127, 131, 137, 139, 143,
        149, 151, 157, 163, 167, 169, 173, 179, 181, 187, 191, 193, 197, 199,
        209, 211];
    pub fn new() -> Self {
        Self::default()
    }
}

impl Iterator for PrimeWheel210 {
    type Item = u128;
    fn next(&mut self) -> Option<Self::Item> {
        if self.first < Self::FIRSTS.len() {
            let n = Self::FIRSTS[self.first];
            self.first += 1;
            Some(n)
        } else if self.base == 87841638446235960 && self.index > 1 {
            None
        } else if self.index < Self::SPOKES.len() {
            let n = self.base + Self::SPOKES[self.index];
            self.index += 1;
            Some(n)
        } else {
            self.base += 210;
            self.index = 1;
            Some(self.base + Self::SPOKES[0])
        }
    }
}

// Bit-map: 0x0200a2_88282288_20a08a08_820228a2_02088288_28208a20_a08a2802
const PW210_BITMAP_B: [u8; 27] = [
    0x02, 0x28, 0x8a, 0xa0, 0x20, 0x8a, 0x20, 0x28,
    0x88, 0x82, 0x08, 0x02, 0xa2, 0x28, 0x02, 0x82,
    0x08, 0x8a, 0xa0, 0x20, 0x88, 0x22, 0x28, 0x88,
    0xa2, 0x00, 0x02];

pub fn is_pw210_candidate_b(num: u128) -> bool {
    if num < 11 {
        matches!(num, 2 | 3 | 5 | 7)
    } else {
        let index = (num % 210) as usize; // Calculate bit position (0 to 209)
        let byte_index = index / 8; // Calculate byte index within the array
        let bit_mask = 1 << (index % 8); // Calculate bit-mask within the byte
        PW210_BITMAP_B[byte_index] & bit_mask > 0
    }
}

const PW210_BITMAP_32: [u32; 7] = [
    0xa08a2802,
    0x28208a20,
    0x02088288,
    0x820228a2,
    0x20a08a08,
    0x88282288,
    0x000200a2,
];

pub fn is_pw210_candidate(num: u128) -> bool {
    if num < 11 {
        matches!(num, 2 | 3 | 5 | 7)
    } else {
        let index = (num % 210) as usize;  // Calculate bit position (0 to 209)
        let dword_index = index / 32;      // Calculate dword index
        let bit_mask = 1 << (index & 0x1F); // Calculate bit-mask
        PW210_BITMAP_32[dword_index] & bit_mask > 0
    }
}

#[cfg(test)]
mod tests {
    use super::{PrimeWheel210, is_pw210_candidate, is_pw210_candidate_b};
    #[test]
    fn test_spokes() {
        (8..212).into_iter().for_each(|n| {
            assert_eq!(PrimeWheel210::SPOKES.contains(&n), is_pw210_candidate(n));
        });
    }
    #[test]
    fn test_bitmaps() {
        for n in 0..210 {
            assert_eq!(is_pw210_candidate(n), is_pw210_candidate_b(n));
        }
    }
}
