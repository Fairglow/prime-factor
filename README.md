# Prime Factor

The library will calculate all the prime number factors of any 128-bit unsigned integer. These are all the smallest values that when multiplied with each other produces that number. You can use the included application to play around with it.

## Memory efficiency

Main memory accesses are typically quite slow[^1] and a lot of the algorithms for calculating prime numbers use quite a lot of memory, the cache helps but not enough. During a single main memory load there is time for in the order of hundreds of calculations, which will be wasted cycles unless we can utilize them somehow. This means that we can get an efficient algorithm even with a bit of wasted computations if we can limit the number of memory operations.

In my personal experience any algorithm that needs to store a lot of data will be limited by the memory accesses, it is often faster to recreate some computations rather than loading them from memory. Therefore do not save values to memory that can be easily computed.

One of the design goals for this code is to minimize the memory overhead during factorization. Only the final factors will be saved to memory, with an exponent, and none of which will be read back during operation.

[^1]: [Latency Numbers Every Programmer Should Know](https://gist.github.com/jboner/2841832)

## Prime wheel generator

I know [generators have not been stabilized](https://github.com/rust-lang/rust/issues/43122) in Rust yet. However, in the code I will use [genawaiter](https://github.com/whatisaphone/genawaiter) which is based on the Rust async handling. Using generators allows me to have minimal state while running. This is a slight optimization over an iterator, where you'd have to do a bit more work to achieve the same output, because an iterator restarts its function while a generator continues from its last call.

The code uses a generator to find potential prime candidates and its algorithm must guarantee that all primes are generated, but it may produce some false positives. The cost of a false positive is one loop with one modulo calculation. Note that any non-prime value from the generator will have all its factors appear among the already generated numbers and therefore they can never appear in the final output.

We want this generator to be fast and give reasonably good guesses for prime numbers. For this purpose we use a prime wheel[^2] function with a base of 30. In the first million of numbers it has a hit-rate of about 26.7%, which is pretty good considering its speed. Consider that a false positive is not that expensive, but a false negative is a fatal flaw. I fully expect the hit-rate to drop for higher numbers.

[^2]: See the Wikipedia article on [wheel factorization](https://en.wikipedia.org/wiki/Wheel_factorization) for more information.

## Factorization performance

On an old system (i7-6700):
- 32-bit, random number in about 32 ms and worst case 300 ms
- 64-bit, random number in about 1.4 s (± 1.1 s) with worst case about 20 s
- full test suite completes in about 7 minutes

On a modern system (i7-12700):
- 32-bit, random number in about 7 us and worst case in 66.5 us
- 64-bit, random number in about 348 ms ([58 .. 765] ms) and worst case in 4.44 s
- full test suite completes in less than 3 minutes

The above numbers are taken from the included benchmark test, which you can run with the command: `cargo bench`. Note that it will take a few minutes to run the full suite, in which time you need to close all other applications and leave it unattended, to give the benchmark the most processing power possible.
