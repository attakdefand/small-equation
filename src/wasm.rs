//! WebAssembly bindings for the Trillion Dollar Equation project
//!
//! This module provides WASM bindings for the financial models
//! to enable use in web browsers and other WASM environments.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// Import only the core models that don't have database dependencies
use crate::models::{
    european_options::EuropeanCallOption,
    european_options::EuropeanPutOption,
    black_model::BlacksModel,
    merton_jump_diffusion::MertonJumpDiffusion,
    garman_kohlhagen::GarmanKohlhagen,
    greeks::Greeks,
    implied_volatility::ImpliedVolatilitySolver,
};

/// WASM wrapper for European Call Option pricing
#[wasm_bindgen]
pub fn price_european_call(
    underlying_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    volatility: f64,
) -> f64 {
    let option = EuropeanCallOption::new(
        underlying_price,
        strike_price,
        time_to_expiry,
        risk_free_rate,
        volatility,
    );
    option.price()
}

/// WASM wrapper for European Put Option pricing
#[wasm_bindgen]
pub fn price_european_put(
    underlying_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    volatility: f64,
) -> f64 {
    let option = EuropeanPutOption::new(
        underlying_price,
        strike_price,
        time_to_expiry,
        risk_free_rate,
        volatility,
    );
    option.price()
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
    let greeks = Greeks::new(
        underlying_price,
        strike_price,
        time_to_expiry,
        risk_free_rate,
        volatility,
    );
    
    let result = GreeksResult {
        call_delta: greeks.delta_call(),
        put_delta: greeks.delta_put(),
        gamma: greeks.gamma(),
        vega: greeks.vega(),
        call_theta: greeks.theta_call(),
        put_theta: greeks.theta_put(),
        call_rho: greeks.rho_call(),
        put_rho: greeks.rho_put(),
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

/// WASM wrapper for implied volatility calculation
#[wasm_bindgen]
pub fn calculate_implied_volatility(
    underlying_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    market_price: f64,
    is_call: bool,
) -> f64 {
    let solver = ImpliedVolatilitySolver::new();
    
    if is_call {
        let option = EuropeanCallOption::new(
            underlying_price,
            strike_price,
            time_to_expiry,
            risk_free_rate,
            0.2, // Initial guess for volatility
        );
        solver.calculate_call_iv(&option, market_price).unwrap_or(0.0)
    } else {
        let option = EuropeanPutOption::new(
            underlying_price,
            strike_price,
            time_to_expiry,
            risk_free_rate,
            0.2, // Initial guess for volatility
        );
        solver.calculate_put_iv(&option, market_price).unwrap_or(0.0)
    }
}

/// WASM wrapper for Black's model (futures options)
#[wasm_bindgen]
pub fn price_futures_option(
    futures_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    volatility: f64,
    is_call: bool,
) -> f64 {
    let model = BlacksModel::new(
        futures_price,
        strike_price,
        time_to_expiry,
        volatility,
    );
    
    if is_call {
        model.call_price()
    } else {
        model.put_price()
    }
}

/// WASM wrapper for Garman-Kohlhagen model (FX options)
#[wasm_bindgen]
pub fn price_fx_option(
    spot_rate: f64,
    strike_rate: f64,
    time_to_expiry: f64,
    domestic_rate: f64,
    foreign_rate: f64,
    volatility: f64,
    is_call: bool,
) -> f64 {
    let model = GarmanKohlhagen::new(
        spot_rate,
        strike_rate,
        time_to_expiry,
        domestic_rate,
        foreign_rate,
        volatility,
    );
    
    if is_call {
        model.call_price()
    } else {
        model.put_price()
    }
}

/// WASM wrapper for Merton Jump Diffusion model
#[wasm_bindgen]
pub fn price_jump_diffusion_option(
    underlying_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    volatility: f64,
    jump_intensity: f64,
    jump_mean: f64,
    jump_volatility: f64,
    is_call: bool,
) -> f64 {
    let model = MertonJumpDiffusion::new(
        underlying_price,
        strike_price,
        time_to_expiry,
        risk_free_rate,
        volatility,
        jump_intensity,
        jump_mean,
        jump_volatility,
    );
    
    if is_call {
        model.call_price()
    } else {
        0.0 // For simplicity, we'll only implement call pricing in this example
    }
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