// Copyright 2024 Protocol Labs
// SPDX-License-Identifier: MIT

use num_complex::{Complex64, ComplexFloat};
use scilib::math::bessel;

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
pub fn skellam_pmf(k: i32, mu1: f64, mu2: f64) -> f64 {
    // Parameterise and compute the Modified Bessel function of the first kind
    let nu = 2.0 * (mu1 * mu2).sqrt();
    let z = Complex64::new(k as f64, 0.0);
    let bessel = bessel::i_nu(nu, z);

    // Compute the pmf
    (-(mu1 + mu2)).exp() * (mu1 / mu2).powf(k as f64 / 2.0) * bessel.abs()
}