#[allow(unused_imports)]
use primefactor::*;
use rand::Rng;
use genawaiter::{yield_, stack::let_gen};

fn is_prime(n: u128) -> bool {
    let_gen!(mpgen, {
        yield_!(2);
        yield_!(3);
        yield_!(5);
        yield_!(7);
        // All remaining prime numbers must end in either of 1, 3, 7 or 9
        let mut accum: u128 = 11;
        loop {
            yield_!(accum); // ending in 1
            accum += 2;
            yield_!(accum); // ending in 3
            accum += 4;
            yield_!(accum); // ending in 7
            accum += 2;
            yield_!(accum); // ending in 9
            accum += 2;
        }
    });
    // A factor of n must have a value less than or equal to sqrt(n)
    let maxf = u128_sqrt(n) + 1;
    for p in mpgen.into_iter() {
        if p >= maxf {
            break;
        }
        if n % p == 0 {
            return false;
        }
    }
    return true;
}
    
#[test]
fn test_is_prime() {
    for num in 1..1000 {
        let facts = PrimeFactors::from(num);
        let prime = facts.is_prime();
        assert_eq!(is_prime(num), prime, "is num {} prime?", num);
        let sum: u128 = facts.to_vec().iter()
            .map(|fc| fc.prime.pow(fc.exponent))
            .product();
        assert_eq!(num, sum);
    }
}

#[test]
fn test_some_factors() {
    let mut rnd = rand::thread_rng();
    for _ in 0..1000 {
        let num = rnd.gen_range(1..u32::MAX as u128);
        let facts = PrimeFactors::from(num);
        if facts.is_prime() {
            assert_eq!(is_prime(num), true);
            let fe = &facts.to_vec()[0];
            assert_eq!(fe.prime, num);
            assert_eq!(fe.exponent, 1);
        } else {
            let sum: u128 = facts.to_vec().iter()
                .map(|fc| fc.prime.pow(fc.exponent))
                .product();
            assert_eq!(num, sum);
        }
    }
}

#[test]
fn test_a_few_gcd() {
    assert_eq!(u128_gcd(2*3*5*7, 2*5*11), PrimeFactors::from(2*5));
    assert_eq!(u128_gcd(3*4*5, 3*4*7), PrimeFactors::from(3*4));
    assert_eq!(u128_gcd(9*4*11, 3*8*13), PrimeFactors::from(3*4));
    assert_eq!(u128_gcd(27*64*121, 9*32*49), PrimeFactors::from(9*32));
    let no_gcd = u128_gcd(3*7*13, 2*5*11);
    assert!(no_gcd.is_empty());
}
