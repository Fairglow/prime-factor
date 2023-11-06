#[allow(unused_imports)]
use rand::Rng;
use rayon::prelude::*;
use reikna;
use genawaiter::stack::let_gen_using;
use primefactor::{
    candidates::prime_wheel_30,
    primefactor_gcd,
    PrimeFactors,
    u128_gcd,
    u128_is_prime,
    u128_lcm,
    u128_sqrt};

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
fn test_early_prime_candidate_numbers() {
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

#[test]
fn test_is_prime() {
    for num in 2..=1000 {
        let prime = u128_is_prime(num);
        assert_eq!(reikna::prime::is_prime(num as u64), prime,
                   "is num {} prime?", num);
    }
}

#[test]
fn test_some_factors() {
    let mut rnd = rand::thread_rng();
    for _ in 0..1000 {
        let num = rnd.gen_range(2..u32::MAX as u128);
        let facts = PrimeFactors::from(num);
        assert_eq!(reikna::prime::is_prime(num as u64), facts.is_prime());
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
    assert_eq!(primefactor_gcd(2*3*5*7, 2*5*11), PrimeFactors::from(2*5));
    assert_eq!(primefactor_gcd(3*4*5, 3*4*7), PrimeFactors::from(3*4));
    assert_eq!(primefactor_gcd(9*4*11, 3*8*13), PrimeFactors::from(3*4));
    assert_eq!(primefactor_gcd(27*64*121, 9*32*49), PrimeFactors::from(9*32));
    let no_gcd = primefactor_gcd(3*7*13, 2*5*11);
    assert!(no_gcd.is_empty());
    assert!(primefactor_gcd(1, 1).is_empty());
    assert!(primefactor_gcd(1, 0).is_empty());
    assert!(primefactor_gcd(0, 1).is_empty());
    assert!(primefactor_gcd(0, 0).is_empty());
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
        let mut rnd = rand::thread_rng();
        let a = rnd.gen_range(2..u32::MAX as u128);
        let b = rnd.gen_range(2..u32::MAX as u128);
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
        let mut rnd = rand::thread_rng();
        let a = rnd.gen_range(2..u32::MAX as u128);
        let b = rnd.gen_range(2..u32::MAX as u128);
        let c = rnd.gen_range(2..u32::MAX as u128);

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
        let mut rnd = rand::thread_rng();
        let a = rnd.gen_range(2..u32::MAX as u64);
        let b = rnd.gen_range(2..u32::MAX as u64);
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
    (0..5).into_iter().for_each(|n| {
        let num: u128 = (u32::MAX - n) as u128;
        if u128_is_prime(num) {
            println!("#{}: {} is a prime number", n, num);
            found = num;
        }
    });
    assert_eq!(found, 4294967291);
}

#[test]
fn worst_case_64bit_prime() {
    let num: u128 = (u64::MAX - 58) as u128;
    assert_eq!(num, 18446744073709551557);
    assert_eq!(u128_is_prime(num), true);
}
