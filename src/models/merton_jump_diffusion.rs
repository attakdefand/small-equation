//! Merton Jump Diffusion model implementation
use crate::models::european_options::EuropeanCallOption;

/// Merton Jump Diffusion model for options pricing with jumps
#[derive(Debug, Clone)]
pub struct MertonJumpDiffusion {
    pub underlying_price: f64,
    pub strike_price: f64,
    pub time_to_expiry: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
    pub jump_intensity: f64,     // Lambda (λ) - expected number of jumps per year
    pub jump_mean: f64,          // Mu_j (μj) - mean of jump size
    pub jump_volatility: f64,    // Sigma_j (σj) - volatility of jump size
}

impl MertonJumpDiffusion {
    /// Create a new Merton Jump Diffusion model instance
    pub fn new(
        underlying_price: f64,
        strike_price: f64,
        time_to_expiry: f64,
        risk_free_rate: f64,
        volatility: f64,
        jump_intensity: f64,
        jump_mean: f64,
        jump_volatility: f64,
    ) -> Self {
        Self {
            underlying_price,
            strike_price,
            time_to_expiry,
            risk_free_rate,
            volatility,
            jump_intensity,
            jump_mean,
            jump_volatility,
        }
    }

    /// Calculate the price of a call option using Merton's Jump Diffusion model
    /// This is a simplified implementation using the first-order approximation
    pub fn call_price(&self) -> f64 {
        // Adjust parameters for jumps
        let gamma = self.jump_mean;  // Mean of jump size
        let sigma_j = self.jump_volatility;  // Volatility of jump size
        let lambda = self.jump_intensity;  // Jump intensity
        
        // Adjusted risk-free rate
        let k = (gamma + 1.0).ln();  // Expected jump size
        let r_adj = self.risk_free_rate - lambda * k + lambda;
        
        // Adjusted volatility
        let sigma_adj = (self.volatility.powi(2) + lambda * sigma_j.powi(2)).sqrt();
        
        // Create an equivalent European option with adjusted parameters
        let equivalent_option = EuropeanCallOption::new(
            self.underlying_price,
            self.strike_price,
            self.time_to_expiry,
            r_adj,
            sigma_adj,
        );
        
        equivalent_option.price()
    }
}