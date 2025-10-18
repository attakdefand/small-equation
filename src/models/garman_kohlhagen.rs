//! Garman-Kohlhagen model for foreign exchange options
use crate::utils::normal_cdf;

/// Garman-Kohlhagen model for pricing foreign exchange options
#[derive(Debug, Clone)]
pub struct GarmanKohlhagen {
    pub spot_exchange_rate: f64,  // S
    pub strike_price: f64,        // K
    pub time_to_expiry: f64,      // T
    pub domestic_rate: f64,       // r_d (domestic risk-free rate)
    pub foreign_rate: f64,        // r_f (foreign risk-free rate)
    pub volatility: f64,          // σ
}

impl GarmanKohlhagen {
    /// Create a new Garman-Kohlhagen model instance
    pub fn new(
        spot_exchange_rate: f64,
        strike_price: f64,
        time_to_expiry: f64,
        domestic_rate: f64,
        foreign_rate: f64,
        volatility: f64,
    ) -> Self {
        Self {
            spot_exchange_rate,
            strike_price,
            time_to_expiry,
            domestic_rate,
            foreign_rate,
            volatility,
        }
    }

    /// Calculate the price of a call option on a foreign exchange rate
    pub fn call_price(&self) -> f64 {
        let d1 = self.calculate_d1();
        let d2 = self.calculate_d2();
        
        self.spot_exchange_rate * (-self.foreign_rate * self.time_to_expiry).exp() * normal_cdf(d1)
            - self.strike_price * (-self.domestic_rate * self.time_to_expiry).exp() * normal_cdf(d2)
    }

    /// Calculate the price of a put option on a foreign exchange rate
    pub fn put_price(&self) -> f64 {
        let d1 = self.calculate_d1();
        let d2 = self.calculate_d2();
        
        self.strike_price * (-self.domestic_rate * self.time_to_expiry).exp() * normal_cdf(-d2)
            - self.spot_exchange_rate * (-self.foreign_rate * self.time_to_expiry).exp() * normal_cdf(-d1)
    }

    /// Calculate d1 parameter
    fn calculate_d1(&self) -> f64 {
        ((self.spot_exchange_rate / self.strike_price).ln() 
            + (self.domestic_rate - self.foreign_rate + 0.5 * self.volatility.powi(2)) * self.time_to_expiry)
            / (self.volatility * self.time_to_expiry.sqrt())
    }

    /// Calculate d2 parameter
    fn calculate_d2(&self) -> f64 {
        self.calculate_d1() - self.volatility * self.time_to_expiry.sqrt()
    }
}