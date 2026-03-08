//! There are many functions that can produce prime number candidates, but only
//! a few that are guaranteed to produce all primes.
//!
//! Implementations of Prime wheels for number factorization
//! https://en.wikipedia.org/wiki/Wheel_factorization
//!
//! We can omit overflow bounds checks for the wheel iterators, since the
//! callers stop consuming them well before values approach the u128 limit.
//! In `factorize`, the iterator is only consumed up to sqrt(n), which for
//! the maximum u128 value is approximately 1.84e19.
//!
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

/// Fast prime candidate filter using the 210-spoke wheel bitmap.
/// Returns false for any number divisible by 2, 3, 5, or 7,
/// eliminating ~77% of all composites with a single modulo + bit-test.
#[inline(always)]
pub(crate) fn is_prime_candidate(n: u128) -> bool {
    if n < 11 {
        return matches!(n, 2 | 3 | 5 | 7);
    }
    const BITMAP: [u32; 7] = [
        0xa08a_2802, 0x2820_8a20, 0x0208_8288, 0x8202_28a2,
        0x20a0_8a08, 0x8828_2288, 0x0002_00a2,
    ];
    let index = (n % 210) as usize;
    BITMAP[index / 32] & (1 << (index & 0x1F)) != 0
}

/// Modular exponentiation: (base^exp) mod modulus.
#[inline]
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 { return 0; }
    let mut result: u128 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, modulus);
        }
        exp >>= 1;
        if exp > 0 {
            base = mod_mul(base, base, modulus);
        }
    }
    result
}

/// Modular addition: (a + b) mod m, without overflow.
/// Requires a < m and b < m.
#[inline]
fn add_mod(a: u128, b: u128, m: u128) -> u128 {
    debug_assert!(a < m);
    debug_assert!(b < m);
    if a >= m - b {
        a - (m - b)
    } else {
        a + b
    }
}

/// Modular multiplication: (a * b) mod m, without overflow.
/// Uses direct multiplication when the product fits in u128.
/// For larger products, it uses Russian peasant multiplication.
/// When m <= 2^127, it uses a faster doubling path; otherwise it switches
/// to an overflow-safe modular addition path.
#[inline]
fn mod_mul(a: u128, b: u128, m: u128) -> u128 {
    debug_assert!(m > 0);
    // For small moduli where a*b won't overflow u128, use direct multiplication
    if a.leading_zeros() + b.leading_zeros() >= 128 {
        return (a * b) % m;
    }
    let mut result: u128 = 0;
    let mut a = a % m;
    let mut b = b % m;
    // Full-range safe fallback for very large moduli.
    while b > 0 {
        if b & 1 == 1 {
            result = add_mod(result, a, m);
        }
        b >>= 1;
        if b > 0 {
            a = add_mod(a, a, m);
        }
    }
    result
}

/// Test a single Miller-Rabin witness against n.
/// Returns true if n passes the test for this witness (probably prime).
fn miller_rabin_witness(n: u128, a: u128, d: u128, r: u32) -> bool {
    debug_assert!(n >= 2);
    let mut x = mod_pow(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 1..r {
        x = mod_mul(x, x, n);
        if x == n - 1 {
            return true;
        }
    }
    false
}

/// Deterministic Miller-Rabin primality test (for n >= 2).
///
/// Uses witnesses {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37} which are
/// proven sufficient for all numbers below 3,317,044,064,679,887,385,961,981.
///
/// Reference: <https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test>
pub(crate) fn miller_rabin(n: u128) -> bool {
    const WITNESSES: [u128; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    debug_assert!(n >= 2);
    let n_minus_1 = n - 1;
    let r = n_minus_1.trailing_zeros();
    let d = n_minus_1 >> r;
    WITNESSES.iter().all(|&a| a >= n || miller_rabin_witness(n, a, d, r))
}

#[cfg(test)]
mod tests {
    use reikna::prime::{is_prime, next_prime};
    use super::{PrimeWheel30, PrimeWheel210, add_mod, mod_mul};

    fn mod_mul_reference(a: u128, b: u128, m: u128) -> u128 {
        let mut result = 0;
        let mut a = a % m;
        let mut b = b % m;
        while b > 0 {
            if b & 1 == 1 {
                result = add_mod(result, a, m);
            }
            b >>= 1;
            if b > 0 {
                a = add_mod(a, a, m);
            }
        }

        result
    }

    #[test]
    fn test_prime_wheel_30_first_1000() {
        let mut wheel = PrimeWheel30::new();
        let mut misses = 0;
        let mut p = 0;
        for _ in 0..1000 {
            p = next_prime(p);
            for n in wheel.by_ref() {
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
            for n in wheel.by_ref() {
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

    #[test]
    fn test_add_mod_large_values() {
        let m = u128::MAX - 158;
        let a = m - 1;
        let b = m - 1;
        assert_eq!(add_mod(a, b, m), m - 2);
    }

    #[test]
    fn test_mod_mul_matches_direct_below_2pow127_boundary() {
        let m = (1_u128 << 127) - 1;
        let values = [
            0,
            1,
            2,
            3,
            17,
            (1_u128 << 64) + 13,
            m - 2,
            m - 1,
        ];

        for &a in &values {
            for &b in &values {
                assert_eq!(mod_mul(a, b, m), mod_mul_reference(a, b, m));
            }
        }
    }

    #[test]
    fn test_mod_mul_large_modulus_regression_cases() {
        let m = u128::MAX - 158;
        let cases = [
            (m - 1, m - 1, 1),
            (m - 1, 2, m - 2),
            (m - 2, 2, m - 4),
            (m - 1, m - 2, 2),
        ];

        for (a, b, expected) in cases {
            assert_eq!(mod_mul(a, b, m), expected);
            assert_eq!(mod_mul(b, a, m), expected);
        }
    }
}
