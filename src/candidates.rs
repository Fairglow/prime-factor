//! There are many functions that can produce prime number candidates, but only
//! a few that are guaranteed to produce all primes.
//!
//! Implementations of Prime wheels for number factorization
//! https://en.wikipedia.org/wiki/Wheel_factorization
//!
//! We can omit the overflow bounds checks for the wheel iterators, since they
//! are only used to generate prime candidates and the highest prime candidate
//! is much smaller than the u128 limit. The wheel iterators will stop before
//! the square root of the maximum u128 value, which is approximately 1.15e19.
//!
#![allow(dead_code)]

/// Wheel factorization algorithm with base {2, 3, 5} (30 spokes)
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PrimeWheel30 {
    base: u128,
    index: usize,
}

impl PrimeWheel30 {
    const GAPS: [u128; 12] = [
        2, // +2 = 2
        1, // +1 = 3
        2, // +2 = 5
        2, // +2 = 7 (index 3, end of initial phase)
        4, // +4 = 11 + n * 30 (index 4, start of cycle)
        2, // +2 = 13 + n * 30
        4, // +4 = 17 + n * 30
        2, // +2 = 19 + n * 30
        4, // +4 = 23 + n * 30
        6, // +6 = 29 + n * 30
        2, // +2 = 31 + n * 30
        6  // +6 = 37 + n * 30 (index 11, end of cycle)
    ];
    pub fn new() -> Self {
        Self::default()
    }
}

impl Iterator for PrimeWheel30 {
    type Item = u128;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let gap = Self::GAPS.get(self.index)?;
        self.base += gap; 
        self.index += 1;
        if self.index == 12 {
            self.index = 4;
        }
        Some(self.base)
    }
}

/// Wheel factorization algorithm with base {2, 3, 5, 7} (210 spokes)
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PrimeWheel210 {
    base: u128,
    index: usize,
}

impl PrimeWheel210 {
    const GAPS: [u128; 53] = [
        2, 1, 2, 2, 4, // initial phase: 2, 3, 5, 7, 11 (index 0-4)
        2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, // 13..71 (index 5, start of cycle)
        2, 6, 4, 6, 8, 4, 2, 4, 2, 4, 8, 6, 4, 6, 2, 4, // 73..143
        6, 2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2, // 149..211
        10 // 221 + n * 210 (index 52, end of cycle, wraps to index 5)
    ];
    pub fn new() -> Self {
        Self::default()
    }
}

impl Iterator for PrimeWheel210 {
    type Item = u128;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let gap = Self::GAPS.get(self.index)?;
        self.base += gap;
        self.index += 1;
        if self.index == 53 {
            self.index = 5;
        }
        Some(self.base)
    }
}

#[cfg(test)]
mod tests {
    use reikna::prime::{is_prime, next_prime};
    use super::{PrimeWheel30, PrimeWheel210};

    #[test]
    fn test_prime_wheel_30_first_1000() {
        let mut wheel = PrimeWheel30::new();
        let mut misses = 0;
        let mut p = 0;
        for _ in 0..1000 {
            p = next_prime(p);
            while let Some(n) = wheel.next() {
                if n == p as u128 {
                    break;
                }
                assert!(!is_prime(n as u64));
                misses += 1;
            }
        }
        // Assert the exact number of expected misses for the first 100 primes
        assert_eq!(misses, 1114);
    }

    #[test]
    fn test_prime_wheel_210_first_1000() {
        let mut wheel = PrimeWheel210::new();
        let mut misses = 0;
        let mut p = 0;
        for _ in 0..1000 {
            p = next_prime(p);
            while let Some(n) = wheel.next() {
                if n == p as u128 {
                    break;
                }
                assert!(!is_prime(n as u64));
                misses += 1;
            }
        }
        // Assert the exact number of expected misses for the first 1000 primes
        assert_eq!(misses, 813);
    }

    #[test]
    fn test_prime_wheel_30_quality() {
        const TOTAL: u128 = 1000000;
        let mut primes: u128 = 0;
        let pw_iter = PrimeWheel30::new();
        for p in pw_iter.take(TOTAL as usize) {
            primes += is_prime(p as u64) as u128;
        }
        let percent = primes as f64 / TOTAL as f64 * 100.0;
        println!("Prime wheel generated {}/{} ({:.3}%) primes",
                primes, TOTAL, percent);
        assert!(percent > 25.0);
    }

    #[test]
    fn test_prime_wheel_210_quality() {
        const TOTAL: u128 = 1000000;
        let mut primes: u128 = 0;
        let pw_iter = PrimeWheel210::new();
        for p in pw_iter.take(TOTAL as usize) {
            primes += is_prime(p as u64) as u128;
        }
        let percent = primes as f64 / TOTAL as f64 * 100.0;
        println!("Prime wheel generated {}/{} ({:.3}%) primes",
                primes, TOTAL, percent);
        assert!(percent > 30.0);
    }
}
