//! Trillion Dollar Equation - Black-Scholes Model Implementations
//!
//! This library provides implementations of various Black-Scholes model variants
//! for pricing financial options and calculating risk metrics.

pub mod models;
pub mod utils;

// Re-export the main types for easier access
pub use models::european_options::{EuropeanCallOption, EuropeanPutOption};
pub use models::black_model::BlacksModel;
pub use models::merton_jump_diffusion::MertonJumpDiffusion;
pub use models::garman_kohlhagen::GarmanKohlhagen;
pub use models::greeks::Greeks;
pub use models::implied_volatility::ImpliedVolatilitySolver;
pub use models::binomial_model::{AmericanOption, EuropeanOption, BinomialGreeks, BinomialConfig};
pub use models::monte_carlo::{EuropeanMonteCarlo, AsianMonteCarlo, BarrierMonteCarlo, VarianceReducedMonteCarlo, MonteCarloConfig};