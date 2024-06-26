use criterion::{black_box, criterion_group, criterion_main, Criterion};
use primefactor::*;
use rand::Rng;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::ops::RangeInclusive;
use std::time::Duration;

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

fn pf_random(rnd: &mut SmallRng, interval: RangeInclusive<u64>, _n: u128) {
    let _ = PrimeFactors::from(rnd.gen_range(interval) as u128);
}

fn criterion_benchmark(c: &mut Criterion) {
    let count = 1 << 20; // 1Mi
    for shift in (0..32).step_by(10) {
        let base = 1 << shift;
        let name = format!("prime-factor  {} from {}",
            num_str(count), num_str(base));
        c.bench_function(&name, |b| b.iter(||
            pf_number(base as u128 + black_box(count as u128))));
    }

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
    fixed_grp.measurement_time(Duration::new(60, 0));
    fixed_grp.bench_function("prime-factor   highest 64-bit prime", |b| b.iter(||
        pf_number(18446744073709551557)));
    fixed_grp.bench_function("prime-factor   lowest 65-bit prime", |b| b.iter(||
        pf_number(18446744073709551629)));
    fixed_grp.finish();

    let mut rand_grp = c.benchmark_group("random-numbers");
    let mut rnd = SmallRng::from_rng(rand::thread_rng()).unwrap();
    rand_grp.bench_function("prime-factor   1Mi random 32-bit (> 16-bit)",
        |b| b.iter(|| pf_random(
            &mut rnd, u16::MAX as u64..=u32::MAX as u64, black_box(1 << 20))));
    rand_grp.sample_size(20);
    rand_grp.measurement_time(Duration::new(30, 0));
    rand_grp.bench_function("prime-factor   20  random 64-bit (> 32-bit)",
        |b| b.iter(|| pf_random(
            &mut rnd, u32::MAX as u64..=u64::MAX, black_box(20))));
    rand_grp.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
