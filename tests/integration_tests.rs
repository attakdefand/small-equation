//! Integration tests for the Trillion Dollar Equation project
//!
//! These tests verify the integration between different components
//! including the financial models and database functionality.

use trillion_dollar_equation::models::*;
use trillion_dollar_equation::{
    EuropeanCallOption, EuropeanPutOption, 
    BlacksModel, MertonJumpDiffusion, 
    GarmanKohlhagen, Greeks,
    ImpliedVolatilitySolver,
    AmericanOption, EuropeanOption, BinomialGreeks, BinomialConfig,
    EuropeanMonteCarlo, AsianMonteCarlo, BarrierMonteCarlo, VarianceReducedMonteCarlo, MonteCarloConfig
};
use chrono::Utc;

#[test]
fn test_financial_model_with_database_integration() {
    // Create a European call option
    let call_option = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    // Calculate price and Greeks
    let price = call_option.price();
    let greeks = Greeks::new(100.0, 100.0, 1.0, 0.05, 0.2);
    
    // Create an OptionCalculation record
    let calculation = OptionCalculation {
        id: 0, // Will be set by database
        underlying_price: 100.0,
        strike_price: 100.0,
        time_to_expiry: 1.0,
        risk_free_rate: 0.05,
        volatility: 0.2,
        dividend_yield: 0.0,
        is_call: true,
        option_price: price,
        delta: greeks.delta_call(),
        gamma: greeks.gamma(),
        theta: greeks.theta_call(),
        vega: greeks.vega(),
        rho: greeks.rho_call(),
        timestamp: Utc::now().to_rfc3339(),
    };
    
    // Verify the calculation data
    assert_eq!(calculation.underlying_price, 100.0);
    assert_eq!(calculation.strike_price, 100.0);
    assert_eq!(calculation.time_to_expiry, 1.0);
    assert_eq!(calculation.risk_free_rate, 0.05);
    assert_eq!(calculation.volatility, 0.2);
    assert_eq!(calculation.dividend_yield, 0.0);
    assert_eq!(calculation.is_call, true);
    assert!(calculation.option_price > 0.0);
    assert!(calculation.delta > 0.0 && calculation.delta < 1.0);
    
    // Create market data
    let market_data = MarketData {
        id: 0, // Will be set by database
        symbol: "TEST".to_string(),
        datetime: Utc::now().to_rfc3339(),
        open: 100.0,
        high: 105.0,
        low: 95.0,
        close: 102.0,
        volume: 100000.0,
        implied_volatility: Some(0.25),
    };
    
    // Verify the market data
    assert_eq!(market_data.symbol, "TEST");
    assert_eq!(market_data.open, 100.0);
    assert_eq!(market_data.high, 105.0);
    assert_eq!(market_data.low, 95.0);
    assert_eq!(market_data.close, 102.0);
    assert_eq!(market_data.volume, 100000.0);
    assert_eq!(market_data.implied_volatility, Some(0.25));
}

#[test]
fn test_database_config_creation() {
    // Test default configuration
    let config = DatabaseConfig::default();
    assert_eq!(config.db_type, "sqlite");
    assert_eq!(config.connection_string, "trillion_dollar_equation.db");
    assert_eq!(config.max_connections, 10);
    
    // Test custom configuration
    let custom_config = DatabaseConfig {
        db_type: "postgres".to_string(),
        connection_string: "postgresql://user:pass@localhost/db".to_string(),
        max_connections: 20,
    };
    
    assert_eq!(custom_config.db_type, "postgres");
    assert_eq!(custom_config.connection_string, "postgresql://user:pass@localhost/db");
    assert_eq!(custom_config.max_connections, 20);
}

#[test]
fn test_database_factory_creation() {
    // Test SQLite database creation
    let sqlite_config = DatabaseConfig {
        db_type: "sqlite".to_string(),
        connection_string: "test.db".to_string(),
        max_connections: 5,
    };
    
    let sqlite_db = DatabaseFactory::create_database(&sqlite_config);
    assert!(sqlite_db.is_ok());
    
    // Test PostgreSQL database creation
    let postgres_config = DatabaseConfig {
        db_type: "postgres".to_string(),
        connection_string: "postgresql://user:pass@localhost/db".to_string(),
        max_connections: 5,
    };
    
    let postgres_db = DatabaseFactory::create_database(&postgres_config);
    assert!(postgres_db.is_ok());
    
    // Test unsupported database type
    let unsupported_config = DatabaseConfig {
        db_type: "mysql".to_string(),
        connection_string: "mysql://user:pass@localhost/db".to_string(),
        max_connections: 5,
    };
    
    let unsupported_db = DatabaseFactory::create_database(&unsupported_config);
    assert!(unsupported_db.is_err());
}

#[test]
fn test_complete_workflow_european_options() {
    // Test the complete workflow for European options
    let call_option = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    let put_option = EuropeanPutOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    // Price the options
    let call_price = call_option.price();
    let put_price = put_option.price();
    
    // Calculate Greeks
    let greeks = Greeks::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    let call_delta = greeks.delta_call();
    let put_delta = greeks.delta_put();
    let gamma = greeks.gamma();
    let vega = greeks.vega();
    
    // Calculate implied volatility
    let solver = ImpliedVolatilitySolver::new();
    let implied_vol_call = solver.calculate_call_iv(&call_option, call_price);
    let implied_vol_put = solver.calculate_put_iv(&put_option, put_price);
    
    // Assertions
    assert!(call_price > 0.0);
    assert!(put_price > 0.0);
    assert!(call_delta > 0.0 && call_delta < 1.0);
    assert!(put_delta < 0.0 && put_delta > -1.0);
    assert!(gamma > 0.0);
    assert!(vega > 0.0);
    assert!(implied_vol_call.is_ok());
    assert!(implied_vol_put.is_ok());
    
    // Implied volatilities should match the original volatility
    assert!((implied_vol_call.unwrap() - 0.2).abs() < 1e-6);
    assert!((implied_vol_put.unwrap() - 0.2).abs() < 1e-6);
}

#[test]
fn test_complete_workflow_other_models() {
    // Test other models in the workflow
    
    // Black's model
    let blacks_model = BlacksModel::new(
        100.0,  // Futures price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.2,    // Volatility (20%)
    );
    
    let futures_call_price = blacks_model.call_price();
    let futures_put_price = blacks_model.put_price();
    
    // Merton Jump Diffusion
    let merton_model = MertonJumpDiffusion::new(
        100.0,   // Underlying price
        100.0,   // Strike price
        1.0,     // Time to expiry (1 year)
        0.05,    // Risk-free rate (5%)
        0.2,     // Volatility (20%)
        0.1,     // Jump intensity (10%)
        0.05,    // Jump mean (5%)
        0.1,     // Jump volatility (10%)
    );
    
    let jump_diffusion_price = merton_model.call_price();
    
    // Garman-Kohlhagen
    let fx_model = GarmanKohlhagen::new(
        1.2,    // Spot exchange rate (e.g., EUR/USD)
        1.2,    // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Domestic rate (USD rate)
        0.03,   // Foreign rate (EUR rate)
        0.15,   // Volatility (15%)
    );
    
    let fx_call_price = fx_model.call_price();
    let fx_put_price = fx_model.put_price();
    
    // Assertions
    assert!(futures_call_price > 0.0);
    assert!(futures_put_price > 0.0);
    assert!(jump_diffusion_price > 0.0);
    assert!(fx_call_price > 0.0);
    assert!(fx_put_price > 0.0);
}

#[test]
fn test_binomial_model_integration() {
    // Test integration of binomial model with other components
    
    // Create a European option using the binomial model
    let binomial_european = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create binomial European option");
    
    // Price using binomial model
    let config = BinomialConfig { num_steps: 500 };
    let binomial_price = binomial_european.price(Some(config)).expect("Failed to price with binomial model");
    
    // Create equivalent option using the original EuropeanCallOption
    let original_european = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    // Price using original Black-Scholes model
    let bs_price = original_european.price();
    
    // The prices should be close (binomial converges to Black-Scholes)
    let difference = (binomial_price - bs_price).abs();
    assert!(difference / bs_price < 0.01); // Within 1%
    
    // Test with American option (using a put option which is more likely to have early exercise value)
    let american_option = AmericanOption::new(
        100.0,  // Underlying price
        105.0,  // Strike price (in-the-money put)
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        false,  // Put option
        0.0,    // No dividends
    ).expect("Failed to create American option");
    
    let american_price = american_option.price(Some(config)).expect("Failed to price American option");
    
    // American put should be worth at least as much as European put
    // Create European equivalent
    let european_put = EuropeanOption::new(
        100.0,  // Underlying price
        105.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        false,  // Put option
        0.0,    // No dividends
    ).expect("Failed to create European put option");
    
    let european_price = european_put.price(Some(config)).expect("Failed to price European option");
    
    // American put should be worth at least as much as European put due to early exercise feature
    assert!(american_price >= european_price);
}

#[test]
fn test_monte_carlo_integration() {
    // Test integration of Monte Carlo simulation with other components
    
    // Create a European option using the Monte Carlo model
    let mc_european = EuropeanMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create Monte Carlo European option");
    
    // Price using Monte Carlo simulation
    let config = MonteCarloConfig {
        num_paths: 10_000,  // Reduced for faster testing
        num_steps: 252,     // Trading days in a year
        seed: Some(42),     // Fixed seed for reproducibility
    };
    
    let mc_price = mc_european.price(Some(config)).expect("Failed to price with Monte Carlo");
    
    // Create equivalent option using the original EuropeanCallOption
    let original_european = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    // Price using original Black-Scholes model
    let bs_price = original_european.price();
    
    // The prices should be reasonably close (Monte Carlo converges to Black-Scholes)
    let difference = (mc_price - bs_price).abs();
    assert!(difference / bs_price < 0.1); // Within 10%
}