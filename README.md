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

Worst-case numbers (primes) on a modern system (i7-12700) with a 210-spoke Prime Wheel:

| Bitsize | Average Time |  Fastest |  Slowest |
|--------:|-------------:|---------:|---------:|
|       2 |       5.6 ns |   5.6 ns |   5.7 ns |
|       4 |      10.1 ns |  10.0 ns |  10.1 ns |
|       8 |      19.5 ns |  19.4 ns |  19.6 ns |
|      12 |      47.1 ns |  46.9 ns |  47.4 ns |
|      16 |     150.1 ns | 149.4 ns | 150.7 ns |
|      20 |     557.3 ns | 554.6 ns | 560.6 ns |
|      24 |      2.20 us |  2.20 us |  2.20 us |
|      28 |      8.60 us |  8.60 us |  8.70 us |
|      32 |      34.8 us |  34.7 us |  35.0 us |
|      36 |     136.4 us | 135.9 us | 137.0 us |
|      40 |     555.4 us | 553.7 us | 557.3 us |
|      44 |      2.21 ms |  2.20 ms |  2.22 ms |
|      48 |      8.90 ms |  8.88 ms |  8.92 ms |
|      52 |     35.21 ms | 35.05 ms | 35.41 ms |
|      56 |     140.4 ms | 139.9 ms | 141.0 ms |
|      60 |     559.0 ms | 557.9 ms | 560.2 ms |
|      64 |      2.24 s  |  2.23 s  |  2.25 s  |
|      68 |      8.94 s  |  8.93 s  |  8.95 s  |

The above numbers are taken from the included benchmark test, which you can run with the command: `cargo bench`. Note that it will take a few minutes to run the full suite, and in the meantime you should keep all other applications closed and leave the computer unattended, to give the benchmark the most processing power possible.

All in all, it takes about 5 minutes to run the full benchmark suite.

## Limitations & Practical Performance

While the library can parse and accept up to 128-bit unsigned integers, it currently uses **Trial Division** heavily optimized with a prime wheel. Trial Division operates in $O(\sqrt{N})$ time, meaning performance will vary wildly depending on the size of the *factors*, not just the size of the number.

If a large 128-bit number has small prime factors, it returns results instantly. However, for "worst-case scenario" numbers (semiprimes made of two similarly sized primes), the practical performance cut-offs look roughly like this:

* **Good (< 1 second): Up to ~62 bits.**
  Numbers this size or smaller are crunched virtually instantly. Great for normal use cases.
* **Acceptable (Seconds to Minutes): 63-75 bits.**
  You can expect a 68-bit worst-case integer to factorize in about 9 seconds. A 75-bit worst-case integer can take an hour. It is perfectly fine for background batch processing.
* **Not Practical (Hours, Days, or longer): > 75 bits.**
  If you attempt to factor a worst-case 100-bit or 128-bit number (like cryptographic keys) with this version of the library, the application will appear to hang. True 128-bit worst-case factorization would take centuries to compute with Trial Division!

> *Note: Future optimizations may incorporate sub-exponential algorithms like **Pollard's rho** which could significantly improve these upper bounds.*
