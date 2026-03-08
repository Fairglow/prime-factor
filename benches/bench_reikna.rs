use criterion::{criterion_group, criterion_main, Criterion};
use primefactor::*;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use std::hint::black_box;
use std::time::Duration;

fn num_str(n: u64) -> String {
    let units = ["", "ki", "Mi", "Gi", "Ti", "Pi", "Ei"];
    let mask = (1 << 10) - 1;
    let mut r = n;
    for unit in &units {
        if r & mask > 0 {
            return format!("{r}{unit}");
        }
        r >>= 10;
    }
    format!("{n}")
}

/// Fixed seed for reproducible random benchmarks.
const SEED: u64 = 0xDEAD_BEEF_CAFE_BABE;

fn sequential_factorization(c: &mut Criterion) {
    let count: u64 = 1 << 10; // 1ki
    let sprimes = reikna::prime::prime_sieve(reikna::factor::MAX_SMALL_NUM);

    for shift in [0u32, 10, 20, 30] {
        let base: u64 = 1 << shift;
        let label = num_str(base);

        let mut grp = c.benchmark_group(format!("factorize  {count} from {label}"));
        grp.bench_function("prime-factor", |b| b.iter(|| {
            for n in base..base + black_box(count) {
                let _ = PrimeFactors::factorize(n as u128);
            }
        }));
        grp.bench_function("reikna", |b| b.iter(|| {
            for n in base..base + black_box(count) {
                let _ = reikna::factor::quick_factorize_wsp(n, &sprimes);
            }
        }));
        grp.finish();
    }
}

fn sequential_next_prime(c: &mut Criterion) {
    let count: u64 = 1 << 10; // 1ki

    for shift in [0u32, 10, 20, 30] {
        let base: u64 = 1 << shift;
        let label = num_str(base);

        let mut grp = c.benchmark_group(format!("next_prime  {count} from {label}"));
        grp.bench_function("prime-factor", |b| b.iter(|| {
            let mut primes = PrimeNumbers::from(black_box(base as u128));
            for _ in 0..count {
                black_box(primes.next());
            }
        }));
        grp.bench_function("reikna", |b| b.iter(|| {
            let mut n = black_box(base);
            for _ in 0..count {
                n = reikna::prime::next_prime(n);
                black_box(n);
            }
        }));
        grp.finish();
    }
}

fn random_factorization(c: &mut Criterion) {
    let count: u64 = 1 << 10; // 1ki
    let sprimes = reikna::prime::prime_sieve(reikna::factor::MAX_SMALL_NUM);

    let mut least: u64 = 2;
    for bits in [16u32, 24, 32, 40] {
        let max = (1u64 << bits) - 1;
        let numbers: Vec<u64> = {
            let mut rng = StdRng::seed_from_u64(SEED);
            (0..count).map(|_| rng.random_range(least..=max)).collect()
        };

        let mut grp = c.benchmark_group(format!("factorize  {count} random {bits}-bit"));
        if bits >= 40 {
            grp.measurement_time(Duration::from_secs(10));
        }
        grp.sample_size(10);
        grp.bench_function("prime-factor", |b| b.iter(|| {
            for &n in &numbers {
                let _ = PrimeFactors::factorize(black_box(n as u128));
            }
        }));
        grp.bench_function("reikna", |b| b.iter(|| {
            for &n in &numbers {
                let _ = reikna::factor::quick_factorize_wsp(black_box(n), &sprimes);
            }
        }));
        grp.finish();
        least = max + 1;
    }
}

fn worst_case_primality(c: &mut Criterion) {
    // Highest primes per bit-width (every 4 bits, capped at 40 for reikna)
    const PRIMES: [(u8, u64, u64); 10] = [
        ( 4,  1, 13),
        ( 8,  1, 251),
        (12,  1, 4093),
        (16,  1, 65521),
        (20,  1, 1048573),
        (24,  1, 16777213),
        (28,  1, 268435399),
        (32,  1, 4294967291),
        (36,  1, 68719476731),
        (40,  5, 1099511627689),
    ];

    let mut grp = c.benchmark_group("is_prime  worst-case");
    grp.sample_size(10);
    for (bits, secs, prime) in PRIMES {
        grp.measurement_time(Duration::from_secs(secs));
        grp.bench_function(format!("prime-factor  {bits}-bit {prime}"), |b| {
            b.iter(|| u128_is_prime(black_box(prime as u128)))
        });
        grp.bench_function(format!("reikna       {bits}-bit {prime}"), |b| {
            b.iter(|| reikna::prime::is_prime(black_box(prime)))
        });
    }
    grp.finish();
}

criterion_group!(benches,
    sequential_factorization,
    sequential_next_prime,
    random_factorization,
    worst_case_primality,
);
criterion_main!(benches);
