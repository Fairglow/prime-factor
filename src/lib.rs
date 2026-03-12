//! Module for factorizing integers
#![deny(unsafe_code)]
pub mod candidates;

use std::cmp::{min, Ordering};
use std::fmt;
use candidates::PrimeWheel210 as PrimeWheel;
use candidates::{is_prime_candidate, miller_rabin};

/// The threshold where Miller-Rabin primality checking becomes faster than
/// naive trial division. Below this limit, testing wheel candidates up to
/// the square root takes fewer CPU cycles than MR's modulus exponentiations.
const MR_TRIAL_DIVISION_CROSSOVER: u128 = 10_000_000;

/// The threshold below which our specific 12-base Miller-Rabin test is
/// mathematically proven to be 100% accurate (no false positives).
/// Above this limit, MR operates probabilistically and needs fallback verification.
const MR_DETERMINISTIC_LIMIT: u128 = 3_317_044_064_679_887_385_961_981;

/// A prime factor with its exponent (e.g., 2^3 means integer=2, exponent=3).
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IntFactor {
    pub integer: u128,
    pub exponent: u32,
}

impl IntFactor {
    #[must_use]
    pub fn to_vec(&self) -> Vec<u128> {
        vec![self.integer; self.exponent as usize]
    }
}

impl fmt::Display for IntFactor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.exponent > 1 {
            write!(f, "{}^{}", self.integer, self.exponent)
        } else {
            write!(f, "{}", self.integer)
        }
    }
}

/// The prime factorization of an integer, represented as a list of
/// prime factors with their exponents.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PrimeFactors {
    factors: Vec<IntFactor>
}

impl PrimeFactors {
    fn new() -> Self {
        PrimeFactors { factors: Vec::with_capacity(8) }
    }
    fn add(&mut self, integer: u128, exponent: u32) {
        self.factors.push(IntFactor { integer, exponent })
    }
    /// Reconstruct the original integer from its prime factorization.
    /// An empty factorization yields 1 (the empty product).
    #[must_use]
    pub fn value(&self) -> u128 {
        self.factors.iter().map(|f| f.integer.pow(f.exponent)).product()
    }
    /// Return the number of distinct prime factors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.factors.len()
    }
    /// Return the total number of prime factors (counting multiplicities).
    #[must_use]
    pub fn count_factors(&self) -> u32 {
        self.factors.iter().map(|f| f.exponent).sum()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.factors.is_empty()
    }
    #[must_use]
    pub fn is_prime(&self) -> bool {
        self.count_factors() == 1
    }
    /// Return a slice of the prime factors with exponents.
    #[must_use]
    pub fn factors(&self) -> &[IntFactor] {
        &self.factors
    }
    /// Expand the factorization into a flat vector of prime factors.
    #[must_use]
    pub fn to_vec(&self) -> Vec<u128> {
        self.factors.iter().flat_map(IntFactor::to_vec).collect()
    }
    /// Compute the GCD of two prime factorizations by intersecting common factors.
    /// Returns an empty result if either factorization is empty.
    #[must_use]
    pub fn gcd(&self, other: &PrimeFactors) -> PrimeFactors {
        let mut pf = PrimeFactors::new();
        if self.is_empty() || other.is_empty() { return pf; }
        let mut s_it = self.factors.iter();
        let mut o_it = other.factors.iter();
        let mut s = s_it.next().unwrap();
        let mut o = o_it.next().unwrap();
        loop {
            match s.integer.cmp(&o.integer) {
                Ordering::Equal => {
                    pf.add(s.integer, min(o.exponent, s.exponent));
                    s = match s_it.next() { Some(n) => n, None => break };
                    o = match o_it.next() { Some(n) => n, None => break };
                }
                Ordering::Less => {
                    s = match s_it.next() { Some(n) => n, None => break };
                }
                Ordering::Greater => {
                    o = match o_it.next() { Some(n) => n, None => break };
                }
            }
        }
        pf
    }
    /// Check if n has any non-trivial factor using wheel factorization.
    /// Returns true as soon as any factor is found, without full decomposition.
    #[must_use]
    pub fn has_any_factor(n: u128) -> bool {
        if n < 4 { return false; }
        let pw_iter = PrimeWheel::new();
        for f in pw_iter {
            if f * f > n {
                return false;
            }
            if n.is_multiple_of(f) {
                return true;
            }
        }
        false
    }
    /// Compute the prime factorization of n using wheel factorization.
    #[must_use]
    pub fn factorize(n: u128) -> Self {
        let mut pf = PrimeFactors::new();
        if n < 2 { return pf; }
        // --- 1. EARLY EXIT FOR PRIMES ---
        // If the number itself is prime, we don't need to do any trial division.
        // This drops the absolute worst-case scenario from hours down to nanoseconds!
        // We only use MR if n > 10 million where it beats standard division latency.
        if n > MR_TRIAL_DIVISION_CROSSOVER && u128_is_prime(n) {
            pf.add(n, 1);
            return pf;
        }
        let mut maxsq = n;
        let mut x = n;
        let pw_iter = PrimeWheel::new();
        for f in pw_iter {
            if f * f > maxsq {
                break;
            }
            let mut c = 0;
            while x.is_multiple_of(f) {
                x /= f;
                c += 1;
            }
            if c > 0 {
                maxsq = x;
                pf.add(f, c);
                // --- 2. EARLY EXIT FOR INTERMEDIATE PRIMES ---
                // If we stripped out some factors and the remaining chunk 'x'
                // is proven prime by Miller-Rabin, we are fully done.
                if x > MR_TRIAL_DIVISION_CROSSOVER && u128_is_prime(x) {
                    pf.add(x, 1);
                    x = 1;
                    break;
                }
            }
            if x == 1 {
                break;
            }
        }
        if x > 1 {
            pf.add(x, 1);
        }
        pf
    }
}

impl fmt::Display for PrimeFactors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parts: Vec<_> = self.factors.iter()
            .map(|f| f.to_string())
            .collect();
        write!(f, "{}", parts.join(" * "))
    }
}

/// Iterate over the prime factors with their exponents.
impl<'a> IntoIterator for &'a PrimeFactors {
    type Item = &'a IntFactor;
    type IntoIter = std::slice::Iter<'a, IntFactor>;

    fn into_iter(self) -> Self::IntoIter {
        self.factors.iter()
    }
}

/// Consume and iterate over the prime factors with their exponents.
impl IntoIterator for PrimeFactors {
    type Item = IntFactor;
    type IntoIter = std::vec::IntoIter<IntFactor>;

    fn into_iter(self) -> Self::IntoIter {
        self.factors.into_iter()
    }
}

/// An iterator that yields prime numbers in ascending order.
/// Uses wheel factorization to generate candidates, filtering
/// with Miller-Rabin (when available) for fast primality testing.
#[derive(Clone, Debug)]
pub struct PrimeNumbers {
    wheel: PrimeWheel,
}

impl PrimeNumbers {
    #[must_use]
    pub fn new() -> Self {
        Self { wheel: PrimeWheel::new() }
    }
    /// Create an iterator that yields primes >= `start`.
    #[must_use]
    pub fn from(start: u128) -> Self {
        Self { wheel: PrimeWheel::from(start) }
    }
}

impl Default for PrimeNumbers {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for PrimeNumbers {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        self.wheel.by_ref().find(|&n| u128_is_prime(n))
    }
}

/// Test if the value is a prime number.
///
/// Uses deterministic Miller-Rabin for numbers below `MR_DETERMINISTIC_LIMIT`
/// (proven correct). For larger values, Miller-Rabin is used as a fast composite
/// filter (it has no false negatives), and any candidate that passes is verified
/// via trial-division factorization — guaranteeing correctness for all u128.
///
/// Note: for very large primes (above the Miller-Rabin threshold), the
/// factorization fallback may be slow.
#[must_use]
pub fn u128_is_prime(n: u128) -> bool {
    if !is_prime_candidate(n) { return false; }

    // Trial division by subsequent small primes. Even though the wheel
    // filters out multiples of 2, 3, 5, and 7, remaining composites are
    // heavily stripped out by small integer division before hitting the 
    // much slower Miller-Rabin steps.
    const SMALL_PRIMES: &[u128] = &[
        11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97
    ];
    for &p in SMALL_PRIMES {
        if n == p { return true; }
        if n % p == 0 { return false; }
    }

    if n < MR_DETERMINISTIC_LIMIT {
        return miller_rabin(n);
    }
    // MR has no false negatives: if it says composite, it is composite.
    if !miller_rabin(n) { return false; }
    // Verify with guaranteed-correct trial division: if any factor
    // exists, n is composite. Stops at the first factor found.
    !PrimeFactors::has_any_factor(n)
}

/// Return the smallest prime >= n.
///
/// Uses the [`PrimeNumbers`] iterator starting from `n`.
#[must_use]
pub fn next_prime(n: u128) -> u128 {
    PrimeNumbers::from(n).next().unwrap()
}

/// Calculate the Greatest common divisor (GCD) between 2 unsigned integers,
/// returned as a prime factorization.
///
/// Handles the identity gcd(0, n) = n by returning the other value's
/// factorization. Since gcd(0, 0) = 0 has no prime factorization, it
/// returns an empty result. For a simpler numeric GCD, use [`u128_gcd`].
#[must_use]
pub fn primefactor_gcd(this: u128, that: u128) -> PrimeFactors {
    if this == 0 {
        return PrimeFactors::factorize(that);
    }
    if that == 0 {
        return PrimeFactors::factorize(this);
    }
    let pf_this = PrimeFactors::factorize(this);
    let pf_that = PrimeFactors::factorize(that);
    pf_this.gcd(&pf_that)
}

/// Calculate the Greatest common divisor (GCD) between 2 unsigned integers.
/// Based on Euclid's algorithm pseudo code at:
/// <https://en.wikipedia.org/wiki/Euclidean_algorithm>
#[must_use]
pub fn u128_gcd(this: u128, that: u128) -> u128 {
    let mut a = this;
    let mut b = that;
    while b > 0 {
        let c = b;
        b = a % b;
        a = c;
    }
    a
}

/// Calculate the Least common multiple (LCM) for 2 integers.
#[must_use]
pub fn u128_lcm(this: u128, that: u128) -> u128 {
    checked_u128_lcm(this, that).expect("u128_lcm overflow")
}

#[must_use]
pub fn checked_u128_lcm(this: u128, that: u128) -> Option<u128> {
    if this == 0 || that == 0 {
        return Some(0);
    }
    let gcd = u128_gcd(this, that);
    (this / gcd).checked_mul(that)
}
