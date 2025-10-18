//! Minimal WebAssembly bindings for the Trillion Dollar Equation project
//!
//! This module provides a minimal WASM implementation with only the core
//! financial calculations that don't require database or networking dependencies.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::f64::consts::E;

/// WASM wrapper for European Call Option pricing using Black-Scholes model
#[wasm_bindgen]
pub fn price_european_call(
    underlying_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    volatility: f64,
) -> f64 {
    black_scholes_call(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility)
}

/// WASM wrapper for European Put Option pricing using Black-Scholes model
#[wasm_bindgen]
pub fn price_european_put(
    underlying_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    volatility: f64,
) -> f64 {
    black_scholes_put(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility)
}

/// WASM wrapper for calculating Greeks
#[wasm_bindgen]
pub fn calculate_greeks(
    underlying_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    volatility: f64,
) -> String {
    let result = GreeksResult {
        call_delta: delta_call(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility),
        put_delta: delta_put(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility),
        gamma: gamma(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility),
        vega: vega(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility),
        call_theta: theta_call(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility),
        put_theta: theta_put(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility),
        call_rho: rho_call(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility),
        put_rho: rho_put(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility),
    };
    
    serde_json::to_string(&result).unwrap_or("{}".to_string())
}

/// Result structure for Greeks calculation
#[derive(Serialize, Deserialize)]
struct GreeksResult {
    call_delta: f64,
    put_delta: f64,
    gamma: f64,
    vega: f64,
    call_theta: f64,
    put_theta: f64,
    call_rho: f64,
    put_rho: f64,
}

/// Cumulative distribution function for standard normal distribution
fn cumulative_normal(x: f64) -> f64 {
    (1.0 + libm::erf(x / f64::sqrt(2.0))) / 2.0
}

/// Probability density function for standard normal distribution
fn normal_pdf(x: f64) -> f64 {
    (1.0 / f64::sqrt(2.0 * std::f64::consts::PI)) * f64::exp(-0.5 * x * x)
}

/// Black-Scholes call option pricing formula
fn black_scholes_call(
    s: f64, // underlying price
    k: f64, // strike price
    t: f64, // time to expiry
    r: f64, // risk-free rate
    sigma: f64, // volatility
) -> f64 {
    let d1 = (f64::ln(s / k) + (r + 0.5 * sigma * sigma) * t) / (sigma * f64::sqrt(t));
    let d2 = d1 - sigma * f64::sqrt(t);
    
    s * cumulative_normal(d1) - k * f64::exp(-r * t) * cumulative_normal(d2)
}

/// Black-Scholes put option pricing formula
fn black_scholes_put(
    s: f64, // underlying price
    k: f64, // strike price
    t: f64, // time to expiry
    r: f64, // risk-free rate
    sigma: f64, // volatility
) -> f64 {
    let d1 = (f64::ln(s / k) + (r + 0.5 * sigma * sigma) * t) / (sigma * f64::sqrt(t));
    let d2 = d1 - sigma * f64::sqrt(t);
    
    k * f64::exp(-r * t) * cumulative_normal(-d2) - s * cumulative_normal(-d1)
}

/// Delta for call option
fn delta_call(
    s: f64, // underlying price
    k: f64, // strike price
    t: f64, // time to expiry
    r: f64, // risk-free rate
    sigma: f64, // volatility
) -> f64 {
    let d1 = (f64::ln(s / k) + (r + 0.5 * sigma * sigma) * t) / (sigma * f64::sqrt(t));
    cumulative_normal(d1)
}

/// Delta for put option
fn delta_put(
    s: f64, // underlying price
    k: f64, // strike price
    t: f64, // time to expiry
    r: f64, // risk-free rate
    sigma: f64, // volatility
) -> f64 {
    let d1 = (f64::ln(s / k) + (r + 0.5 * sigma * sigma) * t) / (sigma * f64::sqrt(t));
    cumulative_normal(d1) - 1.0
}

/// Gamma (same for call and put)
fn gamma(
    s: f64, // underlying price
    k: f64, // strike price
    t: f64, // time to expiry
    r: f64, // risk-free rate
    sigma: f64, // volatility
) -> f64 {
    let d1 = (f64::ln(s / k) + (r + 0.5 * sigma * sigma) * t) / (sigma * f64::sqrt(t));
    normal_pdf(d1) / (s * sigma * f64::sqrt(t))
}

/// Vega (same for call and put)
fn vega(
    s: f64, // underlying price
    k: f64, // strike price
    t: f64, // time to expiry
    r: f64, // risk-free rate
    sigma: f64, // volatility
) -> f64 {
    let d1 = (f64::ln(s / k) + (r + 0.5 * sigma * sigma) * t) / (sigma * f64::sqrt(t));
    s * normal_pdf(d1) * f64::sqrt(t)
}

/// Theta for call option
fn theta_call(
    s: f64, // underlying price
    k: f64, // strike price
    t: f64, // time to expiry
    r: f64, // risk-free rate
    sigma: f64, // volatility
) -> f64 {
    let d1 = (f64::ln(s / k) + (r + 0.5 * sigma * sigma) * t) / (sigma * f64::sqrt(t));
    let d2 = d1 - sigma * f64::sqrt(t);
    
    - (s * normal_pdf(d1) * sigma) / (2.0 * f64::sqrt(t)) 
    - r * k * f64::exp(-r * t) * cumulative_normal(d2)
}

/// Theta for put option
fn theta_put(
    s: f64, // underlying price
    k: f64, // strike price
    t: f64, // time to expiry
    r: f64, // risk-free rate
    sigma: f64, // volatility
) -> f64 {
    let d1 = (f64::ln(s / k) + (r + 0.5 * sigma * sigma) * t) / (sigma * f64::sqrt(t));
    let d2 = d1 - sigma * f64::sqrt(t);
    
    - (s * normal_pdf(d1) * sigma) / (2.0 * f64::sqrt(t)) 
    + r * k * f64::exp(-r * t) * cumulative_normal(-d2)
}

/// Rho for call option
fn rho_call(
    s: f64, // underlying price
    k: f64, // strike price
    t: f64, // time to expiry
    r: f64, // risk-free rate
    sigma: f64, // volatility
) -> f64 {
    let d1 = (f64::ln(s / k) + (r + 0.5 * sigma * sigma) * t) / (sigma * f64::sqrt(t));
    let d2 = d1 - sigma * f64::sqrt(t);
    
    k * t * f64::exp(-r * t) * cumulative_normal(d2)
}

/// Rho for put option
fn rho_put(
    s: f64, // underlying price
    k: f64, // strike price
    t: f64, // time to expiry
    r: f64, // risk-free rate
    sigma: f64, // volatility
) -> f64 {
    let d1 = (f64::ln(s / k) + (r + 0.5 * sigma * sigma) * t) / (sigma * f64::sqrt(t));
    let d2 = d1 - sigma * f64::sqrt(t);
    
    -k * t * f64::exp(-r * t) * cumulative_normal(-d2)
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn main() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}