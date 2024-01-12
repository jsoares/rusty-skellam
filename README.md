# rusty-skellam

Trivial Rust implementation of the PMF for the [Skellam Distribution](https://en.wikipedia.org/wiki/Skellam_distribution).

Its precision is limited by the underlying [scilib](https://github.com/At0micBee/scilib) Bessel function.

## Usage

```rust
/// Calculate the probability mass function (PMF) of a Skellam distribution.
///
/// The Skellam distribution is the probability distribution of the difference
/// of two independent Poisson random variables.
///
/// # Arguments
///
/// * `k` - The difference of two Poisson random variables.
/// * `mu1` - The expected value of the first Poisson distribution.
/// * `mu2` - The expected value of the second Poisson distribution.
///
/// # Returns
///
/// * A `f64` representing the PMF of the Skellam distribution at `k`.
///
pub fn skellam_pmf(k: i32, mu1: f64, mu2: f64) -> f64 
```

## Example

```rust
let k: i32 = 2;
let mu1: f64 = 1.0;
let mu2: f64 = 1.0;
skellam_pmf(k, mu1, mu2)
// 0.09324018489817358
```
