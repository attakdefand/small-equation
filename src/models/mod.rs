//! Black-Scholes model implementations and variants
pub mod european_options;
pub mod black_model;
pub mod merton_jump_diffusion;
pub mod garman_kohlhagen;
pub mod greeks;
pub mod implied_volatility;
pub mod binomial_model;
pub mod monte_carlo;
pub mod database;

pub use european_options::{EuropeanCallOption, EuropeanPutOption};
pub use black_model::BlacksModel;
pub use merton_jump_diffusion::MertonJumpDiffusion;
pub use garman_kohlhagen::GarmanKohlhagen;
pub use greeks::Greeks;
pub use implied_volatility::ImpliedVolatilitySolver;
pub use binomial_model::{AmericanOption, EuropeanOption, BinomialGreeks, BinomialConfig};
pub use monte_carlo::{EuropeanMonteCarlo, AsianMonteCarlo, BarrierMonteCarlo, VarianceReducedMonteCarlo, MonteCarloConfig};
pub use database::{Database, DatabaseConfig, OptionCalculation, MarketData, DatabaseFactory};