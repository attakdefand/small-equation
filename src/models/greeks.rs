//! Greeks calculations for risk management
use crate::utils::{normal_cdf, normal_pdf};

/// Greeks for measuring risk sensitivities of options
#[derive(Debug, Clone)]
pub struct Greeks {
    pub underlying_price: f64,
    pub strike_price: f64,
    pub time_to_expiry: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
}

impl Greeks {
    /// Create a new Greeks calculator
    pub fn new(
        underlying_price: f64,
        strike_price: f64,
        time_to_expiry: f64,
        risk_free_rate: f64,
        volatility: f64,
    ) -> Self {
        Self {
            underlying_price,
            strike_price,
            time_to_expiry,
            risk_free_rate,
            volatility,
        }
    }

    /// Calculate d1 parameter
    fn calculate_d1(&self) -> f64 {
        ((self.underlying_price / self.strike_price).ln() 
            + (self.risk_free_rate + 0.5 * self.volatility.powi(2)) * self.time_to_expiry)
            / (self.volatility * self.time_to_expiry.sqrt())
    }

    /// Calculate d2 parameter
    fn calculate_d2(&self) -> f64 {
        self.calculate_d1() - self.volatility * self.time_to_expiry.sqrt()
    }

    /// Delta measures the rate of change of the option price with respect to the price of the underlying asset
    pub fn delta_call(&self) -> f64 {
        let d1 = self.calculate_d1();
        normal_cdf(d1)
    }

    /// Delta for put options
    pub fn delta_put(&self) -> f64 {
        let d1 = self.calculate_d1();
        normal_cdf(d1) - 1.0
    }

    /// Gamma measures the rate of change of delta with respect to the price of the underlying asset
    pub fn gamma(&self) -> f64 {
        let d1 = self.calculate_d1();
        normal_pdf(d1) / (self.underlying_price * self.volatility * self.time_to_expiry.sqrt())
    }

    /// Vega measures the rate of change of the option price with respect to the volatility of the underlying asset
    pub fn vega(&self) -> f64 {
        let d1 = self.calculate_d1();
        self.underlying_price * normal_pdf(d1) * self.time_to_expiry.sqrt()
    }

    /// Theta measures the rate of change of the option price with respect to time
    pub fn theta_call(&self) -> f64 {
        let d1 = self.calculate_d1();
        let d2 = self.calculate_d2();
        
        -self.underlying_price * normal_pdf(d1) * self.volatility / (2.0 * self.time_to_expiry.sqrt())
            - self.risk_free_rate * self.strike_price * (-self.risk_free_rate * self.time_to_expiry).exp() * normal_cdf(d2)
    }

    /// Theta for put options
    pub fn theta_put(&self) -> f64 {
        let d1 = self.calculate_d1();
        let d2 = self.calculate_d2();
        
        -self.underlying_price * normal_pdf(d1) * self.volatility / (2.0 * self.time_to_expiry.sqrt())
            + self.risk_free_rate * self.strike_price * (-self.risk_free_rate * self.time_to_expiry).exp() * normal_cdf(-d2)
    }

    /// Rho measures the rate of change of the option price with respect to the interest rate
    pub fn rho_call(&self) -> f64 {
        let d2 = self.calculate_d2();
        self.strike_price * self.time_to_expiry * (-self.risk_free_rate * self.time_to_expiry).exp() * normal_cdf(d2)
    }

    /// Rho for put options
    pub fn rho_put(&self) -> f64 {
        let d2 = self.calculate_d2();
        -self.strike_price * self.time_to_expiry * (-self.risk_free_rate * self.time_to_expiry).exp() * normal_cdf(-d2)
    }
}