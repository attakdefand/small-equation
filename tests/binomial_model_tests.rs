//! Tests for the Binomial Option Pricing Model implementation

use trillion_dollar_equation::models::{
    binomial_model::{AmericanOption, EuropeanOption, BinomialGreeks, BinomialConfig, BinomialModelError}
};

#[test]
fn test_american_call_option_creation() {
    let option = AmericanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    );
    
    assert!(option.is_ok());
}

#[test]
fn test_american_put_option_creation() {
    let option = AmericanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        false,  // Put option
        0.0,    // No dividends
    );
    
    assert!(option.is_ok());
}

#[test]
fn test_european_option_creation() {
    let option = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    );
    
    assert!(option.is_ok());
}

#[test]
fn test_invalid_input_parameters() {
    // Test negative underlying price
    let option = AmericanOption::new(
        -100.0, // Invalid negative price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    );
    
    assert!(option.is_err());
    assert_eq!(option.unwrap_err(), BinomialModelError::InvalidInput);
    
    // Test zero time to expiry
    let option = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        0.0,    // Invalid zero time
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    );
    
    assert!(option.is_err());
    assert_eq!(option.unwrap_err(), BinomialModelError::InvalidInput);
    
    // Test negative volatility
    let option = AmericanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        -0.2,   // Invalid negative volatility
        true,   // Call option
        0.0,    // No dividends
    );
    
    assert!(option.is_err());
    assert_eq!(option.unwrap_err(), BinomialModelError::InvalidInput);
}

#[test]
fn test_american_call_option_pricing() {
    let option = AmericanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create American call option");
    
    let price = option.price(None);
    assert!(price.is_ok());
    
    let price_value = price.unwrap();
    assert!(price_value > 0.0);
    
    // Price with specific configuration
    let config = BinomialConfig { num_steps: 500 };
    let price_config = option.price(Some(config));
    assert!(price_config.is_ok());
    
    let price_config_value = price_config.unwrap();
    assert!(price_config_value > 0.0);
}

#[test]
fn test_american_put_option_pricing() {
    let option = AmericanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        false,  // Put option
        0.0,    // No dividends
    ).expect("Failed to create American put option");
    
    let price = option.price(None);
    assert!(price.is_ok());
    
    let price_value = price.unwrap();
    assert!(price_value > 0.0);
}

#[test]
fn test_european_option_pricing() {
    let option = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create European option");
    
    let price = option.price(None);
    assert!(price.is_ok());
    
    let price_value = price.unwrap();
    assert!(price_value > 0.0);
}

#[test]
fn test_european_vs_black_scholes_convergence() {
    let option = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create European option");
    
    // Get Black-Scholes price for comparison
    let bs_price = option.black_scholes_price();
    
    // Test convergence with increasing steps
    let config_100 = BinomialConfig { num_steps: 100 };
    let config_1000 = BinomialConfig { num_steps: 1000 };
    
    let binomial_100 = option.price(Some(config_100)).expect("Failed to price with 100 steps");
    let binomial_1000 = option.price(Some(config_1000)).expect("Failed to price with 1000 steps");
    
    // With more steps, should be closer to Black-Scholes price
    let diff_100 = (binomial_100 - bs_price).abs();
    let diff_1000 = (binomial_1000 - bs_price).abs();
    
    // 1000 steps should be more accurate than 100 steps
    assert!(diff_1000 < diff_100);
    
    // Both should be reasonably close to Black-Scholes (within 1%)
    assert!(diff_1000 / bs_price < 0.01);
}

#[test]
fn test_american_vs_european_pricing() {
    let american_put = AmericanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        false,  // Put option
        0.0,    // No dividends
    ).expect("Failed to create American put option");
    
    let european_put = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        false,  // Put option
        0.0,    // No dividends
    ).expect("Failed to create European put option");
    
    let config = BinomialConfig { num_steps: 500 };
    
    let american_price = american_put.price(Some(config)).expect("Failed to price American option");
    let european_price = european_put.price(Some(config)).expect("Failed to price European option");
    
    // American put should be worth at least as much as European put due to early exercise feature
    assert!(american_price >= european_price);
}

#[test]
fn test_dividend_paying_options() {
    let no_dividend_option = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create no-dividend option");
    
    let dividend_option = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.03,   // 3% continuous dividend yield
    ).expect("Failed to create dividend option");
    
    let config = BinomialConfig { num_steps: 500 };
    
    let no_dividend_price = no_dividend_option.price(Some(config)).expect("Failed to price no-dividend option");
    let dividend_price = dividend_option.price(Some(config)).expect("Failed to price dividend option");
    
    // Call option with dividends should be worth less than without dividends
    assert!(dividend_price < no_dividend_price);
}

#[test]
fn test_binomial_greeks_delta() {
    let option = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create European option");
    
    let greeks = BinomialGreeks::new(option);
    let delta = greeks.delta(None);
    
    assert!(delta.is_ok());
    let delta_value = delta.unwrap();
    
    // Call option delta should be between 0 and 1
    assert!(delta_value >= 0.0 && delta_value <= 1.0);
}

#[test]
fn test_binomial_greeks_gamma() {
    let option = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create European option");
    
    let greeks = BinomialGreeks::new(option);
    let gamma = greeks.gamma(None);
    
    assert!(gamma.is_ok());
    let gamma_value = gamma.unwrap();
    
    // Gamma should be positive for both call and put options
    assert!(gamma_value > 0.0);
}

#[test]
fn test_binomial_greeks_theta() {
    let option = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create European option");
    
    let greeks = BinomialGreeks::new(option);
    let theta = greeks.theta(None);
    
    assert!(theta.is_ok());
    let theta_value = theta.unwrap();
    
    // Theta is typically negative for options (they lose value over time)
    // But for deep in-the-money calls, theta can be positive
    // We'll just check that it's a reasonable value
    assert!(theta_value.abs() < 50.0); // Reasonable bound
}

#[test]
fn test_binomial_greeks_vega() {
    let option = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create European option");
    
    let greeks = BinomialGreeks::new(option);
    let vega = greeks.vega(None);
    
    assert!(vega.is_ok());
    let vega_value = vega.unwrap();
    
    // Vega should be positive (option value increases with volatility)
    assert!(vega_value > 0.0);
}

#[test]
fn test_convergence_with_steps() {
    let option = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create European option");
    
    let config_50 = BinomialConfig { num_steps: 50 };
    let config_100 = BinomialConfig { num_steps: 100 };
    let config_200 = BinomialConfig { num_steps: 200 };
    
    let price_50 = option.price(Some(config_50)).expect("Failed to price with 50 steps");
    let price_100 = option.price(Some(config_100)).expect("Failed to price with 100 steps");
    let price_200 = option.price(Some(config_200)).expect("Failed to price with 200 steps");
    
    // Prices should converge as steps increase
    let diff_50_100 = (price_50 - price_100).abs();
    let diff_100_200 = (price_100 - price_200).abs();
    
    // More steps should lead to smaller differences
    assert!(diff_100_200 < diff_50_100);
}