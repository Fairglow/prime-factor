#![deny(unsafe_code)]
#![allow(dead_code)]
use genawaiter::yield_;
use genawaiter::stack::producer_fn;

/// Wheel factorization algorithm with base {2, 3, 5}
/// https://en.wikipedia.org/wiki/Wheel_factorization
#[producer_fn(u128)]
pub async fn prime_wheel_30() {
    yield_!(2);
    yield_!(3);
    yield_!(5);
    let mut base = 0u128;
    loop {
        for n in [7, 11, 13, 17, 19, 23, 29, 31] {
            yield_!(base + n);
        }
        base += 30;
    }
}
