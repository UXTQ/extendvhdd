
# Comparison with other hashers

[![Comparison chart](Table.png)](https://docs.google.com/spreadsheets/d/e/2PACX-1vSK7Li2nS-Bur9arAYF9IfT37MP-ohAe1v19lZu5fd9MajI1fSveLAQZyEie4Ea9k5-SWHTff7nL2DW/pubhtml?gid=0&single=true)

## Design

AHash is designed *exclusively* for use in in-memory hashmaps. It does not have a fixed standard, but uses different
algorithms depending on the availability of hardware instructions. Whenever possible aHash uses the [hardware AES instruction](https://en.wikipedia.org/wiki/AES_instruction_set)
on X86 processors when it is available. If no specialized instructions are available, it falls back on an
[algorithm based on multiplication](https://github.com/tkaitchuck/aHash/wiki/AHash-fallback-algorithm)).

Because aHash does not have a fixed standard for its output, it can optimize its performance to a much greater extent than
algorithms which don't have this flexibility. This is great for Hashmaps but makes aHash inappropriate for applications where
a hash needs to be sent over the network, or persisted.

## Quality

**AHash passes the full [SMHasher test suite](https://github.com/rurban/smhasher)** both with and without AES support.
The output of the tests is checked into the [smhasher subdirectory](../smhasher). 

At **over 50GB/s** aHash is the fastest algorithm to pass the full test suite by more than a factor of 2. 
Even the fallback algorithm is in the top 5 in terms of throughput, beating out many other algorithms that rely on SSE and AVX instructions.

## Speed

aHash is the fastest non-trivial hasher implementation in Rust. Below is a comparison with 10 other popular hashing algorithms.  

![Hasher perfromance](https://docs.google.com/spreadsheets/d/e/2PACX-1vSK7Li2nS-Bur9arAYF9IfT37MP-ohAe1v19lZu5fd9MajI1fSveLAQZyEie4Ea9k5-SWHTff7nL2DW/pubchart?oid=1323618938&format=image)

## DOS resistance

AHash provides DOS resistance by incorporating random keys into the hash. There is a full explanation [here](https://github.com/tkaitchuck/aHash/wiki/How-aHash-is-resists-DOS-attacks).

If the `std` feature flag is enabled (On by default) it uses the `getrandom` crate to generate random seeds upon initialization.
If `std` is disabled and the `compile-time-rng` flag is enabled instead it will use the `const-random` to generate random seeds 
at compile time and embed them into the application binary.  

If neither `std` or `compile-time-rng` flags are enabled aHash will fall back on using the numeric value of memory addresses as a source of randomness.
This is somewhat strong if ALSR is turned on (it is by default in Rust) but for some platforms this is not available.
As a result this should not be relied on. For this reason it is strongly recommended that if you disable `std` because you program needs to run in a 
`no-std` environment to enable the `compile-time-rng` feature flag.


# Why use aHash over X