//! Utility functions for the Black-Scholes model implementations

use statrs::function::erf;

/// Calculate the cumulative distribution function for a standard normal distribution
pub fn normal_cdf(x: f64) -> f64 {
    erf::erf(x / 2.0_f64.sqrt()) * 0.5 + 0.5
}

/// Calculate the probability density function for a standard normal distribution
pub fn normal_pdf(x: f64) -> f64 {
    (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt()
}