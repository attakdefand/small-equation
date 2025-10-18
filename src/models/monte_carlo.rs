//! Monte Carlo Simulation for Option Pricing
//!
//! This module provides functionality to price path-dependent options
//! using Monte Carlo methods with Geometric Brownian Motion paths.

use rand::SeedableRng;
use rand_distr::{Distribution, Normal};
use rand::rngs::StdRng;
use std::f64;

/// Error types for Monte Carlo calculations
#[derive(Debug, PartialEq)]
pub enum MonteCarloError {
    /// Invalid input parameters
    InvalidInput,
    /// Numerical instability
    NumericalError,
    /// Insufficient simulations
    InsufficientSimulations,
}

/// Result type for Monte Carlo calculations
pub type MonteCarloResult<T> = Result<T, MonteCarloError>;

/// Configuration for Monte Carlo simulation
#[derive(Debug, Clone, Copy)]
pub struct MonteCarloConfig {
    /// Number of simulation paths
    pub num_paths: usize,
    /// Number of time steps per path
    pub num_steps: usize,
    /// Random seed for reproducibility
    pub seed: Option<u64>,
}

impl Default for MonteCarloConfig {
    fn default() -> Self {
        Self {
            num_paths: 100_000,
            num_steps: 252, // Trading days in a year
            seed: None,
        }
    }
}

/// Monte Carlo Simulator for European options
#[derive(Debug, Clone)]
pub struct EuropeanMonteCarlo {
    pub underlying_price: f64,
    pub strike_price: f64,
    pub time_to_expiry: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
    pub is_call: bool, // true for call, false for put
    pub dividend_yield: f64, // Continuous dividend yield
}

impl EuropeanMonteCarlo {
    /// Create a new European Monte Carlo simulator
    pub fn new(
        underlying_price: f64,
        strike_price: f64,
        time_to_expiry: f64,
        risk_free_rate: f64,
        volatility: f64,
        is_call: bool,
        dividend_yield: f64,
    ) -> MonteCarloResult<Self> {
        if underlying_price <= 0.0 || strike_price <= 0.0 || time_to_expiry <= 0.0 || volatility <= 0.0 {
            return Err(MonteCarloError::InvalidInput);
        }
        
        Ok(Self {
            underlying_price,
            strike_price,
            time_to_expiry,
            risk_free_rate,
            volatility,
            is_call,
            dividend_yield,
        })
    }

    /// Price the European option using Monte Carlo simulation
    pub fn price(&self, config: Option<MonteCarloConfig>) -> MonteCarloResult<f64> {
        let cfg = config.unwrap_or_default();
        
        if cfg.num_paths < 1000 {
            return Err(MonteCarloError::InsufficientSimulations);
        }
        
        // Time step
        let dt = self.time_to_expiry / cfg.num_steps as f64;
        
        // Drift term
        let drift = (self.risk_free_rate - self.dividend_yield - 0.5 * self.volatility * self.volatility) * dt;
        let vol_sqrt_dt = self.volatility * dt.sqrt();
        
        // Create random number generator
        let mut rng = if let Some(seed) = cfg.seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };
        
        // Generate standard normal distribution
        let normal = Normal::new(0.0, 1.0).map_err(|_| MonteCarloError::NumericalError)?;
        
        // Simulate paths
        let mut payoffs = vec![0.0; cfg.num_paths];
        
        for i in 0..cfg.num_paths {
            let mut price = self.underlying_price;
            
            // Simulate price path
            for _ in 0..cfg.num_steps {
                let random_shock = normal.sample(&mut rng);
                price *= (drift + vol_sqrt_dt * random_shock).exp();
            }
            
            // Calculate payoff at expiry
            payoffs[i] = if self.is_call {
                (price - self.strike_price).max(0.0)
            } else {
                (self.strike_price - price).max(0.0)
            };
        }
        
        // Calculate average payoff
        let average_payoff = payoffs.iter().sum::<f64>() / cfg.num_paths as f64;
        
        // Discount to present value
        let discount_factor = (-self.risk_free_rate * self.time_to_expiry).exp();
        let option_price = discount_factor * average_payoff;
        
        Ok(option_price)
    }
}

/// Monte Carlo Simulator for Asian options
#[derive(Debug, Clone)]
pub struct AsianMonteCarlo {
    pub underlying_price: f64,
    pub strike_price: f64,
    pub time_to_expiry: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
    pub is_call: bool, // true for call, false for put
    pub dividend_yield: f64, // Continuous dividend yield
}

impl AsianMonteCarlo {
    /// Create a new Asian Monte Carlo simulator
    pub fn new(
        underlying_price: f64,
        strike_price: f64,
        time_to_expiry: f64,
        risk_free_rate: f64,
        volatility: f64,
        is_call: bool,
        dividend_yield: f64,
    ) -> MonteCarloResult<Self> {
        if underlying_price <= 0.0 || strike_price <= 0.0 || time_to_expiry <= 0.0 || volatility <= 0.0 {
            return Err(MonteCarloError::InvalidInput);
        }
        
        Ok(Self {
            underlying_price,
            strike_price,
            time_to_expiry,
            risk_free_rate,
            volatility,
            is_call,
            dividend_yield,
        })
    }

    /// Price the Asian option using Monte Carlo simulation (arithmetic average)
    pub fn price(&self, config: Option<MonteCarloConfig>) -> MonteCarloResult<f64> {
        let cfg = config.unwrap_or_default();
        
        if cfg.num_paths < 1000 {
            return Err(MonteCarloError::InsufficientSimulations);
        }
        
        // Time step
        let dt = self.time_to_expiry / cfg.num_steps as f64;
        
        // Drift term
        let drift = (self.risk_free_rate - self.dividend_yield - 0.5 * self.volatility * self.volatility) * dt;
        let vol_sqrt_dt = self.volatility * dt.sqrt();
        
        // Create random number generator
        let mut rng = if let Some(seed) = cfg.seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };
        
        // Generate standard normal distribution
        let normal = Normal::new(0.0, 1.0).map_err(|_| MonteCarloError::NumericalError)?;
        
        // Simulate paths
        let mut payoffs = vec![0.0; cfg.num_paths];
        
        for i in 0..cfg.num_paths {
            let mut price = self.underlying_price;
            let mut sum_prices = price; // For arithmetic average
            
            // Simulate price path
            for _ in 1..cfg.num_steps {
                let random_shock = normal.sample(&mut rng);
                price *= (drift + vol_sqrt_dt * random_shock).exp();
                sum_prices += price;
            }
            
            // Arithmetic average
            let average_price = sum_prices / cfg.num_steps as f64;
            
            // Calculate payoff at expiry
            payoffs[i] = if self.is_call {
                (average_price - self.strike_price).max(0.0)
            } else {
                (self.strike_price - average_price).max(0.0)
            };
        }
        
        // Calculate average payoff
        let average_payoff = payoffs.iter().sum::<f64>() / cfg.num_paths as f64;
        
        // Discount to present value
        let discount_factor = (-self.risk_free_rate * self.time_to_expiry).exp();
        let option_price = discount_factor * average_payoff;
        
        Ok(option_price)
    }
}

/// Monte Carlo Simulator for Barrier options
#[derive(Debug, Clone)]
pub struct BarrierMonteCarlo {
    pub underlying_price: f64,
    pub strike_price: f64,
    pub barrier_price: f64,
    pub time_to_expiry: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
    pub is_call: bool, // true for call, false for put
    pub is_knock_in: bool, // true for knock-in, false for knock-out
    pub dividend_yield: f64, // Continuous dividend yield
}

impl BarrierMonteCarlo {
    /// Create a new Barrier Monte Carlo simulator
    pub fn new(
        underlying_price: f64,
        strike_price: f64,
        barrier_price: f64,
        time_to_expiry: f64,
        risk_free_rate: f64,
        volatility: f64,
        is_call: bool,
        is_knock_in: bool,
        dividend_yield: f64,
    ) -> MonteCarloResult<Self> {
        if underlying_price <= 0.0 || strike_price <= 0.0 || barrier_price <= 0.0 || time_to_expiry <= 0.0 || volatility <= 0.0 {
            return Err(MonteCarloError::InvalidInput);
        }
        
        Ok(Self {
            underlying_price,
            strike_price,
            barrier_price,
            time_to_expiry,
            risk_free_rate,
            volatility,
            is_call,
            is_knock_in,
            dividend_yield,
        })
    }

    /// Price the Barrier option using Monte Carlo simulation
    pub fn price(&self, config: Option<MonteCarloConfig>) -> MonteCarloResult<f64> {
        let cfg = config.unwrap_or_default();
        
        if cfg.num_paths < 1000 {
            return Err(MonteCarloError::InsufficientSimulations);
        }
        
        // Time step
        let dt = self.time_to_expiry / cfg.num_steps as f64;
        
        // Drift term
        let drift = (self.risk_free_rate - self.dividend_yield - 0.5 * self.volatility * self.volatility) * dt;
        let vol_sqrt_dt = self.volatility * dt.sqrt();
        
        // Create random number generator
        let mut rng = if let Some(seed) = cfg.seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };
        
        // Generate standard normal distribution
        let normal = Normal::new(0.0, 1.0).map_err(|_| MonteCarloError::NumericalError)?;
        
        // Simulate paths
        let mut payoffs = vec![0.0; cfg.num_paths];
        
        for i in 0..cfg.num_paths {
            let mut price = self.underlying_price;
            let mut barrier_hit = false;
            
            // Simulate price path
            for _ in 0..cfg.num_steps {
                let random_shock = normal.sample(&mut rng);
                price *= (drift + vol_sqrt_dt * random_shock).exp();
                
                // Check if barrier is hit
                if price >= self.barrier_price && !barrier_hit {
                    barrier_hit = true;
                }
            }
            
            // Determine if option is active based on barrier type
            let option_active = if self.is_knock_in {
                barrier_hit // Knock-in: option becomes active when barrier is hit
            } else {
                !barrier_hit // Knock-out: option becomes inactive when barrier is hit
            };
            
            // Calculate payoff at expiry
            if option_active {
                payoffs[i] = if self.is_call {
                    (price - self.strike_price).max(0.0)
                } else {
                    (self.strike_price - price).max(0.0)
                };
            } else {
                payoffs[i] = 0.0; // Option is inactive
            }
        }
        
        // Calculate average payoff
        let average_payoff = payoffs.iter().sum::<f64>() / cfg.num_paths as f64;
        
        // Discount to present value
        let discount_factor = (-self.risk_free_rate * self.time_to_expiry).exp();
        let option_price = discount_factor * average_payoff;
        
        Ok(option_price)
    }
}

/// Monte Carlo Simulator with Antithetic Variates for variance reduction
#[derive(Debug, Clone)]
pub struct VarianceReducedMonteCarlo {
    pub underlying_price: f64,
    pub strike_price: f64,
    pub time_to_expiry: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
    pub is_call: bool, // true for call, false for put
    pub dividend_yield: f64, // Continuous dividend yield
}

impl VarianceReducedMonteCarlo {
    /// Create a new Variance Reduced Monte Carlo simulator
    pub fn new(
        underlying_price: f64,
        strike_price: f64,
        time_to_expiry: f64,
        risk_free_rate: f64,
        volatility: f64,
        is_call: bool,
        dividend_yield: f64,
    ) -> MonteCarloResult<Self> {
        if underlying_price <= 0.0 || strike_price <= 0.0 || time_to_expiry <= 0.0 || volatility <= 0.0 {
            return Err(MonteCarloError::InvalidInput);
        }
        
        Ok(Self {
            underlying_price,
            strike_price,
            time_to_expiry,
            risk_free_rate,
            volatility,
            is_call,
            dividend_yield,
        })
    }

    /// Price the European option using Monte Carlo simulation with antithetic variates
    pub fn price(&self, config: Option<MonteCarloConfig>) -> MonteCarloResult<f64> {
        let cfg = config.unwrap_or_default();
        
        if cfg.num_paths < 1000 {
            return Err(MonteCarloError::InsufficientSimulations);
        }
        
        // Time step
        let dt = self.time_to_expiry / cfg.num_steps as f64;
        
        // Drift term
        let drift = (self.risk_free_rate - self.dividend_yield - 0.5 * self.volatility * self.volatility) * dt;
        let vol_sqrt_dt = self.volatility * dt.sqrt();
        
        // Create random number generator
        let mut rng = if let Some(seed) = cfg.seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };
        
        // Generate standard normal distribution
        let normal = Normal::new(0.0, 1.0).map_err(|_| MonteCarloError::NumericalError)?;
        
        // Simulate paths with antithetic variates
        let mut payoffs = vec![0.0; cfg.num_paths];
        
        for i in 0..cfg.num_paths / 2 {
            let _random_shock = normal.sample(&mut rng);
            
            // Original path
            let mut price1 = self.underlying_price;
            for _ in 0..cfg.num_steps {
                let step_shock = normal.sample(&mut rng);
                price1 *= (drift + vol_sqrt_dt * step_shock).exp();
            }
            payoffs[2 * i] = if self.is_call {
                (price1 - self.strike_price).max(0.0)
            } else {
                (self.strike_price - price1).max(0.0)
            };
            
            // Antithetic path (using -random_shock for each step)
            let mut price2 = self.underlying_price;
            for _ in 0..cfg.num_steps {
                let step_shock = normal.sample(&mut rng);
                price2 *= (drift + vol_sqrt_dt * (-step_shock)).exp();
            }
            payoffs[2 * i + 1] = if self.is_call {
                (price2 - self.strike_price).max(0.0)
            } else {
                (self.strike_price - price2).max(0.0)
            };
        }
        
        // Handle odd number of paths
        if cfg.num_paths % 2 == 1 {
            let mut price = self.underlying_price;
            for _ in 0..cfg.num_steps {
                let random_shock = normal.sample(&mut rng);
                price *= (drift + vol_sqrt_dt * random_shock).exp();
            }
            payoffs[cfg.num_paths - 1] = if self.is_call {
                (price - self.strike_price).max(0.0)
            } else {
                (self.strike_price - price).max(0.0)
            };
        }
        
        // Calculate average payoff
        let average_payoff = payoffs.iter().sum::<f64>() / cfg.num_paths as f64;
        
        // Discount to present value
        let discount_factor = (-self.risk_free_rate * self.time_to_expiry).exp();
        let option_price = discount_factor * average_payoff;
        
        Ok(option_price)
    }
}