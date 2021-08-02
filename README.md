# Prime Factor

The library will calculate all the prime number factors of any 128-bit unsigned integer. These are all the smallest values that when multiplied with each other produces that number. You can use the included application to play around with it.

## Prime Numbers

"Is that true that all the prime numbers are of the form 6m±1?" -- [Math StackExchange](https://math.stackexchange.com/questions/41623/is-that-true-that-all-the-prime-numbers-are-of-the-form-6m-pm-1)

"This is true of all prime numbers except for 2 and 3. The reason is that numbers with remainders 0, 2 and 4 modulo 6 are divisible by 2, and numbers with remainders 0 and 3 modulo 6 are divisible by 3, so other than 2 and 3 themselves, all prime numbers must have remainder 1 or 5 modulo 6." -- [joriki](https://math.stackexchange.com/users/6622/joriki)

Another observation that I made was that numbers ending in 5 occur with a periodicity of 10, at positions 6 and 9 when starting from 7, forming the end-digit pattern:
`7, 1, 3, 7, 9, 3, 5, 9, 1, 5`

Example:
```
  07, 11, 13, 17, 19, 23, 25, 29, 31, 35,
  37, 41, 43, 47, 49, 53, 55, 59, 61, 65,
  ...
```

## Memory efficient operation

Memory accesses are typically quite slow and a lot of the algorithms for calculating prime numbers use quite a lot of memory. During a single memory load there is time for many calculations, which will be wasted cycles unless we can utilize them somehow. This means that we can get an efficient algorithm even with a bit of wasted computations if we can limit the number of memory operations.

In my personal experience any algorithm that needs to store a lot of data will be limited by the memory accesses, it is often faster to recreate some computations rather than loading them from memory. Therefore do not save values to memory that can be easily computed.

One of the design goals for this code is to minimize the memory overhead during factorization. Only the final factors will be saved to memory, with an exponent, and none of which will be read back during operation.

## Generators

I know these have not been stabilized yet in Rust. In the code I will use genawaiter which is based on the async handling in Rust. Using generators allows me to have minimal state while running. This is a slight optimization over an iterator, where you'd have to do a bit more work to achieve the same output, because the state must be saved and loaded between calls.

The code uses a generator to find potential prime candidates. It must guarantee that all primes are generated, but it is ok if it also produces some false positives. The cost is one loop with one modulo calculation. Note that any non-prime value from the generator will have all its factors appear among the already generated numbers. Therefore they can never appear in the final output.

We want this generator to be fast and give a reasonably good guess for prime numbers. In the first million of numbers it has a hit-rate of about 26.7%, which is pretty good considering its speed. Consider that a false positive is not that expensive, but a false negative is a fatal flaw. I fully expect the hit-rate to drop for even higher numbers.

If you know of any better prime candidate generator, please let me know!

## Performance

Typically, any 32-bit value can be factorized within a millisecond on a reasonably modern system. On my own machine, which is a few years old but was quite good at the time, I average about 600 nanoseconds for values around 1 Gi.

64-bit values greater than 32-bit, are typically factorized between 0.4 to 2.5 seconds in the benchmark results on my machine, which also factorizes the worst case 32-bit prime number in about 250 ms and the worst case 64-bit prime number in about 20.5 seconds.

The above numbers are taken from the included benchmark test, which you can run with the command: `cargo bench`. Note that it will take a few minutes to run the full suite (about 15 minutes on my machine).
