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
    fixed_grp.bench_function("prime-factor   lowest prime", |b| b.iter(||
        pf_number(2)));
    fixed_grp.bench_function("prime-factor   highest 8-bit prime", |b| b.iter(||
        pf_number(251)));
    fixed_grp.bench_function("prime-factor   lowest 9-bit prime", |b| b.iter(||
        pf_number(257)));
    fixed_grp.bench_function("prime-factor   highest 16-bit prime", |b| b.iter(||
        pf_number(65521)));
    fixed_grp.bench_function("prime-factor   lowest 17-bit prime", |b| b.iter(||
        pf_number(65537)));
    fixed_grp.bench_function("prime-factor   highest 32-bit prime", |b| b.iter(||
        pf_number(4294967291)));
    fixed_grp.bench_function("prime-factor   lowest 33-bit prime", |b| b.iter(||
        pf_number(4294967311)));
    fixed_grp.bench_function("prime-factor   highest 40-bit prime", |b| b.iter(||
        pf_number(1099511627689)));
    fixed_grp.bench_function("prime-factor   lowest 41-bit prime", |b| b.iter(||
        pf_number(1099511627791)));
    fixed_grp.bench_function("prime-factor   highest 48-bit prime", |b| b.iter(||
        pf_number(281474976710597)));
    fixed_grp.bench_function("prime-factor   lowest 49-bit prime", |b| b.iter(||
        pf_number(281474976710677)));
    fixed_grp.measurement_time(Duration::new(10, 0));
    fixed_grp.bench_function("prime-factor   highest 56-bit prime", |b| b.iter(||
        pf_number(72057594037927931)));
    fixed_grp.bench_function("prime-factor   lowest 57-bit prime", |b| b.iter(||
        pf_number(72057594037928017)));
    fixed_grp.measurement_time(Duration::new(60, 0));
    fixed_grp.bench_function("prime-factor   highest 64-bit prime", |b| b.iter(||
        pf_number(18446744073709551557)));
    fixed_grp.bench_function("prime-factor   lowest 65-bit prime", |b| b.iter(||
        pf_number(18446744073709551629)));
    fixed_grp.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
