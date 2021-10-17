//! Module for factorizing integers
use std::cmp::{min, Ordering};
use std::convert::From;
use std::fmt;
use genawaiter::yield_;
use genawaiter::stack::{let_gen_using, producer_fn};

/// Wheel factorization algorithm with base {2, 3, 5}
/// https://en.wikipedia.org/wiki/Wheel_factorization
#[producer_fn(u128)]
async fn prime_wheel_30() {
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IntFactor {
    pub integer: u128,
    pub exponent: u32,
}

impl IntFactor {
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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PrimeFactors {
    factors: Vec<IntFactor>
}

impl PrimeFactors {
    fn new() -> Self {
        PrimeFactors { factors: Vec::new() }
    }
    fn add(&mut self, integer: u128, exponent: u32) {
        self.factors.push(IntFactor { integer, exponent })
    }
    pub fn value(&self) -> u128 {
        self.factors.iter().map(|f| f.integer.pow(f.exponent)).product()
    }
    pub fn len(&self) -> usize {
        self.factors.len()
    }
    pub fn count_factors(&self) -> u32 {
        self.factors.iter().map(|f| f.exponent).sum()
    }
    pub fn is_empty(&self) -> bool {
        self.factors.is_empty()
    }
    pub fn is_prime(&self) -> bool {
        self.count_factors() == 1
    }
    pub fn iter(&self) -> PrimeFactorsIter {
        PrimeFactorsIter { vec: &self.factors, ndx: 0 }
    }
    pub fn to_factor_vec(&self) -> &Vec<IntFactor> {
        &self.factors
    }
    pub fn to_vec(&self) -> Vec<u128> {
        let mut ret = Vec::new();
        self.factors.iter().for_each(|f| ret.extend(f.to_vec()));
        ret
    }
    pub fn gcd(&self, other: &PrimeFactors) -> PrimeFactors {
        let mut pf = PrimeFactors::new();
        if self.is_empty() || other.is_empty() { return pf; }
        let mut s_it = self.factors.iter();
        let mut o_it = other.factors.iter();
        let mut s = s_it.next().unwrap();
        let mut o = o_it.next().unwrap();
        loop {
            let prime_cmp = s.integer.cmp(&o.integer);
            if prime_cmp == Ordering::Equal {
                pf.add(s.integer, min(o.exponent, s.exponent));
            }
            match prime_cmp {
                Ordering::Less | Ordering::Equal => {
                    if let Some(n) = s_it.next() { s = n; } else { break; }
                },
                Ordering::Greater => {
                    if let Some(n) = o_it.next() { o = n; } else { break; }
                },
            }
        }
        pf
    }
}

impl From<u128> for PrimeFactors {
    fn from(n: u128) -> Self {
        let mut pf = PrimeFactors::new();
        if n < 2 { return pf; }
        // A factor of n must have a value less than or equal to sqrt(n)
        let mut maxf = u128_sqrt(n) + 1;
        let_gen_using!(mpgen, prime_wheel_30);
        let mut x = n;
        for f in mpgen.into_iter() {
            if f >= maxf {
                break;
            }
            let mut c = 0;
            while x % f == 0 {
                x /= f;
                c += 1;
            }
            if c > 0 {
                // A factor of x must have a value less than or equal to sqrt(x)
                maxf = u128_sqrt(x) + 1;
                pf.add(f, c);
            }
            if x == 1 {
                break;
            }
        }
        if x > 1 || pf.is_empty() {
            // Any remainder must be the number itself or a factor of it.
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

/// IntFactor interator
pub struct PrimeFactorsIter<'a> {
    vec: &'a Vec<IntFactor>,
    ndx: usize,
}

impl<'a> Iterator for PrimeFactorsIter<'a> {
    type Item = IntFactor;

    fn next(&mut self) -> Option<IntFactor> {
        if self.ndx >= self.vec.len() { return None; }
        let pf = self.vec[self.ndx];
        self.ndx += 1;
        Some(pf)
    }
}

impl<'a> IntoIterator for &'a PrimeFactors {
    type Item = IntFactor;
    type IntoIter = PrimeFactorsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PrimeFactorsIter {
            vec: &self.factors,
            ndx: 0,
        }
    }
}

/// Unsigned 128-bit integer square root calculation.
/// Based on example implementation in C at:
/// https://en.wikipedia.org/wiki/Integer_square_root
pub fn u128_sqrt(s: u128) -> u128 {
    let mut g = s >> 1; // Initial guess
    if g == 0 { return s; } // sanity check
    let mut u = (g + s / g) >> 1; // update
    while u < g { // this also checks for cycle
        g = u;
        u = (g + s / g) >> 1;
    }
    g
}

/// Test if the value is a prime number, or not
pub fn u128_is_prime(n: u128) -> bool {
    if n < 2 { return false; }
    if n > 30 {
        // check spoke in the prime wheel (base 30)
        match n % 30 {
            1|7|11|13|17|19|23|29 => (), // may be prime
            _ => return false, // cannot be prime
        }
    }
    // A factor of n must have a value less than or equal to sqrt(n)
    let maxf = u128_sqrt(n) + 1;
    let_gen_using!(mpgen, prime_wheel_30);
    for f in mpgen.into_iter() {
        if f >= maxf {
            break;
        }
        if n % f == 0 {
            return false;
        }
    }
    true
}

/// Calculate the Greatest common divisor (GCD) between 2 unsigned integers
pub fn primefactor_gcd(this: u128, that: u128) -> PrimeFactors {
    let pf_this = PrimeFactors::from(this);
    let pf_that = PrimeFactors::from(that);
    pf_this.gcd(&pf_that)
}

/// Calculate the Greatest common divisor (GCD) between 2 unsigned integers.
/// Based on Euclid's algorithm pseudo code at:
/// https://en.wikipedia.org/wiki/Euclidean_algorithm
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

/// Calculate the Least common multiple (LCM) for 2 integers
pub fn u128_lcm(this: u128, that: u128) -> u128 {
    if this == 0 && that == 0 { return 0; }
    let gcd = u128_gcd(this, that);
    this * that / gcd
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::*;
    use rand::Rng;
    use genawaiter::stack::let_gen_using;
    
    #[test]
    fn test_early_prime_wheel_numbers() {
        let testvec = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59,
            61, 67, 71, 73, 77, 79, 83, 89, 91, 97, 101, 103, 107, 109, 113
        ];
        let_gen_using!(mpgen, prime_wheel_30);
        let mut mp = mpgen.into_iter();
        for i in 0..testvec.len() {
            let p = mp.next().unwrap();
            assert_eq!(testvec[i], p);
        }
    }

    #[test]
    fn test_prime_wheel_quality() {
        let mut primes: u128 = 0;
        let mut others: u128 = 0;
        let_gen_using!(mpgen, prime_wheel_30);
        let mut mp = mpgen.into_iter();
        for _ in 0..1000000 {
            let p = mp.next().unwrap();
            if u128_is_prime(p) {
                primes += 1;
            } else {
                others += 1;
            }
        }
        let percent = primes as f64 / (primes + others) as f64 * 100.0;
        println!("Prime wheel generated {}/{} ({:.3}%) primes",
                 primes, primes+others, percent);
        assert!(percent > 25.0);
    }

    #[test]
    fn test_int_sqrt_pow_of_2() {
        let mut rnd = rand::thread_rng();
        for _ in 1..1000 {
            let n = rnd.gen_range(1..u128_sqrt(u128::MAX));
            let sqrt = u128_sqrt(n.pow(2));
            assert_eq!(sqrt, n);
        }
    }

    #[test]
    fn test_int_sqrt_floor() {
        let mut rnd = rand::thread_rng();
        for _ in 1..1000 {
            // Largest integer in a f64 is 2^53-1 (52 bits mantissa)
            let n = rnd.gen_range(1..u64::pow(2, 53) as u128);
            let expt = f64::sqrt(n as f64) as u128;
            let sqrt = u128_sqrt(n);
            assert_eq!(sqrt, expt);
        }
    }
}
