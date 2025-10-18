//! Test to verify that the library can be used as a dependency

use trillion_dollar_equation::{
    EuropeanCallOption, 
    EuropeanPutOption,
    models::{
        ImpliedVolatilitySolver, EuropeanOption, AmericanOption,
        EuropeanMonteCarlo, MonteCarloConfig
    }
};

#[test]
fn test_library_usage() {
    // Test that we can use the public API of our library
    
    // Create a European call option
    let call_option = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    // Price the option
    let price = call_option.price();
    assert!(price > 0.0);
    
    // Calculate implied volatility
    let solver = ImpliedVolatilitySolver::new();
    let iv_result = solver.calculate_call_iv(&call_option, price);
    assert!(iv_result.is_ok());
    
    let implied_vol = iv_result.unwrap();
    // Should be close to the original volatility
    assert!((implied_vol - 0.2).abs() < 1e-6);
    
    // Test with put option as well
    let put_option = EuropeanPutOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    let put_price = put_option.price();
    assert!(put_price > 0.0);
    
    let put_iv_result = solver.calculate_call_iv(&call_option, price);
    assert!(put_iv_result.is_ok());
    
    // Test binomial model usage
    let binomial_european = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create binomial European option");
    
    let binomial_price = binomial_european.price(None).expect("Failed to price with binomial model");
    assert!(binomial_price > 0.0);
    
    // Test American option
    let american_option = AmericanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        false,  // Put option
        0.0,    // No dividends
    ).expect("Failed to create American option");
    
    let american_price = american_option.price(None).expect("Failed to price American option");
    assert!(american_price > 0.0);
    
    // Test Monte Carlo simulation
    let mc_european = EuropeanMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create Monte Carlo European option");
    
    let config = MonteCarloConfig {
        num_paths: 10_000,  // Smaller number for faster testing
        num_steps: 252,
        seed: Some(42),
        num_threads: 1,
    };
    
    let mc_price = mc_european.price(Some(config)).expect("Failed to price with Monte Carlo");
    assert!(mc_price > 0.0);
}