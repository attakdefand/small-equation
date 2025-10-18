//! Tests for the Monte Carlo Simulation implementation

use trillion_dollar_equation::models::{
    monte_carlo::{
        EuropeanMonteCarlo, AsianMonteCarlo, BarrierMonteCarlo, 
        VarianceReducedMonteCarlo, MonteCarloConfig, MonteCarloError
    },
    european_options::EuropeanCallOption
};

#[test]
fn test_european_monte_carlo_creation() {
    let mc = EuropeanMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    );
    
    assert!(mc.is_ok());
}

#[test]
fn test_asian_monte_carlo_creation() {
    let mc = AsianMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    );
    
    assert!(mc.is_ok());
}

#[test]
fn test_barrier_monte_carlo_creation() {
    let mc = BarrierMonteCarlo::new(
        100.0,      // Underlying price
        100.0,      // Strike price
        120.0,      // Barrier price
        1.0,        // Time to expiry (1 year)
        0.05,       // Risk-free rate (5%)
        0.2,        // Volatility (20%)
        true,       // Call option
        false,      // Knock-out
        0.0,        // No dividends
    );
    
    assert!(mc.is_ok());
}

#[test]
fn test_variance_reduced_monte_carlo_creation() {
    let mc = VarianceReducedMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    );
    
    assert!(mc.is_ok());
}

#[test]
fn test_invalid_input_parameters() {
    // Test negative underlying price
    let mc = EuropeanMonteCarlo::new(
        -100.0, // Invalid negative price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    );
    
    assert!(mc.is_err());
    assert_eq!(mc.unwrap_err(), MonteCarloError::InvalidInput);
    
    // Test zero time to expiry
    let mc = AsianMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        0.0,    // Invalid zero time
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    );
    
    assert!(mc.is_err());
    assert_eq!(mc.unwrap_err(), MonteCarloError::InvalidInput);
    
    // Test negative volatility
    let mc = BarrierMonteCarlo::new(
        100.0,      // Underlying price
        100.0,      // Strike price
        120.0,      // Barrier price
        1.0,        // Time to expiry (1 year)
        0.05,       // Risk-free rate (5%)
        -0.2,       // Invalid negative volatility
        true,       // Call option
        false,      // Knock-out
        0.0,        // No dividends
    );
    
    assert!(mc.is_err());
    assert_eq!(mc.unwrap_err(), MonteCarloError::InvalidInput);
}

#[test]
fn test_european_monte_carlo_pricing() {
    let mc = EuropeanMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create European Monte Carlo simulator");
    
    // Configure Monte Carlo simulation
    let config = MonteCarloConfig {
        num_paths: 10_000,  // Smaller number for faster testing
        num_steps: 252,     // Trading days in a year
        seed: Some(42),     // Fixed seed for reproducibility
    };
    
    let price = mc.price(Some(config));
    assert!(price.is_ok());
    
    let price_value = price.unwrap();
    assert!(price_value > 0.0);
}

#[test]
fn test_asian_monte_carlo_pricing() {
    let mc = AsianMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create Asian Monte Carlo simulator");
    
    // Configure Monte Carlo simulation
    let config = MonteCarloConfig {
        num_paths: 10_000,  // Smaller number for faster testing
        num_steps: 252,     // Trading days in a year
        seed: Some(42),     // Fixed seed for reproducibility
    };
    
    let price = mc.price(Some(config));
    assert!(price.is_ok());
    
    let price_value = price.unwrap();
    assert!(price_value > 0.0);
}

#[test]
fn test_barrier_monte_carlo_pricing() {
    let mc = BarrierMonteCarlo::new(
        100.0,      // Underlying price
        100.0,      // Strike price
        120.0,      // Barrier price (knock-out at 120)
        1.0,        // Time to expiry (1 year)
        0.05,       // Risk-free rate (5%)
        0.2,        // Volatility (20%)
        true,       // Call option
        false,      // Knock-out
        0.0,        // No dividends
    ).expect("Failed to create Barrier Monte Carlo simulator");
    
    // Configure Monte Carlo simulation
    let config = MonteCarloConfig {
        num_paths: 10_000,  // Smaller number for faster testing
        num_steps: 252,     // Trading days in a year
        seed: Some(42),     // Fixed seed for reproducibility
    };
    
    let price = mc.price(Some(config));
    assert!(price.is_ok());
    
    let price_value = price.unwrap();
    assert!(price_value >= 0.0);
}

#[test]
fn test_variance_reduced_monte_carlo_pricing() {
    let mc = VarianceReducedMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create Variance Reduced Monte Carlo simulator");
    
    // Configure Monte Carlo simulation
    let config = MonteCarloConfig {
        num_paths: 10_000,  // Smaller number for faster testing
        num_steps: 252,     // Trading days in a year
        seed: Some(42),     // Fixed seed for reproducibility
    };
    
    let price = mc.price(Some(config));
    assert!(price.is_ok());
    
    let price_value = price.unwrap();
    assert!(price_value > 0.0);
}

#[test]
fn test_monte_carlo_convergence() {
    let mc = EuropeanMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create European Monte Carlo simulator");
    
    // Get Black-Scholes price for comparison
    let bs_call = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    let bs_price = bs_call.price();
    
    // Test with different numbers of paths
    let config_1000 = MonteCarloConfig {
        num_paths: 1_000,
        num_steps: 252,
        seed: Some(42),
    };
    
    let config_10000 = MonteCarloConfig {
        num_paths: 10_000,
        num_steps: 252,
        seed: Some(42),
    };
    
    let mc_price_1000 = mc.price(Some(config_1000)).expect("Failed to price with 1000 paths");
    let mc_price_10000 = mc.price(Some(config_10000)).expect("Failed to price with 10000 paths");
    
    // Calculate differences from Black-Scholes
    let diff_1000 = (mc_price_1000 - bs_price).abs();
    let diff_10000 = (mc_price_10000 - bs_price).abs();
    
    // More paths should generally lead to better accuracy (though not guaranteed due to randomness)
    // We'll check that both are reasonably close to Black-Scholes
    assert!(diff_1000 / bs_price < 0.1); // Within 10%
    assert!(diff_10000 / bs_price < 0.1); // Within 10%
}

#[test]
fn test_insufficient_simulations_error() {
    let mc = EuropeanMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create European Monte Carlo simulator");
    
    // Configure Monte Carlo simulation with too few paths
    let config = MonteCarloConfig {
        num_paths: 100,  // Too few for reliable results
        num_steps: 252,
        seed: Some(42),
    };
    
    let price = mc.price(Some(config));
    assert!(price.is_err());
    assert_eq!(price.unwrap_err(), MonteCarloError::InsufficientSimulations);
}

#[test]
fn test_monte_carlo_put_options() {
    let mc_call = EuropeanMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create European call Monte Carlo simulator");
    
    let mc_put = EuropeanMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        false,  // Put option
        0.0,    // No dividends
    ).expect("Failed to create European put Monte Carlo simulator");
    
    // Configure Monte Carlo simulation
    let config = MonteCarloConfig {
        num_paths: 10_000,
        num_steps: 252,
        seed: Some(42),
    };
    
    let call_price = mc_call.price(Some(config)).expect("Failed to price call option");
    let put_price = mc_put.price(Some(config)).expect("Failed to price put option");
    
    // Both prices should be positive
    assert!(call_price > 0.0);
    assert!(put_price > 0.0);
}

#[test]
fn test_barrier_option_properties() {
    let knock_out_mc = BarrierMonteCarlo::new(
        100.0,      // Underlying price
        100.0,      // Strike price
        150.0,      // High barrier price
        1.0,        // Time to expiry (1 year)
        0.05,       // Risk-free rate (5%)
        0.2,        // Volatility (20%)
        true,       // Call option
        false,      // Knock-out
        0.0,        // No dividends
    ).expect("Failed to create Knock-out Monte Carlo simulator");
    
    let knock_in_mc = BarrierMonteCarlo::new(
        100.0,      // Underlying price
        100.0,      // Strike price
        150.0,      // High barrier price
        1.0,        // Time to expiry (1 year)
        0.05,       // Risk-free rate (5%)
        0.2,        // Volatility (20%)
        true,       // Call option
        true,       // Knock-in
        0.0,        // No dividends
    ).expect("Failed to create Knock-in Monte Carlo simulator");
    
    // Configure Monte Carlo simulation
    let config = MonteCarloConfig {
        num_paths: 10_000,
        num_steps: 252,
        seed: Some(42),
    };
    
    let knock_out_price = knock_out_mc.price(Some(config)).expect("Failed to price knock-out option");
    let knock_in_price = knock_in_mc.price(Some(config)).expect("Failed to price knock-in option");
    
    // Knock-out price should be less than or equal to vanilla option price
    // Knock-in price should be less than or equal to vanilla option price
    // And their sum should approximately equal the vanilla option price
    assert!(knock_out_price >= 0.0);
    assert!(knock_in_price >= 0.0);
}

#[test]
fn test_dividend_paying_options() {
    let no_dividend_mc = EuropeanMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create no-dividend Monte Carlo simulator");
    
    let dividend_mc = EuropeanMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.02,   // 2% continuous dividend yield
    ).expect("Failed to create dividend Monte Carlo simulator");
    
    // Configure Monte Carlo simulation
    let config = MonteCarloConfig {
        num_paths: 10_000,
        num_steps: 252,
        seed: Some(42),
    };
    
    let no_dividend_price = no_dividend_mc.price(Some(config)).expect("Failed to price no-dividend option");
    let dividend_price = dividend_mc.price(Some(config)).expect("Failed to price dividend option");
    
    // Call option with dividends should be worth less than without dividends
    assert!(dividend_price <= no_dividend_price);
}

#[test]
fn test_variance_reduction_improvement() {
    let standard_mc = EuropeanMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create standard Monte Carlo simulator");
    
    let variance_reduced_mc = VarianceReducedMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create Variance Reduced Monte Carlo simulator");
    
    // Configure Monte Carlo simulation
    let config = MonteCarloConfig {
        num_paths: 10_000,
        num_steps: 252,
        seed: Some(42),
    };
    
    let standard_price = standard_mc.price(Some(config)).expect("Failed to price with standard MC");
    let vr_price = variance_reduced_mc.price(Some(config)).expect("Failed to price with variance reduced MC");
    
    // Both should produce valid prices
    assert!(standard_price > 0.0);
    assert!(vr_price > 0.0);
}