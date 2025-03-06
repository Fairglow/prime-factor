use criterion::{black_box, criterion_group, criterion_main, Criterion};
use primefactor::*;
use std::time::Duration;
use crate::candidates::{PrimeWheel30, PrimeWheel210};

fn num_str(n: u64) -> String {
    let units = ["", "ki", "Mi", "Gi", "Ti", "Pi", "Ei"];
    let mask = (1 << 10) - 1;
    let mut r = n;
    for u in 0..units.len() {
        if r & mask > 0 {
            return format!("{}{}", r, units[u]);
        }
        r >>= 10;
    }
    format!("{}", n)
}

fn pf_number(n: u128) {
    let _ = PrimeFactors::from(n);
}

fn criterion_benchmark(c: &mut Criterion) {
    const BREAK_PRIME: u128 = 10000019;
    let count = 1 << 20; // 1Mi
    for shift in (0..32).step_by(10) {
        let base = 1 << shift;
        let name = format!("prime-factor  {} from {}",
            num_str(count), num_str(base));
        c.bench_function(&name, |b| b.iter(||
            pf_number(base as u128 + black_box(count as u128))));
    }
    let mut pw_grp = c.benchmark_group("prime-wheel");
    pw_grp.sample_size(10);
    pw_grp.bench_function("30-spokes   up to 10000019", |b| b.iter(|| {
        let mut pw_iter = PrimeWheel30::new();
        let mut sum: u128 = 0;
        while let Some(n) = pw_iter.next() {
            if n == BREAK_PRIME {
                break;
            }
            sum += n;
        }
        assert_eq!(sum, 13333376666710);
    }));
    pw_grp.bench_function("210-spokes  up to 10000019", |b| b.iter(|| {
        let mut pw_iter = PrimeWheel210::new();
        let mut sum: u128 = 0;
        while let Some(n) = pw_iter.next() {
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
    const FIXED_VEC: [(u8, u64, u128); 17] = [
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
        (56,   9, 72057594037927931),
        (60,  36, 1152921504606846883),
        (64,  60, 18446744073709551557),
        (68, 107, 295147905179352825833),
    ];
    for (bits, secs, prime) in FIXED_VEC.into_iter() {
        fixed_grp.measurement_time(Duration::new(secs, 0));
        let msg = format!("prime-factors for highest {:2}-bit prime number: {}", bits, prime);
        fixed_grp.bench_function(&msg, |b| b.iter(|| pf_number(prime)));
    }
    fixed_grp.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
