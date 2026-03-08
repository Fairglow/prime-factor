//! Module for factorizing integers
#![deny(unsafe_code)]
pub mod candidates;

use std::cmp::{min, Ordering};
use std::fmt;
use candidates::PrimeWheel210 as PrimeWheel;
use candidates::{is_prime_candidate, miller_rabin};

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
    /// Compute the prime factorization of n using wheel factorization.
    #[must_use]
    pub fn factorize(n: u128) -> Self {
        let mut pf = PrimeFactors::new();
        if n < 2 { return pf; }
        // The smallest prime factor of n must be <= sqrt(n)
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
                // The smallest prime factor of x must be <= sqrt(x)
                maxsq = x;
                pf.add(f, c);
            }
            if x == 1 {
                break;
            }
        }
        if x > 1 || pf.is_empty() {
            // Any remainder x > 1 must itself be prime.
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

/// Test if the value is a prime number.
///
/// Uses deterministic Miller-Rabin for numbers below 3,317,044,064,679,887,385,961,981
/// (proven correct). For larger values, Miller-Rabin is used as a fast composite
/// filter (it has no false negatives), and any candidate that passes is verified
/// via trial-division factorization — guaranteeing correctness for all u128.
///
/// Note: for very large primes (above the Miller-Rabin threshold), the
/// factorization fallback may be slow.
#[must_use]
pub fn u128_is_prime(n: u128) -> bool {
    if !is_prime_candidate(n) { return false; }
    if n < 3_317_044_064_679_887_385_961_981 {
        return miller_rabin(n);
    }
    // MR has no false negatives: if it says composite, it is composite.
    if !miller_rabin(n) { return false; }
    // Verify with guaranteed-correct trial-division factorization.
    PrimeFactors::factorize(n).is_prime()
}

/// Calculate the Greatest common divisor (GCD) between 2 unsigned integers.
#[must_use]
pub fn primefactor_gcd(this: u128, that: u128) -> PrimeFactors {
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
