# rusty-skellam
Makeshift Rust implementation of Skellam Distribution's PMF.

## Example
```rust
let k: i32 = 2;
let mu1: f64 = 1.0;
let mu2: f64 = 1.0;
let pmf: f64 = skellam_pmf(k, mu1, mu2);
println!("skellam({},{},{}): {}", k, mu1, mu2, pmf);
```
