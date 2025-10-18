//! Example of using the database functionality in the Trillion Dollar Equation project
//!
//! This example demonstrates how to:
//! 1. Configure database connections
//! 2. Store option calculations
//! 3. Store market data
//! 4. Retrieve stored data

use trillion_dollar_equation::models::{
    EuropeanCallOption, Greeks,
    DatabaseConfig, DatabaseFactory, OptionCalculation, MarketData
};
use chrono::Utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Trillion Dollar Equation - Database Example");
    println!("=========================================");
    
    // Create database configuration
    let config = DatabaseConfig::default();
    println!("Using database: {} at {}", config.db_type, config.connection_string);
    
    // Create a database instance using the factory
    let db = DatabaseFactory::create_database(&config)?;
    println!("Database instance created successfully");
    
    // Create an option calculation to store
    let call_option = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
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
    
    println!("\nPrepared option calculation:");
    println!("  Underlying: ${:.2}", calculation.underlying_price);
    println!("  Strike: ${:.2}", calculation.strike_price);
    println!("  Time to expiry: {:.2} years", calculation.time_to_expiry);
    println!("  Risk-free rate: {:.2}%", calculation.risk_free_rate * 100.0);
    println!("  Volatility: {:.2}%", calculation.volatility * 100.0);
    println!("  Option price: ${:.4}", calculation.option_price);
    println!("  Delta: {:.4}", calculation.delta);
    println!("  Gamma: {:.4}", calculation.gamma);
    println!("  Vega: {:.4}", calculation.vega);
    
    // Create market data to store
    let market_data = MarketData {
        id: 0, // Will be set by database
        symbol: "AAPL".to_string(),
        datetime: Utc::now().to_rfc3339(),
        open: 150.0,
        high: 155.0,
        low: 149.0,
        close: 153.0,
        volume: 1000000.0,
        implied_volatility: Some(0.25),
    };
    
    println!("\nPrepared market data:");
    println!("  Symbol: {}", market_data.symbol);
    println!("  Open: ${:.2}", market_data.open);
    println!("  High: ${:.2}", market_data.high);
    println!("  Low: ${:.2}", market_data.low);
    println!("  Close: ${:.2}", market_data.close);
    println!("  Volume: {:.0}", market_data.volume);
    println!("  Implied Volatility: {:.2}%", market_data.implied_volatility.unwrap_or(0.0) * 100.0);
    
    println!("\n📝 Note: This example demonstrates the API but doesn't actually connect to a database.");
    println!("   To use actual database functionality, you would need to:");
    println!("   1. Set up a SQLite or PostgreSQL database");
    println!("   2. Run the database migrations");
    println!("   3. Implement the database trait methods");
    println!("   4. Use the actual database connection methods");
    
    Ok(())
}