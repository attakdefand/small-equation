//! Binomial Option Pricing Model implementation
//!
//! This module provides functionality to price American and European options
//! using the binomial tree method, which is particularly useful for options
//! that can be exercised before expiry.

use crate::utils::normal_cdf;
use std::f64;

/// Error types for binomial model calculations
#[derive(Debug, PartialEq)]
pub enum BinomialModelError {
    /// Invalid input parameters
    InvalidInput,
    /// Numerical instability
    NumericalError,
}

/// Result type for binomial model calculations
pub type BinomialModelResult<T> = Result<T, BinomialModelError>;

/// Configuration for binomial model
#[derive(Debug, Clone, Copy)]
pub struct BinomialConfig {
    /// Number of time steps in the binomial tree
    pub num_steps: usize,
}

impl Default for BinomialConfig {
    fn default() -> Self {
        Self {
            num_steps: 100,
        }
    }
}

/// Represents an American Option that can be exercised at any time before expiry
#[derive(Debug, Clone)]
pub struct AmericanOption {
    pub underlying_price: f64,
    pub strike_price: f64,
    pub time_to_expiry: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
    pub is_call: bool, // true for call, false for put
    pub dividend_yield: f64, // Continuous dividend yield
}

impl AmericanOption {
    /// Create a new American option
    pub fn new(
        underlying_price: f64,
        strike_price: f64,
        time_to_expiry: f64,
        risk_free_rate: f64,
        volatility: f64,
        is_call: bool,
        dividend_yield: f64,
    ) -> BinomialModelResult<Self> {
        if underlying_price <= 0.0 || strike_price <= 0.0 || time_to_expiry <= 0.0 || volatility <= 0.0 {
            return Err(BinomialModelError::InvalidInput);
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

    /// Price the American option using binomial tree method
    pub fn price(&self, config: Option<BinomialConfig>) -> BinomialModelResult<f64> {
        let steps = config.unwrap_or_default().num_steps;
        
        if steps == 0 {
            return Err(BinomialModelError::InvalidInput);
        }
        
        // Calculate time step
        let dt = self.time_to_expiry / steps as f64;
        
        // Calculate up and down factors
        let up = (self.volatility * dt.sqrt()).exp();
        let down = 1.0 / up;
        
        // Calculate risk-neutral probability
        let discount_factor = (self.risk_free_rate - self.dividend_yield) * dt;
        let prob_up = ((discount_factor.exp() - down) / (up - down)).max(0.0).min(1.0);
        let prob_down = 1.0 - prob_up;
        
        // Calculate discount factor for present value
        let df = (-self.risk_free_rate * dt).exp();
        
        // Initialize option values at maturity
        let mut option_values = vec![0.0; steps + 1];
        
        for i in 0..=steps {
            let spot_price = self.underlying_price * up.powi(i as i32) * down.powi((steps - i) as i32);
            option_values[i] = if self.is_call {
                (spot_price - self.strike_price).max(0.0)
            } else {
                (self.strike_price - spot_price).max(0.0)
            };
        }
        
        // Backward induction
        for step in (0..steps).rev() {
            for i in 0..=step {
                // Calculate expected value
                let expected_value = df * (prob_up * option_values[i + 1] + prob_down * option_values[i]);
                
                // For American options, check for early exercise
                let spot_price = self.underlying_price * up.powi(i as i32) * down.powi((step - i) as i32);
                let exercise_value = if self.is_call {
                    (spot_price - self.strike_price).max(0.0)
                } else {
                    (self.strike_price - spot_price).max(0.0)
                };
                
                // Take the maximum of expected value and exercise value
                option_values[i] = expected_value.max(exercise_value);
            }
        }
        
        Ok(option_values[0])
    }
}

/// Represents a European Option that can only be exercised at expiry
#[derive(Debug, Clone)]
pub struct EuropeanOption {
    pub underlying_price: f64,
    pub strike_price: f64,
    pub time_to_expiry: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
    pub is_call: bool, // true for call, false for put
    pub dividend_yield: f64, // Continuous dividend yield
}

impl EuropeanOption {
    /// Create a new European option
    pub fn new(
        underlying_price: f64,
        strike_price: f64,
        time_to_expiry: f64,
        risk_free_rate: f64,
        volatility: f64,
        is_call: bool,
        dividend_yield: f64,
    ) -> BinomialModelResult<Self> {
        if underlying_price <= 0.0 || strike_price <= 0.0 || time_to_expiry <= 0.0 || volatility <= 0.0 {
            return Err(BinomialModelError::InvalidInput);
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

    /// Price the European option using binomial tree method
    pub fn price(&self, config: Option<BinomialConfig>) -> BinomialModelResult<f64> {
        let steps = config.unwrap_or_default().num_steps;
        
        if steps == 0 {
            return Err(BinomialModelError::InvalidInput);
        }
        
        // Calculate time step
        let dt = self.time_to_expiry / steps as f64;
        
        // Calculate up and down factors
        let up = (self.volatility * dt.sqrt()).exp();
        let down = 1.0 / up;
        
        // Calculate risk-neutral probability
        let discount_factor = (self.risk_free_rate - self.dividend_yield) * dt;
        let prob_up = ((discount_factor.exp() - down) / (up - down)).max(0.0).min(1.0);
        let prob_down = 1.0 - prob_up;
        
        // Calculate discount factor for present value
        let df = (-self.risk_free_rate * dt).exp();
        
        // Initialize option values at maturity
        let mut option_values = vec![0.0; steps + 1];
        
        for i in 0..=steps {
            let spot_price = self.underlying_price * up.powi(i as i32) * down.powi((steps - i) as i32);
            option_values[i] = if self.is_call {
                (spot_price - self.strike_price).max(0.0)
            } else {
                (self.strike_price - spot_price).max(0.0)
            };
        }
        
        // Backward induction (no early exercise for European options)
        for step in (0..steps).rev() {
            for i in 0..=step {
                // Calculate expected value
                let expected_value = df * (prob_up * option_values[i + 1] + prob_down * option_values[i]);
                option_values[i] = expected_value;
            }
        }
        
        Ok(option_values[0])
    }
    
    /// Price the European option using Black-Scholes formula for comparison
    pub fn black_scholes_price(&self) -> f64 {
        let d1 = ((self.underlying_price / self.strike_price).ln() 
            + (self.risk_free_rate - self.dividend_yield + 0.5 * self.volatility.powi(2)) * self.time_to_expiry)
            / (self.volatility * self.time_to_expiry.sqrt());
        let d2 = d1 - self.volatility * self.time_to_expiry.sqrt();
        
        if self.is_call {
            self.underlying_price * (-self.dividend_yield * self.time_to_expiry).exp() * normal_cdf(d1) 
                - self.strike_price * (-self.risk_free_rate * self.time_to_expiry).exp() * normal_cdf(d2)
        } else {
            self.strike_price * (-self.risk_free_rate * self.time_to_expiry).exp() * normal_cdf(-d2)
                - self.underlying_price * (-self.dividend_yield * self.time_to_expiry).exp() * normal_cdf(-d1)
        }
    }
}

/// Greeks calculation for binomial model
pub struct BinomialGreeks {
    pub option: EuropeanOption, // Greeks are typically calculated for European options
}

impl BinomialGreeks {
    /// Create a new BinomialGreeks calculator
    pub fn new(option: EuropeanOption) -> Self {
        Self { option }
    }
    
    /// Calculate Delta (rate of change of option price with respect to underlying price)
    pub fn delta(&self, config: Option<BinomialConfig>) -> BinomialModelResult<f64> {
        let steps = config.unwrap_or_default().num_steps;
        
        // Small change in underlying price
        let ds = self.option.underlying_price * 0.01;
        
        let mut up_option = self.option.clone();
        up_option.underlying_price += ds;
        
        let mut down_option = self.option.clone();
        down_option.underlying_price -= ds;
        
        let up_price = up_option.price(Some(BinomialConfig { num_steps: steps }))?;
        let down_price = down_option.price(Some(BinomialConfig { num_steps: steps }))?;
        
        Ok((up_price - down_price) / (2.0 * ds))
    }
    
    /// Calculate Gamma (rate of change of Delta with respect to underlying price)
    pub fn gamma(&self, config: Option<BinomialConfig>) -> BinomialModelResult<f64> {
        let steps = config.unwrap_or_default().num_steps;
        
        // Small change in underlying price
        let ds = self.option.underlying_price * 0.01;
        
        let mut up_option = self.option.clone();
        up_option.underlying_price += ds;
        
        let mut down_option = self.option.clone();
        down_option.underlying_price -= ds;
        
        let base_price = self.option.price(Some(BinomialConfig { num_steps: steps }))?;
        let up_price = up_option.price(Some(BinomialConfig { num_steps: steps }))?;
        let down_price = down_option.price(Some(BinomialConfig { num_steps: steps }))?;
        
        Ok((up_price - 2.0 * base_price + down_price) / (ds * ds))
    }
    
    /// Calculate Theta (rate of change of option price with respect to time)
    pub fn theta(&self, config: Option<BinomialConfig>) -> BinomialModelResult<f64> {
        let steps = config.unwrap_or_default().num_steps;
        
        // Small change in time
        let dt = self.option.time_to_expiry * 0.01;
        
        let mut short_maturity_option = self.option.clone();
        short_maturity_option.time_to_expiry -= dt;
        
        let base_price = self.option.price(Some(BinomialConfig { num_steps: steps }))?;
        let short_price = short_maturity_option.price(Some(BinomialConfig { num_steps: steps }))?;
        
        Ok(-(short_price - base_price) / dt)
    }
    
    /// Calculate Vega (rate of change of option price with respect to volatility)
    pub fn vega(&self, config: Option<BinomialConfig>) -> BinomialModelResult<f64> {
        let steps = config.unwrap_or_default().num_steps;
        
        // Small change in volatility
        let dv = self.option.volatility * 0.01;
        
        let mut high_vol_option = self.option.clone();
        high_vol_option.volatility += dv;
        
        let mut low_vol_option = self.option.clone();
        low_vol_option.volatility -= dv;
        
        let high_price = high_vol_option.price(Some(BinomialConfig { num_steps: steps }))?;
        let low_price = low_vol_option.price(Some(BinomialConfig { num_steps: steps }))?;
        
        Ok((high_price - low_price) / (2.0 * dv))
    }
}