[![Rust](https://github.com/Fairglow/prime-factor/actions/workflows/rust.yml/badge.svg)](https://github.com/Fairglow/prime-factor/actions/workflows/rust.yml)

# Prime Factor

The library will calculate all the prime number factors of any 128-bit unsigned integer. These are all the smallest values that when multiplied with each other produces that number. You can use the included application to play around with it.

## Memory efficiency

A lot of prime number algorithms require a significant amount of memory, but accessing main memory can be a slow process[^1]. While the cache can provide some assistance, it may not be sufficient. With each load from main memory, there is typically enough time for up to hundreds of calculations. These cycles would be wasted, unless we can find some work to do while waiting for the load. Therefore, even with some amount of wasted computations, we can still achieve an efficient algorithm if we can minimize the number of memory operations.

In my personal experience any algorithm that needs to store a lot of data will be limited by the memory accesses, it is often faster to recreate some computations rather than loading them from memory. Therefore do not save values to memory that can be easily computed.

One of the design goals for this code is to minimize the memory overhead during factorization. Only the final factors will be saved to memory, with an exponent, and none of which will be read back during operation.

[^1]: [Latency Numbers Every Programmer Should Know](https://gist.github.com/jboner/2841832)

## Prime wheel iterator

The code uses an iterator to find potential prime candidates and its algorithm must guarantee that all primes are generated, but it may produce some false positives. The cost of a false positive is one loop with one modulo calculation. Note that any non-prime value from the iterator will have all its factors appear among the already generated numbers and therefore they can never appear in the final output.

We want this iterator to be fast and give reasonably good guesses for prime numbers. For this purpose we use a prime wheel[^2] function with a base of 210. In the first million of numbers it has a hit-rate of about 30.8%, which is pretty good considering its speed. Consider that a false positive is not that expensive, but a false negative is a fatal flaw. I fully expect the hit-rate to drop for higher numbers. The 30-spoke prime wheel has a 26.7% hit-rate.

[^2]: See the Wikipedia article on [wheel factorization](https://en.wikipedia.org/wiki/Wheel_factorization) for more information.

## Factorization performance

Modern system (i7-12700) with a 210-spoke Prime Wheel:

| Bitsize | Average Time | Min .. Max        |
|---------|--------------|-------------------|
| 2       | 11.3 ns      | 11.29 .. 11.39 ns |
| 4       | 12.0 ns      | 11.96 .. 11.99 ns |
| 8       | 20.5 ns      | 20.51 .. 20.53 ns |
| 12      | 50.1 ns      | 50.09 .. 50.22 ns |
| 16      | 177 ns       | 176.6 .. 177.9 ns |
| 20      | 632 ns       | 626.0 .. 654.8 ns |
| 24      | 2.44 us      | 2.439 .. 2.445 us |
| 28      | 9.68 us      | 9.667 .. 9.699 us |
| 32      | 38.1 us      | 37.93 .. 38.39 us |
| 36      | 155 us       | 154.5 .. 155.4 us |
| 40      | 609 us       | 607.6 .. 610.1 us |
| 44      | 2.44 ms      | 2.435 .. 2.459 ms |
| 48      | 9.91 ms      | 9.901 .. 9.916 ms |
| 52      | 39.5 ms      | 39.25 .. 39.75 ms |
| 56      | 159 ms       | 158.5 .. 159.3 ms |
| 60      | 637 ms       | 634.8 .. 639.3 ms |
| 64      | 2.55 s       | 2.539 .. 2.551 s  |
| 68      | 10.2 s       | 10.18 .. 10.22 s  |

The above numbers are taken from the included benchmark test, which you can run with the command: `cargo bench`. Note that it will take a few minutes to run the full suite, and in the meantime you should keep all other applications closed and leave the computer unattended, to give the benchmark the most processing power possible.

All in all, it takes almost 10 minutes to run the full benchmark suite.
