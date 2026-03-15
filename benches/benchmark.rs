use criterion::{criterion_group, criterion_main, Criterion};
use primefactor::*;
use std::{hint::black_box, time::Duration};
use crate::candidates::{PrimeWheel30, PrimeWheel210};

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

fn pf_number(n: u128) {
    let _ = PrimeFactors::factorize(n);
}

fn criterion_benchmark(c: &mut Criterion) {
    const BREAK_PRIME: u128 = 10000019;
    let count: u64 = 1 << 10; // 1ki
    for shift in (0..32).step_by(10) {
        let base: u128 = 1 << shift;
        let name = format!("prime-factor  {} from {}",
            num_str(count), num_str(base as u64));
        c.bench_function(&name, |b| b.iter(|| {
            for n in base..base + black_box(count as u128) {
                pf_number(n);
            }
        }));
    }
    let mut pw_grp = c.benchmark_group("prime-wheel");
    pw_grp.sample_size(10);
    pw_grp.bench_function("30-spokes   up to 10000019", |b| b.iter(|| {
        let pw_iter = PrimeWheel30::new();
        let mut sum: u128 = 0;
        for n in pw_iter {
            if n == BREAK_PRIME {
                break;
            }
            sum += n;
        }
        assert_eq!(sum, 13333376666710);
    }));
    pw_grp.bench_function("210-spokes  up to 10000019", |b| b.iter(|| {
        let pw_iter = PrimeWheel210::new();
        let mut sum: u128 = 0;
        for n in pw_iter {
            if n == BREAK_PRIME {
                break;
            }
            sum += n;
        }
        assert_eq!(sum, 11428608571480);
    }));
    pw_grp.finish();

    let mut fixed_grp = c.benchmark_group("worst-case");
    fixed_grp.sample_size(10);
    fixed_grp.bench_function("prime-factor   lowest prime: 2", |b| b.iter(||
        pf_number(2)));
    const FIXED_VEC: [(u8, u64, u128); 23] = [
        ( 4,   1, 13),
        ( 8,   1, 251),
        (12,   1, 4093),
        (16,   1, 65521),
        (20,   1, 1048573),
        (24,   1, 16777213),
        (28,   1, 268435399),
        (32,   1, 4294967291),
        (36,   1, 68719476731),
        (40,   1, 1099511627689),
        (44,   1, 17592186044399),
        (48,   1, 281474976710597),
        (52,   1, 4503599627370449),
        (56,   1, 72057594037927931),
        (60,   1, 1152921504606846883),
        (64,   1, 18446744073709551557),
        (68,   1, 295147905179352825833),
        (70,   1, 1180591620717411303389),
        (72,   1, 4722366482869645213603),
        (74,   1, 18889465931478580854749),
        (76,   1, 75557863725914323419121),
        (78,   1, 302231454903657293676533),
        (80,   1, 1208925819614629174706111),
    ];
    for (bits, secs, prime) in FIXED_VEC.into_iter() {
        fixed_grp.measurement_time(Duration::new(secs, 0));
        let msg = format!("prime-factors for highest {bits:2}-bit prime number: {prime}");
        fixed_grp.bench_function(&msg, |b| b.iter(|| pf_number(prime)));
    }
    fixed_grp.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
