[![Rust](https://github.com/Fairglow/prime-factor/actions/workflows/rust.yml/badge.svg)](https://github.com/Fairglow/prime-factor/actions/workflows/rust.yml)

# Prime Factor

The library will calculate all the prime number factors of any 128-bit unsigned integer. These are all the smallest values that when multiplied with each other produces that number. You can use the included application to play around with it.

## Memory efficiency

A lot of prime number algorithms require a significant amount of memory, but accessing main memory can be a slow process[^1]. While the cache can provide some assistance, it may not be sufficient. With each load from main memory, there is typically enough time for up to hundreds of calculations. These cycles would be wasted, unless we can find some work to do while waiting for the load. Therefore, even with some amount of wasted computations, we can still achieve an efficient algorithm if we can minimize the number of memory operations.

In my personal experience any algorithm that needs to store a lot of data will be limited by the memory accesses, it is often faster to recreate some computations rather than loading them from memory. Therefore do not save values to memory that can be easily computed.

One of the design goals for this code is to minimize the memory overhead during factorization. Only the final factors will be saved to memory, with an exponent, and none of which will be read back during operation.

[^1]: [Latency Numbers Every Programmer Should Know](https://gist.github.com/jboner/2841832)

## Prime wheel generator

I know [generators have not been stabilized](https://github.com/rust-lang/rust/issues/43122) in Rust yet. However, in the code I will use [genawaiter](https://github.com/whatisaphone/genawaiter) which is based on the Rust async handling. Using generators allows me to have minimal state while running. This is a slight optimization over an iterator, where you'd have to do a bit more work to achieve the same output, because an iterator restarts its function while a generator continues from its last call.

The code uses a generator to find potential prime candidates and its algorithm must guarantee that all primes are generated, but it may produce some false positives. The cost of a false positive is one loop with one modulo calculation. Note that any non-prime value from the generator will have all its factors appear among the already generated numbers and therefore they can never appear in the final output.

We want this generator to be fast and give reasonably good guesses for prime numbers. For this purpose we use a prime wheel[^2] function with a base of 30. In the first million of numbers it has a hit-rate of about 26.7%, which is pretty good considering its speed. Consider that a false positive is not that expensive, but a false negative is a fatal flaw. I fully expect the hit-rate to drop for higher numbers.

[^2]: See the Wikipedia article on [wheel factorization](https://en.wikipedia.org/wiki/Wheel_factorization) for more information.

## Factorization performance

On an old system (i7-6700), with a 30-spoke wheel:
- 32-bit, random number in about 32 ms and worst case 300 ms
- 64-bit, random number in about 1.4 s (± 1.1 s) with worst case about 20 s
- full benchmark completes in about 7 minutes

On a modern system (i7-12700), with a 30-spoke wheel:
- 32-bit, random number in about 6.5 us and worst case in 68 us
- 64-bit, random number in about 140 ms ([3 .. 340] ms) and worst case in 4.6 s
- full benchmark completes in less than 3 minutes

Modern system (i7-12700) with a 210-spoke Prime Wheel:
- 2..8-bit prime numbers in 12..34 ns
- 9..16-bit prime numbers in 33..252 ns
- 17..32-bit prime numbers in 0.25..57 us, on average 5.6 us
- 33..64-bit prime numbers in 0.056..3704 ms
- 65+ bits prime numbers from 3.65 s

The above numbers are taken from the included benchmark test, which you can run with the command: `cargo bench`. Note that it will take a few minutes to run the full suite, in which time you need to close all other applications and leave it unattended, to give the benchmark the most processing power possible.
