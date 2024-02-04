# approximate

This crate is an implementation of [Scalable statistics counters](https://dl.acm.org/doi/10.1145/2486159.2486182), which allows, with some inaccuracy, to count atomically, but MUCH faster than standard atomic operations.

## Numbers

On my local machine, the 2 examples give the following average results, compared to atomic counters, using thread RNG ([`rand`](https://crates.io/crates/rand) crate):

`std`, using CPU atomics, on `u32`

time spent: 14.39%, error: 0.19%


`no_std`, using software atomics ([`atomic`](https://crates.io/crates/atomic) crate), on `u128`

time spent: 23.70%, error: 0.24%