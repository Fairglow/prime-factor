use rand::RngExt;
use rayon::prelude::*;
use primefactor::{
    primefactor_gcd,
    PrimeFactors,
    PrimeNumbers,
    u128_gcd,
    u128_is_prime,
    u128_lcm};
use reikna::prime::{is_prime, nth_prime};

#[test]
fn test_10k_random_32bit_numbers() {
    let mut rnd = rand::rng();
    for _ in 0..10000 {
        let n = rnd.random_range(0..=u32::MAX);
        assert_eq!(u128_is_prime(n as u128), is_prime(n as u64), "is num {n} prime?");
    }
}

#[test]
fn test_first_5k_primes() {
    for num in 0..=5000 {
        let p = nth_prime(num);
        assert!(u128_is_prime(p as u128), "is num {p} prime?");
    }
}

#[test]
fn test_some_factors() {
    let mut rnd = rand::rng();
    for _ in 0..1000 {
        let num = rnd.random_range(2..=u32::MAX as u128);
        let facts = PrimeFactors::factorize(num);
        assert_eq!(u128_is_prime(num), facts.is_prime());
        if facts.is_prime() {
            let fe = &facts.to_vec();
            assert_eq!(fe.len(), 1);
            assert_eq!(fe[0], num);
        } else {
            assert_eq!(num, facts.value());
        }
    }
}

#[test]
fn test_a_few_gcd() {
    assert_eq!(primefactor_gcd(2*3*5*7, 2*5*11), PrimeFactors::factorize(2*5));
    assert_eq!(primefactor_gcd(3*4*5, 3*4*7), PrimeFactors::factorize(3*4));
    assert_eq!(primefactor_gcd(9*4*11, 3*8*13), PrimeFactors::factorize(3*4));
    assert_eq!(primefactor_gcd(27*64*121, 9*32*49), PrimeFactors::factorize(9*32));
    let no_gcd = primefactor_gcd(3*7*13, 2*5*11);
    assert!(no_gcd.is_empty());
    assert!(primefactor_gcd(1, 1).is_empty());
    assert!(primefactor_gcd(1, 0).is_empty());
    assert!(primefactor_gcd(0, 1).is_empty());
    assert!(primefactor_gcd(0, 0).is_empty());
    assert_eq!(primefactor_gcd(0, 10), PrimeFactors::factorize(10));
    assert_eq!(primefactor_gcd(10, 0), PrimeFactors::factorize(10));
    assert_eq!(u128_gcd(2*3*5*7, 2*5*11), 2*5);
    assert_eq!(u128_gcd(3*4*5, 3*4*7), 3*4);
    assert_eq!(u128_gcd(9*4*11, 3*8*13), 3*4);
    assert_eq!(u128_gcd(27*64*121, 9*32*49), 9*32);
    assert_eq!(u128_gcd(1, 1), 1);
    assert_eq!(u128_gcd(1, 0), 1);
    assert_eq!(u128_gcd(0, 1), 1);
    assert_eq!(u128_gcd(0, 0), 0);
}

#[test]
fn test_compare_some_gcd() {
    (0..100).into_par_iter().for_each(|_| {
        let mut rnd = rand::rng();
        let a = rnd.random_range(2..=u32::MAX as u128);
        let b = rnd.random_range(2..=u32::MAX as u128);
        let pf_gcd = primefactor_gcd(a, b);
        let ea_gcd = u128_gcd(a, b);
        if pf_gcd.is_empty() {
            assert_eq!(ea_gcd, 1);
        } else {
            assert_eq!(ea_gcd, pf_gcd.value());
        }
    })
}

#[test]
fn test_a_few_lcm() {
    assert_eq!(u128_lcm(2*3*5*7, 2*5*11), 2*3*5*7*11);
    assert_eq!(u128_lcm(3*4*5, 3*4*7), 3*4*5*7);
    assert_eq!(u128_lcm(9*4*11, 3*8*13), 8*9*11*13);
    assert_eq!(u128_lcm(27*64*121, 9*32*49), 27*64*49*121);
    assert_eq!(u128_lcm(3*7*13, 2*5*11), 2*3*5*7*11*13);
    assert_eq!(u128_lcm(1, 1), 1);
    assert_eq!(u128_lcm(0, 1), 0);
    assert_eq!(u128_lcm(1, 0), 0);
    assert_eq!(u128_lcm(0, 0), 0);
}

#[test]
fn test_some_gcd_lcm() {
    (0..10).into_par_iter().for_each(|_| {
        let mut rnd = rand::rng();
        let a = rnd.random_range(2..=u32::MAX as u128);
        let b = rnd.random_range(2..=u32::MAX as u128);
        let c = rnd.random_range(2..=u32::MAX as u128);

        // Test using the [Fundamental_theorem_of_arithmetic](https://en.wikipedia.org/wiki/Fundamental_theorem_of_arithmetic)
        assert_eq!(u128_gcd(a, b) * u128_lcm(a, b), a * b);

        // Test idempotent laws
        assert_eq!(u128_gcd(a, a), a);
        assert_eq!(u128_gcd(b, b), b);
        assert_eq!(u128_lcm(a, a), a);
        assert_eq!(u128_lcm(b, b), b);

        // Test commutative laws
        assert_eq!(u128_gcd(a, b), u128_gcd(b, a));
        assert_eq!(u128_lcm(a, b), u128_lcm(b, a));

        // Test absorption laws
        assert_eq!(u128_gcd(a, u128_lcm(a, b)), a);
        assert_eq!(u128_gcd(b, u128_lcm(a, b)), b);
        assert_eq!(u128_lcm(a, u128_gcd(a, b)), a);
        assert_eq!(u128_lcm(b, u128_gcd(a, b)), b);

        // Test associative laws
        assert_eq!(u128_gcd(a, u128_gcd(b, c)), u128_gcd(u128_gcd(a, b), c));
        assert_eq!(u128_lcm(a, u128_lcm(b, c)), u128_lcm(u128_lcm(a, b), c));
    })
}

#[test]
fn test_compare_reikna_gcd_lcm() {
    (0..100).into_par_iter().for_each(|_| {
        let mut rnd = rand::rng();
        let a = rnd.random_range(2..=u32::MAX as u64);
        let b = rnd.random_range(2..=u32::MAX as u64);
        let gcd_r = reikna::factor::gcd(a, b);
        let gcd_t = u128_gcd(a as u128, b as u128) as u64;
        assert_eq!(gcd_r, gcd_t);
        let lcm_r = reikna::factor::lcm(a, b);
        let lcm_t = u128_lcm(a as u128, b as u128) as u64;
        assert_eq!(lcm_r, lcm_t);
    })
}

#[test]
fn find_highest_32bit_prime() {
    let mut found: u128 = 0;
    (0..5).for_each(|n| {
        let num: u128 = (u32::MAX - n) as u128;
        if u128_is_prime(num) {
            println!("#{n}: {num} is a prime number");
            found = num;
        }
    });
    assert_eq!(found, 4294967291);
}

#[test]
fn test_large_composites_above_mr_threshold() {
    // Composites above the MR threshold are rejected quickly by Miller-Rabin
    assert!(!u128_is_prime(170141183460469231731687303715884105729));
    assert!(!u128_is_prime(170141183460469231731687303715884105723));
    assert!(!u128_is_prime(65521 * 4294967291 * 68719476731 * 961748941));
    // Even numbers
    assert!(!u128_is_prime(618970019642690137449562112));
    // Squares of large primes
    assert!(!u128_is_prime(4294967291 * 4294967291 * 4294967291));
}

#[test]
fn test_prime_numbers_iterator() {
    let mut iter = PrimeNumbers::new();
    for i in 0..=10000 {
        let expected = nth_prime(i) as u128;
        let got = iter.next().unwrap();
        assert_eq!(got, expected, "prime #{i}: expected {expected}, got {got}");
    }
}

#[test]
#[ignore] // Very slow: trial-division fallback for primes above the MR threshold
fn test_large_primes_above_mr_threshold() {
    assert!(u128_is_prime(618970019642690137449562111));             // 2^89 - 1
    assert!(u128_is_prime(162259276829213363391578010288127));       // 2^107 - 1
    assert!(u128_is_prime(170141183460469231731687303715884105727)); // 2^127 - 1
}

#[test]
fn test_prev_prime() {
    assert_eq!(primefactor::prev_prime(2), Some(2));
    assert_eq!(primefactor::prev_prime(3), Some(3));
    assert_eq!(primefactor::prev_prime(10), Some(7));
    assert_eq!(primefactor::prev_prime(11), Some(11));
    assert_eq!(primefactor::prev_prime(12), Some(11));
    assert_eq!(primefactor::prev_prime(20), Some(19));
    assert_eq!(primefactor::prev_prime(210), Some(199)); // largest prime <= 210 is 199
    assert_eq!(primefactor::prev_prime(211), Some(211));
    assert_eq!(primefactor::prev_prime(212), Some(211));
    assert_eq!(primefactor::prev_prime(1), None);
}

#[test]
fn test_next_prime() {
    assert_eq!(primefactor::next_prime(0), 2);
    assert_eq!(primefactor::next_prime(1), 2);
    assert_eq!(primefactor::next_prime(2), 2);
    assert_eq!(primefactor::next_prime(3), 3);
    assert_eq!(primefactor::next_prime(10), 11);
    assert_eq!(primefactor::next_prime(11), 11);
    assert_eq!(primefactor::next_prime(12), 13);
    assert_eq!(primefactor::next_prime(20), 23);
    assert_eq!(primefactor::next_prime(210), 211);
    assert_eq!(primefactor::next_prime(211), 211);
    assert_eq!(primefactor::next_prime(212), 223);
}

#[test]
fn test_prime_numbers_iterator_rev() {
    let max_prime = reikna::prime::nth_prime(10000) as u128;
    let mut iter = primefactor::DescendingPrimes::from(max_prime);
    for i in (0..=10000).rev() {
        let expected = reikna::prime::nth_prime(i) as u128;
        let got = iter.next().unwrap();
        assert_eq!(got, expected, "prime #{i} (rev): expected {expected}, got {got}");
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_primefactors_methods() {
    let pf = primefactor::PrimeFactors::factorize(120);
    assert_eq!(pf.value(), 120);
    assert_eq!(pf.len(), 3); 
    assert_eq!(pf.count_factors(), 5); 
    assert!(!pf.is_empty());
    assert!(!pf.is_prime());

    let vec = pf.to_vec();
    assert_eq!(vec, vec![2, 2, 2, 3, 5]);

    let f = pf.factors();
    assert_eq!(f.len(), 3);
    assert_eq!(f[0].integer, 2);
    assert_eq!(f[0].exponent, 3);
    assert_eq!(f[1].integer, 3);
    assert_eq!(f[1].exponent, 1);
    assert_eq!(f[2].integer, 5);
    assert_eq!(f[2].exponent, 1);
    assert_eq!(pf.to_string(), "2^3 * 3 * 5");

    let empty_pf = primefactor::PrimeFactors::factorize(1);
    assert!(empty_pf.is_empty());
    assert_eq!(empty_pf.len(), 0);
    assert_eq!(empty_pf.count_factors(), 0);
    assert_eq!(empty_pf.value(), 1);
    assert_eq!(empty_pf.to_vec(), vec![]);
    assert_eq!(empty_pf.to_string(), "");

    let prime_pf = primefactor::PrimeFactors::factorize(13);
    assert!(prime_pf.is_prime());
    assert_eq!(prime_pf.len(), 1);
    assert_eq!(prime_pf.count_factors(), 1);
    assert_eq!(prime_pf.value(), 13);
    assert_eq!(prime_pf.to_string(), "13");
}

#[test]
fn test_intfactor_methods() {
    let f = primefactor::IntFactor { integer: 7, exponent: 3 };
    assert_eq!(f.to_vec(), vec![7, 7, 7]);
    assert_eq!(f.to_string(), "7^3");
    let f2 = primefactor::IntFactor { integer: 11, exponent: 1 };
    assert_eq!(f2.to_vec(), vec![11]);
    assert_eq!(f2.to_string(), "11");
}

#[test]
fn test_primewheel30() {
    let mut wheel = primefactor::candidates::PrimeWheel30::new();
    let expected = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59, 61, 67, 71
    ];
    for exp in expected {
        assert_eq!(wheel.next().unwrap(), exp);
    }
}
