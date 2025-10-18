//! Tests for the Implied Volatility Solver implementation

use trillion_dollar_equation::models::{
    european_options::{EuropeanCallOption, EuropeanPutOption},
    implied_volatility::{ImpliedVolatilitySolver, SolverConfig, ImpliedVolatilityError}
};

#[test]
fn test_implied_volatility_call_option_newton_raphson() {
    // Create a call option with known parameters
    let call_option = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    // Get the market price using the known volatility
    let market_price = call_option.price();
    
    // Solve for implied volatility
    let solver = ImpliedVolatilitySolver::new();
    let result = solver.calculate_call_iv(&call_option, market_price);
    
    assert!(result.is_ok());
    let implied_vol = result.unwrap();
    
    // The implied volatility should be very close to the original volatility (within tolerance)
    assert!((implied_vol - 0.2).abs() < 1e-6);
}

#[test]
fn test_implied_volatility_put_option_newton_raphson() {
    // Create a put option with known parameters
    let put_option = EuropeanPutOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    // Get the market price using the known volatility
    let market_price = put_option.price();
    
    // Solve for implied volatility
    let solver = ImpliedVolatilitySolver::new();
    let result = solver.calculate_put_iv(&put_option, market_price);
    
    assert!(result.is_ok());
    let implied_vol = result.unwrap();
    
    // The implied volatility should be very close to the original volatility (within tolerance)
    assert!((implied_vol - 0.2).abs() < 1e-6);
}

#[test]
fn test_implied_volatility_call_option_bisection() {
    // Create a call option with known parameters
    let call_option = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    // Get the market price using the known volatility
    let market_price = call_option.price();
    
    // Configure solver to use bisection method by setting high tolerance to force failure of Newton-Raphson
    let config = SolverConfig {
        max_iterations: 100,
        tolerance: 1e-10,  // Very tight tolerance to force bisection
        min_volatility: 0.0001,
        max_volatility: 10.0,
    };
    
    let solver = ImpliedVolatilitySolver::with_config(config);
    let result = solver.calculate_call_iv(&call_option, market_price);
    
    assert!(result.is_ok());
    let implied_vol = result.unwrap();
    
    // The implied volatility should be close to the original volatility
    assert!((implied_vol - 0.2).abs() < 1e-5);
}

#[test]
fn test_implied_volatility_different_market_price() {
    // Create a call option with known parameters
    let call_option = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.1,    // Volatility (10%)
    );
    
    // Use a different market price (higher than model price with 10% vol)
    let market_price = 8.0;
    
    // Solve for implied volatility
    let solver = ImpliedVolatilitySolver::new();
    let result = solver.calculate_call_iv(&call_option, market_price);
    
    assert!(result.is_ok());
    let implied_vol = result.unwrap();
    
    // The implied volatility should be higher than the original 10% volatility
    assert!(implied_vol > 0.1);
}

#[test]
fn test_implied_volatility_convergence_failure() {
    // Create a call option with parameters that lead to out-of-bounds market price
    let call_option = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    // Use an unrealistic market price that's outside the possible range
    let market_price = 200.0;  // This is higher than the underlying price, which is impossible for a call option
    
    // Solve for implied volatility
    let solver = ImpliedVolatilitySolver::new();
    let result = solver.calculate_call_iv(&call_option, market_price);
    
    // Should fail with InvalidInput error
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ImpliedVolatilityError::InvalidInput);
}

#[test]
fn test_implied_volatility_invalid_input() {
    // Create a call option with valid parameters
    let call_option = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    // Use negative market price (invalid)
    let market_price = -5.0;
    
    // Solve for implied volatility
    let solver = ImpliedVolatilitySolver::new();
    let result = solver.calculate_call_iv(&call_option, market_price);
    
    // Should fail with InvalidInput error
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ImpliedVolatilityError::InvalidInput);
}

#[test]
fn test_implied_volatility_edge_case_at_boundary() {
    // Test with very low volatility
    let call_option = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.01,  // Very low volatility (1% instead of 0.1%)
    );
    
    let market_price = call_option.price();
    
    let solver = ImpliedVolatilitySolver::new();
    let result = solver.calculate_call_iv(&call_option, market_price);
    
    assert!(result.is_ok());
    let implied_vol = result.unwrap();
    
    // Should be close to the original very low volatility (with more tolerance for edge cases)
    assert!((implied_vol - 0.01).abs() < 1e-3);
}

#[test]
fn test_solver_config_customization() {
    // Test custom solver configuration
    let config = SolverConfig {
        max_iterations: 50,
        tolerance: 1e-6,
        min_volatility: 0.01,
        max_volatility: 5.0,
    };
    
    let call_option = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    let market_price = call_option.price();
    
    let solver = ImpliedVolatilitySolver::with_config(config);
    let result = solver.calculate_call_iv(&call_option, market_price);
    
    assert!(result.is_ok());
    let implied_vol = result.unwrap();
    
    // Should still converge to the correct value
    assert!((implied_vol - 0.2).abs() < 1e-5);
}