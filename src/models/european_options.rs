//! European Call and Put Options implementation
use crate::utils::normal_cdf;

/// European Call Option pricing using Black-Scholes formula
#[derive(Debug, Clone)]
pub struct EuropeanCallOption {
    pub underlying_price: f64,
    pub strike_price: f64,
    pub time_to_expiry: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
}

impl EuropeanCallOption {
    /// Create a new European Call Option
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

    /// Calculate the price of the European Call Option using Black-Scholes formula
    /// C = S*N(d1) - K*e^(-rT)*N(d2)
    pub fn price(&self) -> f64 {
        let d1 = self.calculate_d1();
        let d2 = self.calculate_d2();
        
        self.underlying_price * normal_cdf(d1) 
            - self.strike_price * (-self.risk_free_rate * self.time_to_expiry).exp() * normal_cdf(d2)
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
}

/// European Put Option pricing using Black-Scholes formula
#[derive(Debug, Clone)]
pub struct EuropeanPutOption {
    pub underlying_price: f64,
    pub strike_price: f64,
    pub time_to_expiry: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
}

impl EuropeanPutOption {
    /// Create a new European Put Option
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

    /// Calculate the price of the European Put Option using Black-Scholes formula
    /// P = K*e^(-rT)*N(-d2) - S*N(-d1)
    pub fn price(&self) -> f64 {
        let d1 = self.calculate_d1();
        let d2 = self.calculate_d2();
        
        self.strike_price * (-self.risk_free_rate * self.time_to_expiry).exp() * normal_cdf(-d2)
            - self.underlying_price * normal_cdf(-d1)
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
}