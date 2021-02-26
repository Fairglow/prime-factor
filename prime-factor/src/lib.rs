use std::cmp::{min, Ordering};
use std::convert::From;
use std::fmt;
use genawaiter::yield_;
use genawaiter::stack::{let_gen_using, producer_fn};

#[producer_fn(u128)]
async fn maybe_prime_gen() {
    yield_!(2);
    yield_!(3);
    yield_!(5); // must not skip the initial 5
    let mut accum = 7u128;
    loop {
        /* Numbers ending in 5's occur with a periodicity of 10,
         * at positions 6 and 9 when starting from 7, forming the
         * end-digit pattern: 7, 1, 3, 7, 9, 3, 5, 9, 1, 5, ... */
        for i in 0..10 {
            match i {
                6|9 => (), // skip numbers ending in 5
                _ => yield_!(accum),
            }
            /* All primes except 2 and 3 are congruent modulo 6 to one of 1 or 5. */
            accum += 2 * (1 - (i&1)) + 2; // alternate between adding 2 and 4
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PrimeFactor {
    pub prime: u128,
    pub exponent: u32,
}

impl fmt::Display for PrimeFactor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.exponent > 1 {
            write!(f, "{}^{}", self.prime, self.exponent)
        } else {
            write!(f, "{}", self.prime)
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PrimeFactors {
    factors: Vec<PrimeFactor>
}

impl PrimeFactors {
    fn new() -> Self {
        PrimeFactors { factors: Vec::new() }
    }
    fn add(&mut self, prime: u128, exponent: u32) {
        self.factors.push(PrimeFactor { prime, exponent })
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
    pub fn to_vec(&self) -> &Vec<PrimeFactor> {
        &self.factors
    }
    pub fn gcd(&self, other: &PrimeFactors) -> PrimeFactors {
        let cnt = min(self.factors.len(), other.factors.len());
        let mut pf = PrimeFactors::new();
        if cnt == 0 { return pf; }
        let mut s_it = self.factors.iter();
        let mut o_it = other.factors.iter();
        let mut s = s_it.next().unwrap();
        let mut o = o_it.next().unwrap();
        loop {
            let prime_cmp = s.prime.cmp(&o.prime);
            if prime_cmp == Ordering::Equal {
                pf.add(s.prime, min(o.exponent, s.exponent));
            }
            match prime_cmp {
                Ordering::Less | Ordering::Equal => {
                    if let Some(n) = s_it.next() { s = &n; } else { break; }
                },
                Ordering::Greater => {
                    if let Some(n) = o_it.next() { o = &n; } else { break; }
                },
            }
        }
        pf
    }
}

impl From<u128> for PrimeFactors {
    fn from(n: u128) -> Self {
        let mut pf = PrimeFactors::new();
        // A factor of n must have a value less than or equal to sqrt(n)
        let mut maxf = u128_sqrt(n) + 1;
        let_gen_using!(mpgen, maybe_prime_gen);
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

/* Integer square root calculation
 * Based on example implementation in C at:
 * https://en.wikipedia.org/wiki/Integer_square_root
 */
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

pub fn u128_gcd(this: u128, that: u128) -> PrimeFactors {
    let pf_this = PrimeFactors::from(this);
    let pf_that = PrimeFactors::from(that);
    pf_this.gcd(&pf_that)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::*;
    use rand::Rng;
    use genawaiter::stack::let_gen_using;
    
    #[test]
    fn test_maybe_prime_generator() {
        let testvec = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59,
            61, 67, 71, 73, 77, 79, 83, 89, 91, 97, 101, 103, 107, 109, 113
        ];
        let_gen_using!(mpgen, maybe_prime_gen);
        let mut mp = mpgen.into_iter();
        for i in 0..testvec.len() {
            let p = mp.next().unwrap();
            assert_eq!(testvec[i], p);
        }
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
