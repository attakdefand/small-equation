//! Black's model implementation for futures/options on forwards
use crate::utils::normal_cdf;

/// Black's model for pricing options on futures
#[derive(Debug, Clone)]
pub struct BlacksModel {
    pub futures_price: f64,
    pub strike_price: f64,
    pub time_to_expiry: f64,
    pub volatility: f64,
}

impl BlacksModel {
    /// Create a new Black's Model instance
    pub fn new(
        futures_price: f64,
        strike_price: f64,
        time_to_expiry: f64,
        volatility: f64,
    ) -> Self {
        Self {
            futures_price,
            strike_price,
            time_to_expiry,
            volatility,
        }
    }

    /// Calculate the price of a call option on a futures contract
    pub fn call_price(&self) -> f64 {
        let d1 = self.calculate_d1();
        let d2 = self.calculate_d2();
        
        self.futures_price * normal_cdf(d1) 
            - self.strike_price * normal_cdf(d2)
    }

    /// Calculate the price of a put option on a futures contract
    pub fn put_price(&self) -> f64 {
        let d1 = self.calculate_d1();
        let d2 = self.calculate_d2();
        
        self.strike_price * normal_cdf(-d2)
            - self.futures_price * normal_cdf(-d1)
    }

    /// Calculate d1 parameter
    fn calculate_d1(&self) -> f64 {
        ((self.futures_price / self.strike_price).ln() 
            + 0.5 * self.volatility.powi(2) * self.time_to_expiry)
            / (self.volatility * self.time_to_expiry.sqrt())
    }

    /// Calculate d2 parameter
    fn calculate_d2(&self) -> f64 {
        self.calculate_d1() - self.volatility * self.time_to_expiry.sqrt()
    }
}