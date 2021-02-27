use criterion::{black_box, criterion_group, criterion_main, Criterion};
use primefactor::*;
use rand::Rng;
use rand::prelude::*;
use rand::rngs::SmallRng;

//#[allow(unused_variables)]

fn num_str(n: u64) -> String {
    let units = ["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei"];
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
    PrimeFactors::from(n);
}

fn pf_random(rnd: &mut SmallRng, _n: u128) {
    PrimeFactors::from(rnd.gen_range(2..u32::MAX as u128));
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

    let mut rnd = SmallRng::from_rng(rand::thread_rng()).unwrap();
    c.bench_function("prime-factor   1Mi 32-bit rand", |b| b.iter(||
        pf_random(&mut rnd, black_box(count as u128))));
    }

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
