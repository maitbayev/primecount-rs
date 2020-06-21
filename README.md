# primecount-rs

primecount-rs is a library that provides APIs for counting the primes below an integer x ≤ 10^31 
using highly optimized implementations of the combinatorial 
[prime counting algorithms](https://en.wikipedia.org/wiki/Prime-counting_function#Algorithms_for_evaluating_%CF%80(x)).

It is a rust wrapper around an awesome [kimwalisch/primecount](https://github.com/kimwalisch/primecount) library.

- [crates.io/crates/primecount](https://crates.io/crates/primecount)
- [docs.rs/primecount/](https://docs.rs/primecount/)

## API

**Add to Cargo.toml of your project:**
```
primecount = "0.1.1"
```

**Examples:**
```rust
use primecount;

fn main() {
    println!("Primes below 1000 = {}", primecount::pi(1000));
    println!("Numbers below 1000 that are not divisible by any of the first 100 primes (a.k.a. Legendre-sum) = {}", primecount::phi(1000, 100));
    println!("10th prime = {}", primecount::nth_prime(10));
}
```

## Contribute

**Update primecount dependency**:

```
git submodule update --init --recursive
```
