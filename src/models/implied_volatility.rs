//! Implied Volatility Solver implementation
//!
//! This module provides functionality to calculate implied volatility
//! from market option prices using numerical methods.

use crate::models::european_options::{EuropeanCallOption, EuropeanPutOption};
use std::f64;

/// Error types for implied volatility calculations
#[derive(Debug, PartialEq)]
pub enum ImpliedVolatilityError {
    /// Convergence failed after maximum iterations
    ConvergenceFailed,
    /// Invalid input parameters
    InvalidInput,
    /// Numerical instability
    NumericalError,
}

/// Result type for implied volatility calculations
pub type ImpliedVolatilityResult = Result<f64, ImpliedVolatilityError>;

/// Configuration for implied volatility solver
#[derive(Debug, Clone, Copy)]
pub struct SolverConfig {
    /// Maximum number of iterations
    pub max_iterations: usize,
    /// Tolerance for convergence
    pub tolerance: f64,
    /// Minimum volatility value
    pub min_volatility: f64,
    /// Maximum volatility value
    pub max_volatility: f64,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-8,
            min_volatility: 0.0001,
            max_volatility: 10.0,
        }
    }
}

/// Implied Volatility Solver using Newton-Raphson and Bisection methods
pub struct ImpliedVolatilitySolver {
    config: SolverConfig,
}

impl ImpliedVolatilitySolver {
    /// Create a new solver with default configuration
    pub fn new() -> Self {
        Self {
            config: SolverConfig::default(),
        }
    }

    /// Create a new solver with custom configuration
    pub fn with_config(config: SolverConfig) -> Self {
        Self { config }
    }

    /// Calculate implied volatility for a European call option using Newton-Raphson method
    /// 
    /// # Arguments
    /// * `option` - European call option with all parameters except volatility
    /// * `market_price` - Observed market price of the option
    /// 
    /// # Returns
    /// * `Ok(volatility)` - Implied volatility if calculation succeeds
    /// * `Err(ImpliedVolatilityError)` - Error if calculation fails
    pub fn calculate_call_iv(
        &self,
        option: &EuropeanCallOption,
        market_price: f64,
    ) -> ImpliedVolatilityResult {
        if market_price <= 0.0 || option.underlying_price <= 0.0 || option.strike_price <= 0.0 {
            return Err(ImpliedVolatilityError::InvalidInput);
        }

        // Try Newton-Raphson first
        match self.newton_raphson_call(option, market_price) {
            Ok(vol) => Ok(vol),
            Err(_) => {
                // Fallback to bisection method
                self.bisection_call(option, market_price)
            }
        }
    }

    /// Calculate implied volatility for a European put option using Newton-Raphson method
    /// 
    /// # Arguments
    /// * `option` - European put option with all parameters except volatility
    /// * `market_price` - Observed market price of the option
    /// 
    /// # Returns
    /// * `Ok(volatility)` - Implied volatility if calculation succeeds
    /// * `Err(ImpliedVolatilityError)` - Error if calculation fails
    pub fn calculate_put_iv(
        &self,
        option: &EuropeanPutOption,
        market_price: f64,
    ) -> ImpliedVolatilityResult {
        if market_price <= 0.0 || option.underlying_price <= 0.0 || option.strike_price <= 0.0 {
            return Err(ImpliedVolatilityError::InvalidInput);
        }

        // Try Newton-Raphson first
        match self.newton_raphson_put(option, market_price) {
            Ok(vol) => Ok(vol),
            Err(_) => {
                // Fallback to bisection method
                self.bisection_put(option, market_price)
            }
        }
    }

    /// Newton-Raphson method for European call option implied volatility
    fn newton_raphson_call(
        &self,
        option: &EuropeanCallOption,
        market_price: f64,
    ) -> ImpliedVolatilityResult {
        let mut volatility = 0.2; // Initial guess

        for _ in 0..self.config.max_iterations {
            // Create option with current volatility guess
            let opt = EuropeanCallOption::new(
                option.underlying_price,
                option.strike_price,
                option.time_to_expiry,
                option.risk_free_rate,
                volatility,
            );

            let price = opt.price();
            let vega = self.calculate_vega_call(&opt);

            // Check for numerical issues
            if vega.abs() < f64::EPSILON {
                return Err(ImpliedVolatilityError::NumericalError);
            }

            // Newton-Raphson update
            let diff = price - market_price;
            let new_vol = volatility - diff / vega;

            // Check for convergence
            if (new_vol - volatility).abs() < self.config.tolerance {
                if new_vol >= self.config.min_volatility && new_vol <= self.config.max_volatility {
                    return Ok(new_vol);
                } else {
                    return Err(ImpliedVolatilityError::InvalidInput);
                }
            }

            volatility = new_vol;

            // Check bounds
            if volatility < self.config.min_volatility {
                volatility = self.config.min_volatility;
            } else if volatility > self.config.max_volatility {
                volatility = self.config.max_volatility;
            }
        }

        Err(ImpliedVolatilityError::ConvergenceFailed)
    }

    /// Newton-Raphson method for European put option implied volatility
    fn newton_raphson_put(
        &self,
        option: &EuropeanPutOption,
        market_price: f64,
    ) -> ImpliedVolatilityResult {
        let mut volatility = 0.2; // Initial guess

        for _ in 0..self.config.max_iterations {
            // Create option with current volatility guess
            let opt = EuropeanPutOption::new(
                option.underlying_price,
                option.strike_price,
                option.time_to_expiry,
                option.risk_free_rate,
                volatility,
            );

            let price = opt.price();
            let vega = self.calculate_vega_put(&opt);

            // Check for numerical issues
            if vega.abs() < f64::EPSILON {
                return Err(ImpliedVolatilityError::NumericalError);
            }

            // Newton-Raphson update
            let diff = price - market_price;
            let new_vol = volatility - diff / vega;

            // Check for convergence
            if (new_vol - volatility).abs() < self.config.tolerance {
                if new_vol >= self.config.min_volatility && new_vol <= self.config.max_volatility {
                    return Ok(new_vol);
                } else {
                    return Err(ImpliedVolatilityError::InvalidInput);
                }
            }

            volatility = new_vol;

            // Check bounds
            if volatility < self.config.min_volatility {
                volatility = self.config.min_volatility;
            } else if volatility > self.config.max_volatility {
                volatility = self.config.max_volatility;
            }
        }

        Err(ImpliedVolatilityError::ConvergenceFailed)
    }

    /// Bisection method for European call option implied volatility
    fn bisection_call(
        &self,
        option: &EuropeanCallOption,
        market_price: f64,
    ) -> ImpliedVolatilityResult {
        let mut low = self.config.min_volatility;
        let mut high = self.config.max_volatility;
        
        // Create options at boundary volatilities
        let opt_low = EuropeanCallOption::new(
            option.underlying_price,
            option.strike_price,
            option.time_to_expiry,
            option.risk_free_rate,
            low,
        );
        
        let opt_high = EuropeanCallOption::new(
            option.underlying_price,
            option.strike_price,
            option.time_to_expiry,
            option.risk_free_rate,
            high,
        );

        let price_low = opt_low.price();
        let price_high = opt_high.price();

        // Check if market price is within bounds
        if market_price < price_low || market_price > price_high {
            return Err(ImpliedVolatilityError::InvalidInput);
        }

        for _ in 0..self.config.max_iterations {
            let mid = (low + high) / 2.0;
            
            let opt_mid = EuropeanCallOption::new(
                option.underlying_price,
                option.strike_price,
                option.time_to_expiry,
                option.risk_free_rate,
                mid,
            );
            
            let price_mid = opt_mid.price();
            let diff = price_mid - market_price;

            if diff.abs() < self.config.tolerance {
                return Ok(mid);
            }

            if diff > 0.0 {
                high = mid;
            } else {
                low = mid;
            }
        }

        Err(ImpliedVolatilityError::ConvergenceFailed)
    }

    /// Bisection method for European put option implied volatility
    fn bisection_put(
        &self,
        option: &EuropeanPutOption,
        market_price: f64,
    ) -> ImpliedVolatilityResult {
        let mut low = self.config.min_volatility;
        let mut high = self.config.max_volatility;
        
        // Create options at boundary volatilities
        let opt_low = EuropeanPutOption::new(
            option.underlying_price,
            option.strike_price,
            option.time_to_expiry,
            option.risk_free_rate,
            low,
        );
        
        let opt_high = EuropeanPutOption::new(
            option.underlying_price,
            option.strike_price,
            option.time_to_expiry,
            option.risk_free_rate,
            high,
        );

        let price_low = opt_low.price();
        let price_high = opt_high.price();

        // Check if market price is within bounds
        if market_price < price_low || market_price > price_high {
            return Err(ImpliedVolatilityError::InvalidInput);
        }

        for _ in 0..self.config.max_iterations {
            let mid = (low + high) / 2.0;
            
            let opt_mid = EuropeanPutOption::new(
                option.underlying_price,
                option.strike_price,
                option.time_to_expiry,
                option.risk_free_rate,
                mid,
            );
            
            let price_mid = opt_mid.price();
            let diff = price_mid - market_price;

            if diff.abs() < self.config.tolerance {
                return Ok(mid);
            }

            if diff > 0.0 {
                high = mid;
            } else {
                low = mid;
            }
        }

        Err(ImpliedVolatilityError::ConvergenceFailed)
    }

    /// Calculate Vega for European call option (derivative of price with respect to volatility)
    fn calculate_vega_call(&self, option: &EuropeanCallOption) -> f64 {
        use crate::utils::normal_pdf;
        
        let d1 = ((option.underlying_price / option.strike_price).ln()
            + (option.risk_free_rate + 0.5 * option.volatility.powi(2)) * option.time_to_expiry)
            / (option.volatility * option.time_to_expiry.sqrt());
        
        option.underlying_price * normal_pdf(d1) * option.time_to_expiry.sqrt()
    }

    /// Calculate Vega for European put option (same as call due to put-call parity)
    fn calculate_vega_put(&self, option: &EuropeanPutOption) -> f64 {
        self.calculate_vega_call(&EuropeanCallOption {
            underlying_price: option.underlying_price,
            strike_price: option.strike_price,
            time_to_expiry: option.time_to_expiry,
            risk_free_rate: option.risk_free_rate,
            volatility: option.volatility,
        })
    }
}