# Prime Factor

[![Rust](https://github.com/Fairglow/prime-factor/actions/workflows/rust.yml/badge.svg)](https://github.com/Fairglow/prime-factor/actions/workflows/rust.yml)

The library will calculate all the prime number factors of any 128-bit unsigned integer (see [Limitations](#limitations--practical-performance) for large inputs). These are the prime factors that, when multiplied together with their multiplicities, reconstruct the original number. You can use the included application to play around with it.

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

Worst-case numbers (primes) on a modern system with a 210-spoke Prime Wheel and Miller-Rabin early exit:

| Bitsize | Average Time |  Fastest |  Slowest |
|--------:|-------------:|---------:|---------:|
|       2 |       5.4 ns |   5.3 ns |   5.4 ns |
|       4 |       9.8 ns |   9.8 ns |   9.9 ns |
|       8 |      18.9 ns |  18.8 ns |  19.0 ns |
|      12 |      46.0 ns |  46.0 ns |  46.0 ns |
|      16 |     145.4 ns | 145.2 ns | 145.8 ns |
|      20 |     548.0 ns | 548.0 ns | 548.3 ns |
|      24 |      1.85 us |  1.84 us |  1.86 us |
|      28 |      2.20 us |  2.20 us |  2.20 us |
|      32 |      2.72 us |  2.70 us |  2.74 us |
|      36 |      3.05 us |  3.05 us |  3.06 us |
|      40 |      3.21 us |  3.20 us |  3.22 us |
|      44 |      3.72 us |  3.71 us |  3.72 us |
|      48 |      3.76 us |  3.74 us |  3.78 us |
|      52 |      4.16 us |  4.14 us |  4.18 us |
|      56 |      4.53 us |  4.50 us |  4.56 us |
|      60 |      4.76 us |  4.73 us |  4.80 us |
|      64 |      5.11 us |  5.08 us |  5.13 us |
|      68 |     307.5 us | 305.3 us | 310.8 us |
|      70 |     344.5 us | 343.2 us | 345.8 us |
|      72 |     371.3 us | 371.0 us | 371.6 us |
|      74 |     387.2 us | 386.0 us | 388.8 us |
|      76 |     405.2 us | 403.9 us | 406.1 us |
|      78 |     441.7 us | 440.6 us | 442.7 us |
|      80 |     470.4 us | 468.9 us | 471.1 us |

For inputs up to 24 bits, pure trial division is used (below the Miller-Rabin crossover threshold). Above 24 bits, the deterministic Miller-Rabin test resolves primes in single-digit microseconds using native `u128` arithmetic. Above 64 bits, modular arithmetic requires a multi-precision fallback, increasing MR cost to hundreds of microseconds. The jump at 64→68 bits corresponds to this transition.

The above numbers are taken from the included benchmark test, which you can run with the command: `cargo bench`. Note that it will take a few minutes to run the full suite, and in the meantime you should keep all other applications closed and leave the computer unattended, to give the benchmark the most processing power possible.

All in all, it takes about 5 minutes to run the full benchmark suite.

## Limitations & Practical Performance

While the library can parse and accept up to 128-bit unsigned integers, it uses a hybrid approach: **deterministic Miller-Rabin** primality testing (proven correct for all numbers below ~3.3 × 10²⁴, approximately 82 bits) for quick prime detection, combined with **Trial Division** heavily optimized with a 210-spoke prime wheel for factoring composites.

**For primes**, Miller-Rabin gives an answer in microseconds for any value up to 64 bits, and hundreds of microseconds up to 80 bits (see the table above). Above the deterministic limit (~82 bits), a trial-division fallback verifies MR candidates, which can be slow for very large primes.

**For composites**, performance depends on the size of the *smallest prime factor*, not just the size of the number. Numbers with small factors decompose nearly instantly. The hard case is semiprimes (products of two large, similarly-sized primes), where trial division in $O(\sqrt{p})$ is needed to find the smaller factor $p$. Practical performance cut-offs for these worst-case composites:

* **Instant (< 1 ms):** Semiprimes with factors up to ~32 bits.
* **Good (< 1 second):** Semiprimes with the smallest factor up to ~40 bits.
* **Acceptable (seconds to minutes):** Smallest factor in the 40–44 bit range.
* **Not practical (hours+):** Smallest factor above ~48 bits. A semiprime composed of two 64-bit primes would take centuries to factorize with trial division.

> *Note: Future optimizations may incorporate sub-exponential algorithms like **Pollard's rho** which could significantly improve factorization of large semiprimes.*
