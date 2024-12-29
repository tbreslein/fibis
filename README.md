# Fibis

FIxed size BIt Set.

## Disclaimer

A lot of utility methods and trait implementations have not been implemented yet!
Also, the data structure is currently limited to elements of type `usize`. This
limitation will be lifted in the future, currently this crate is mostly a proof
of concept in order to see benchmark results.

## General

This crate implements a bit set (currently limited to `usize` elements) for
fixed lower and upper boundaries for your set. I.e., if you create a bit set
with boundaries `10` and `200`, then you can only insert elements in between
those two numbers into the set.

While this is a major limitation, whenever you can set constraints like this you
unlock a lot of performance gains, because you don't need to hash numbers.

The implementation for this type of data structure is fairly simple, so you
should be able to easily adapt this implementation to suit your own needs; just
remember to honor the MIT license.

## Benchmarks

This crate contains a benchmark file `benches/bench.rs`, which benchmark the
following operations:

- construction of new sets
- `.contains()` checks
- insertions (note that the bench run unfortunately includes a clone of the
  initial set before starting the insertions)

The benchmarks compare the following data structures:

- a regular `fibis::BitSet` with a stack allocated array as the backing container
- a `fibis::BitSet` variation that uses a `vec` as the backing container, but
  keeps the lower and upper boundaries as generic parameters (`fibis::vec::BitSet`)
- another `fibis::BitSet` variation that uses a `vec` as the backing container,
  and also drops the upper and lower boundary generic parameters; those are now
  runtime parameters (`fibis::vec2::BitSet`)
- an `IntSet` from the `integer_hasher` crate

The benchmarks have been run on two different machines: An M2 macbook pro, as
well as an `x86_64-linux` machine using a Ryzen 7 5800X CPU.

In general, all `fibis::*::BitSet`s beat the `IntSet` in all those benchmarks,
but the `IntSet` is also way more flexible then the `BitSet`s, allowing you to
insert any `usize` at any time.

Also note that the benchmark results are very different between the M2 and the
x86 architecture, with the latter producing a way larger gap between the
`IntSet` and the `BitSet`s.

I might make the the vector implementation the goto implementation, since it
seems to be the fastest in most or at least the most important benchmarks, while
also being easier to implement; the array implementation needs the array size
declared as a generic parameter, which is pretty annoying.

### M2 benchmark results

```
bench                              fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ construction                                  │               │               │               │         │
│  ├─ construction_1_000                         │               │               │               │         │
│  │  ├─ construction_bitset       1.843 µs      │ 1.905 µs      │ 1.864 µs      │ 1.862 µs      │ 100     │ 400
│  │  ├─ construction_bitset_vec   1.812 µs      │ 3.207 µs      │ 1.874 µs      │ 1.883 µs      │ 100     │ 400
│  │  ├─ construction_bitset_vec2  3.145 µs      │ 3.291 µs      │ 3.187 µs      │ 3.187 µs      │ 100     │ 200
│  │  ╰─ construction_intset       2.457 µs      │ 3.332 µs      │ 2.478 µs      │ 2.55 µs       │ 100     │ 200
│  ├─ construction_1_000_000                     │               │               │               │         │
│  │  ├─ construction_bitset       1.869 ms      │ 1.968 ms      │ 1.879 ms      │ 1.884 ms      │ 100     │ 100
│  │  ├─ construction_bitset_vec   1.946 ms      │ 1.992 ms      │ 1.961 ms      │ 1.964 ms      │ 100     │ 100
│  │  ├─ construction_bitset_vec2  3.39 ms       │ 4.342 ms      │ 3.446 ms      │ 3.474 ms      │ 100     │ 100
│  │  ╰─ construction_intset       2.469 ms      │ 3.57 ms       │ 2.515 ms      │ 2.525 ms      │ 100     │ 100
│  ├─ construction_10_000                        │               │               │               │         │
│  │  ├─ construction_bitset       18.58 µs      │ 23.37 µs      │ 18.95 µs      │ 19.03 µs      │ 100     │ 100
│  │  ├─ construction_bitset_vec   18.7 µs       │ 20.7 µs       │ 19.45 µs      │ 19.39 µs      │ 100     │ 100
│  │  ├─ construction_bitset_vec2  31.49 µs      │ 75.37 µs      │ 31.58 µs      │ 32.36 µs      │ 100     │ 100
│  │  ╰─ construction_intset       24.45 µs      │ 37.7 µs       │ 24.54 µs      │ 25.21 µs      │ 100     │ 100
│  ├─ construction_100                           │               │               │               │         │
│  │  ├─ construction_bitset       142.8 ns      │ 149.4 ns      │ 145.4 ns      │ 145.1 ns      │ 100     │ 3200
│  │  ├─ construction_bitset_vec   162.4 ns      │ 167.6 ns      │ 163.7 ns      │ 163.7 ns      │ 100     │ 3200
│  │  ├─ construction_bitset_vec2  338.2 ns      │ 757.4 ns      │ 348.5 ns      │ 352.8 ns      │ 100     │ 1600
│  │  ╰─ construction_intset       267.8 ns      │ 275.6 ns      │ 270.4 ns      │ 270.2 ns      │ 100     │ 1600
│  ╰─ construction_100_000                       │               │               │               │         │
│     ├─ construction_bitset       188.4 µs      │ 230.2 µs      │ 190 µs        │ 193.7 µs      │ 100     │ 100
│     ├─ construction_bitset_vec   209.6 µs      │ 226 µs        │ 209.8 µs      │ 210.8 µs      │ 100     │ 100
│     ├─ construction_bitset_vec2  314.9 µs      │ 421.2 µs      │ 315.6 µs      │ 318.7 µs      │ 100     │ 100
│     ╰─ construction_intset       244.7 µs      │ 343.7 µs      │ 244.8 µs      │ 247.8 µs      │ 100     │ 100
├─ contains                                      │               │               │               │         │
│  ├─ contains_1_000                             │               │               │               │         │
│  │  ├─ contains_bitset           162.4 ns      │ 573.8 ns      │ 166.3 ns      │ 170.2 ns      │ 100     │ 3200
│  │  ├─ contains_bitset_vec       167.6 ns      │ 399.4 ns      │ 168.9 ns      │ 171.8 ns      │ 100     │ 3200
│  │  ├─ contains_bitset_vec2      429.3 ns      │ 864.2 ns      │ 434.5 ns      │ 438.6 ns      │ 100     │ 1600
│  │  ╰─ contains_intset           473.5 ns      │ 1.03 µs       │ 478.8 ns      │ 484.7 ns      │ 100     │ 1600
│  ├─ contains_1_000_000                         │               │               │               │         │
│  │  ├─ contains_bitset           145.4 µs      │ 181.4 µs      │ 145.9 µs      │ 148.4 µs      │ 100     │ 100
│  │  ├─ contains_bitset_vec       165.2 µs      │ 203.7 µs      │ 177.8 µs      │ 178.8 µs      │ 100     │ 100
│  │  ├─ contains_bitset_vec2      428 µs        │ 470.2 µs      │ 428.1 µs      │ 432.8 µs      │ 100     │ 100
│  │  ╰─ contains_intset           718.4 µs      │ 854.7 µs      │ 723.5 µs      │ 728.9 µs      │ 100     │ 100
│  ├─ contains_10_000                            │               │               │               │         │
│  │  ├─ contains_bitset           1.499 µs      │ 1.947 µs      │ 1.541 µs      │ 1.557 µs      │ 100     │ 400
│  │  ├─ contains_bitset_vec       1.478 µs      │ 1.634 µs      │ 1.53 µs       │ 1.537 µs      │ 100     │ 400
│  │  ├─ contains_bitset_vec2      4.249 µs      │ 4.416 µs      │ 4.291 µs      │ 4.295 µs      │ 100     │ 100
│  │  ╰─ contains_intset           4.916 µs      │ 5.374 µs      │ 5.208 µs      │ 5.174 µs      │ 100     │ 100
│  ├─ contains_100                               │               │               │               │         │
│  │  ├─ contains_bitset           14.7 ns       │ 14.95 ns      │ 14.79 ns      │ 14.79 ns      │ 100     │ 51200
│  │  ├─ contains_bitset_vec       14.95 ns      │ 20.48 ns      │ 15.11 ns      │ 15.34 ns      │ 100     │ 51200
│  │  ├─ contains_bitset_vec2      47.82 ns      │ 49.13 ns      │ 48.48 ns      │ 48.54 ns      │ 100     │ 12800
│  │  ╰─ contains_intset           51.74 ns      │ 54.02 ns      │ 53.52 ns      │ 53.44 ns      │ 100     │ 12800
│  ╰─ contains_100_000                           │               │               │               │         │
│     ├─ contains_bitset           16.12 µs      │ 19.33 µs      │ 19.16 µs      │ 18.75 µs      │ 100     │ 100
│     ├─ contains_bitset_vec       16.12 µs      │ 19.37 µs      │ 19.16 µs      │ 18.67 µs      │ 100     │ 100
│     ├─ contains_bitset_vec2      42.79 µs      │ 42.99 µs      │ 42.83 µs      │ 42.84 µs      │ 100     │ 100
│     ╰─ contains_intset           58.7 µs       │ 68.79 µs      │ 59.12 µs      │ 59.54 µs      │ 100     │ 100
╰─ insert                                        │               │               │               │         │
   ├─ insert_1_000                               │               │               │               │         │
   │  ├─ contains_bitset           273 ns        │ 288.7 ns      │ 280.9 ns      │ 280.7 ns      │ 100     │ 1600
   │  ├─ contains_bitset_vec       257.4 ns      │ 275.7 ns      │ 265.2 ns      │ 264.7 ns      │ 100     │ 1600
   │  ├─ contains_bitset_vec2      926.6 ns      │ 984 ns        │ 937.1 ns      │ 942.6 ns      │ 100     │ 800
   │  ╰─ contains_intset           1.957 µs      │ 7.332 µs      │ 2.041 µs      │ 2.124 µs      │ 100     │ 100
   ├─ insert_1_000_000                           │               │               │               │         │
   │  ├─ contains_bitset           950.1 µs      │ 1.016 ms      │ 953.5 µs      │ 957.1 µs      │ 100     │ 100
   │  ├─ contains_bitset_vec       243.1 µs      │ 272 µs        │ 245.3 µs      │ 248.2 µs      │ 100     │ 100
   │  ├─ contains_bitset_vec2      945.8 µs      │ 1.204 ms      │ 959.6 µs      │ 977.1 µs      │ 100     │ 100
   │  ╰─ contains_intset           6.55 ms       │ 8.997 ms      │ 6.657 ms      │ 6.7 ms        │ 100     │ 100
   ├─ insert_10_000                              │               │               │               │         │
   │  ├─ contains_bitset           2.687 µs      │ 8.52 µs       │ 2.708 µs      │ 2.846 µs      │ 100     │ 200
   │  ├─ contains_bitset_vec       2.27 µs       │ 5.582 µs      │ 2.395 µs      │ 2.418 µs      │ 100     │ 200
   │  ├─ contains_bitset_vec2      9.165 µs      │ 9.374 µs      │ 9.249 µs      │ 9.242 µs      │ 100     │ 100
   │  ╰─ contains_intset           31.45 µs      │ 79.45 µs      │ 31.79 µs      │ 33.76 µs      │ 100     │ 100
   ├─ insert_100                                 │               │               │               │         │
   │  ├─ contains_bitset           51.74 ns      │ 53.69 ns      │ 52.7 ns       │ 52.6 ns       │ 100     │ 12800
   │  ├─ contains_bitset_vec       65.4 ns       │ 67.35 ns      │ 66.06 ns      │ 66.26 ns      │ 100     │ 6400
   │  ├─ contains_bitset_vec2      93.4 ns       │ 96.01 ns      │ 94.7 ns       │ 94.54 ns      │ 100     │ 6400
   │  ╰─ contains_intset           234 ns        │ 270.4 ns      │ 244.4 ns      │ 246.3 ns      │ 100     │ 1600
   ╰─ insert_100_000                             │               │               │               │         │
      ├─ contains_bitset           27.2 µs       │ 27.54 µs      │ 27.33 µs      │ 27.33 µs      │ 100     │ 100
      ├─ contains_bitset_vec       24.04 µs      │ 38.74 µs      │ 24.33 µs      │ 24.52 µs      │ 100     │ 100
      ├─ contains_bitset_vec2      92.08 µs      │ 103 µs        │ 92.24 µs      │ 92.71 µs      │ 100     │ 100
      ╰─ contains_intset           591.2 µs      │ 728.8 µs      │ 598.9 µs      │ 603 µs        │ 100     │ 100
```

### Ryzen 5800X results

```
bench                              fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ construction                                  │               │               │               │         │
│  ├─ construction_1_000                         │               │               │               │         │
│  │  ├─ construction_bitset       769.7 ns      │ 810 ns        │ 782.5 ns      │ 786.7 ns      │ 100     │ 400
│  │  ├─ construction_bitset_vec   1.01 µs       │ 1.079 µs      │ 1.019 µs      │ 1.021 µs      │ 100     │ 200
│  │  ├─ construction_bitset_vec2  2.249 µs      │ 9.87 µs       │ 2.269 µs      │ 2.358 µs      │ 100     │ 100
│  │  ╰─ construction_intset       3.669 µs      │ 8.539 µs      │ 3.679 µs      │ 3.731 µs      │ 100     │ 100
│  ├─ construction_1_000_000                     │               │               │               │         │
│  │  ├─ construction_bitset       806.3 µs      │ 873.4 µs      │ 831.3 µs      │ 832.9 µs      │ 100     │ 100
│  │  ├─ construction_bitset_vec   972 µs        │ 1.037 ms      │ 980.7 µs      │ 983.8 µs      │ 100     │ 100
│  │  ├─ construction_bitset_vec2  2.186 ms      │ 3.166 ms      │ 3.096 ms      │ 3.046 ms      │ 100     │ 100
│  │  ╰─ construction_intset       3.697 ms      │ 4.55 ms       │ 3.702 ms      │ 3.717 ms      │ 100     │ 100
│  ├─ construction_10_000                        │               │               │               │         │
│  │  ├─ construction_bitset       8.039 µs      │ 11.52 µs      │ 8.169 µs      │ 8.235 µs      │ 100     │ 100
│  │  ├─ construction_bitset_vec   9.759 µs      │ 12.81 µs      │ 9.77 µs       │ 9.828 µs      │ 100     │ 100
│  │  ├─ construction_bitset_vec2  30.82 µs      │ 33.53 µs      │ 30.85 µs      │ 30.95 µs      │ 100     │ 100
│  │  ╰─ construction_intset       36.38 µs      │ 39.06 µs      │ 36.46 µs      │ 36.57 µs      │ 100     │ 100
│  ├─ construction_100                           │               │               │               │         │
│  │  ├─ construction_bitset       66.63 ns      │ 73.19 ns      │ 69.13 ns      │ 69.07 ns      │ 100     │ 3200
│  │  ├─ construction_bitset_vec   86.32 ns      │ 89.47 ns      │ 89.13 ns      │ 88.4 ns       │ 100     │ 3200
│  │  ├─ construction_bitset_vec2  323.5 ns      │ 331 ns        │ 326 ns        │ 326.2 ns      │ 100     │ 800
│  │  ╰─ construction_intset       407.2 ns      │ 413.5 ns      │ 408.5 ns      │ 408.6 ns      │ 100     │ 800
│  ╰─ construction_100_000                       │               │               │               │         │
│     ├─ construction_bitset       80.56 µs      │ 88.37 µs      │ 84.49 µs      │ 84.88 µs      │ 100     │ 100
│     ├─ construction_bitset_vec   97.21 µs      │ 101.4 µs      │ 97.23 µs      │ 97.72 µs      │ 100     │ 100
│     ├─ construction_bitset_vec2  308.2 µs      │ 333.7 µs      │ 308.6 µs      │ 310.1 µs      │ 100     │ 100
│     ╰─ construction_intset       367 µs        │ 371.6 µs      │ 367.6 µs      │ 368.7 µs      │ 100     │ 100
├─ contains                                      │               │               │               │         │
│  ├─ contains_1_000                             │               │               │               │         │
│  │  ├─ contains_bitset           254.7 ns      │ 1.079 µs      │ 254.7 ns      │ 263.4 ns      │ 100     │ 200
│  │  ├─ contains_bitset_vec       127.2 ns      │ 127.9 ns      │ 127.8 ns      │ 127.6 ns      │ 100     │ 1600
│  │  ├─ contains_bitset_vec2      719.7 ns      │ 724.7 ns      │ 722.2 ns      │ 721.8 ns      │ 100     │ 400
│  │  ╰─ contains_intset           674.7 ns      │ 1.574 µs      │ 677.2 ns      │ 686.8 ns      │ 100     │ 400
│  ├─ contains_1_000_000                         │               │               │               │         │
│  │  ├─ contains_bitset           238.3 µs      │ 249.7 µs      │ 238.4 µs      │ 239.9 µs      │ 100     │ 100
│  │  ├─ contains_bitset_vec       119.4 µs      │ 126.8 µs      │ 119.5 µs      │ 120.3 µs      │ 100     │ 100
│  │  ├─ contains_bitset_vec2      714.9 µs      │ 726.8 µs      │ 718.3 µs      │ 717.9 µs      │ 100     │ 100
│  │  ╰─ contains_intset           1.69 ms       │ 3.274 ms      │ 1.703 ms      │ 1.723 ms      │ 100     │ 100
│  ├─ contains_10_000                            │               │               │               │         │
│  │  ├─ contains_bitset           1.214 µs      │ 1.244 µs      │ 1.214 µs      │ 1.216 µs      │ 100     │ 200
│  │  ├─ contains_bitset_vec       1.214 µs      │ 1.239 µs      │ 1.219 µs      │ 1.218 µs      │ 100     │ 200
│  │  ├─ contains_bitset_vec2      7.159 µs      │ 10.29 µs      │ 7.16 µs       │ 7.194 µs      │ 100     │ 100
│  │  ╰─ contains_intset           7.299 µs      │ 10.5 µs       │ 7.339 µs      │ 7.377 µs      │ 100     │ 100
│  ├─ contains_100                               │               │               │               │         │
│  │  ├─ contains_bitset           12.68 ns      │ 12.73 ns      │ 12.71 ns      │ 12.7 ns       │ 100     │ 25600
│  │  ├─ contains_bitset_vec       13.78 ns      │ 24.52 ns      │ 13.78 ns      │ 13.89 ns      │ 100     │ 25600
│  │  ├─ contains_bitset_vec2      72.26 ns      │ 72.6 ns       │ 72.29 ns      │ 72.4 ns       │ 100     │ 3200
│  │  ╰─ contains_intset           68.19 ns      │ 68.85 ns      │ 68.51 ns      │ 68.54 ns      │ 100     │ 3200
│  ╰─ contains_100_000                           │               │               │               │         │
│     ├─ contains_bitset           11.93 µs      │ 17.82 µs      │ 11.95 µs      │ 12.2 µs       │ 100     │ 100
│     ├─ contains_bitset_vec       11.94 µs      │ 14.98 µs      │ 11.95 µs      │ 11.98 µs      │ 100     │ 100
│     ├─ contains_bitset_vec2      71.48 µs      │ 74.41 µs      │ 71.51 µs      │ 71.76 µs      │ 100     │ 100
│     ╰─ contains_intset           120.7 µs      │ 135.6 µs      │ 121.1 µs      │ 122.4 µs      │ 100     │ 100
╰─ insert                                        │               │               │               │         │
   ├─ insert_1_000                               │               │               │               │         │
   │  ├─ contains_bitset           288.5 ns      │ 294.7 ns      │ 291 ns        │ 291.1 ns      │ 100     │ 800
   │  ├─ contains_bitset_vec       288.5 ns      │ 294.7 ns      │ 292.2 ns      │ 292.1 ns      │ 100     │ 800
   │  ├─ contains_bitset_vec2      617.2 ns      │ 1.414 µs      │ 619.7 ns      │ 627.7 ns      │ 100     │ 400
   │  ╰─ contains_intset           3.009 µs      │ 7.45 µs       │ 3.069 µs      │ 3.167 µs      │ 100     │ 100
   ├─ insert_1_000_000                           │               │               │               │         │
   │  ├─ contains_bitset           363.2 µs      │ 440.9 µs      │ 388.4 µs      │ 390.2 µs      │ 100     │ 100
   │  ├─ contains_bitset_vec       305.1 µs      │ 323.6 µs      │ 309.8 µs      │ 310.4 µs      │ 100     │ 100
   │  ├─ contains_bitset_vec2      713.7 µs      │ 939.3 µs      │ 729.4 µs      │ 731.3 µs      │ 100     │ 100
   │  ╰─ contains_intset           9.633 ms      │ 13.55 ms      │ 9.891 ms      │ 10 ms         │ 100     │ 100
   ├─ insert_10_000                              │               │               │               │         │
   │  ├─ contains_bitset           2.408 µs      │ 2.729 µs      │ 2.519 µs      │ 2.545 µs      │ 100     │ 100
   │  ├─ contains_bitset_vec       2.249 µs      │ 2.859 µs      │ 2.554 µs      │ 2.535 µs      │ 100     │ 100
   │  ├─ contains_bitset_vec2      5.298 µs      │ 5.539 µs      │ 5.379 µs      │ 5.375 µs      │ 100     │ 100
   │  ╰─ contains_intset           27.24 µs      │ 108.9 µs      │ 27.56 µs      │ 29.28 µs      │ 100     │ 100
   ├─ insert_100                                 │               │               │               │         │
   │  ├─ contains_bitset           40.38 ns      │ 41.16 ns      │ 40.69 ns      │ 40.68 ns      │ 100     │ 6400
   │  ├─ contains_bitset_vec       46.63 ns      │ 47.71 ns      │ 47.26 ns      │ 47.21 ns      │ 100     │ 6400
   │  ├─ contains_bitset_vec2      65.69 ns      │ 68.51 ns      │ 67.26 ns      │ 67.27 ns      │ 100     │ 3200
   │  ╰─ contains_intset           307.2 ns      │ 336 ns        │ 323.5 ns      │ 322.7 ns      │ 100     │ 800
   ╰─ insert_100_000                             │               │               │               │         │
      ├─ contains_bitset           28.95 µs      │ 59.45 µs      │ 40.53 µs      │ 41.01 µs      │ 100     │ 100
      ├─ contains_bitset_vec       22.44 µs      │ 25.99 µs      │ 22.69 µs      │ 22.79 µs      │ 100     │ 100
      ├─ contains_bitset_vec2      53.96 µs      │ 66.26 µs      │ 54.15 µs      │ 54.45 µs      │ 100     │ 100
      ╰─ contains_intset           726.1 µs      │ 1.166 ms      │ 739 µs        │ 743.7 µs      │ 100     │ 100

```

## License

> MIT License
>
> Copyright (c) 2024 Tommy Breslein
>
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the "Software"), to deal
> in the Software without restriction, including without limitation the rights
> to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
> copies of the Software, and to permit persons to whom the Software is
> furnished to do so, subject to the following conditions:
>
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
>
> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
> IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
> FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
> AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
> LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
> OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
> SOFTWARE.
