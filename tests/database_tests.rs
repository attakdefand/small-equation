//! Database integration tests
//!
//! These tests verify the database functionality for storing and retrieving
//! financial data including option calculations and market data.

use trillion_dollar_equation::models::database::*;

#[test]
fn test_database_config_default() {
    let config = DatabaseConfig::default();
    assert_eq!(config.db_type, "sqlite");
    assert_eq!(config.connection_string, "trillion_dollar_equation.db");
    assert_eq!(config.max_connections, 10);
}

#[test]
fn test_database_factory_sqlite() {
    let config = DatabaseConfig {
        db_type: "sqlite".to_string(),
        connection_string: "test.db".to_string(),
        max_connections: 5,
    };
    
    let db = DatabaseFactory::create_database(&config);
    assert!(db.is_ok());
}

#[test]
fn test_database_factory_postgres() {
    let config = DatabaseConfig {
        db_type: "postgres".to_string(),
        connection_string: "postgresql://user:pass@localhost/db".to_string(),
        max_connections: 5,
    };
    
    let db = DatabaseFactory::create_database(&config);
    assert!(db.is_ok());
}

#[test]
fn test_database_factory_unsupported() {
    let config = DatabaseConfig {
        db_type: "mysql".to_string(),
        connection_string: "mysql://user:pass@localhost/db".to_string(),
        max_connections: 5,
    };
    
    let db = DatabaseFactory::create_database(&config);
    assert!(db.is_err());
}

#[test]
fn test_option_calculation_struct() {
    let calc = OptionCalculation {
        id: 1,
        underlying_price: 100.0,
        strike_price: 105.0,
        time_to_expiry: 1.0,
        risk_free_rate: 0.05,
        volatility: 0.2,
        dividend_yield: 0.02,
        is_call: true,
        option_price: 8.5,
        delta: 0.6,
        gamma: 0.02,
        theta: -0.03,
        vega: 0.15,
        rho: 0.08,
        timestamp: "2023-01-01T12:00:00Z".to_string(),
    };
    
    assert_eq!(calc.id, 1);
    assert_eq!(calc.underlying_price, 100.0);
    assert_eq!(calc.strike_price, 105.0);
    assert_eq!(calc.time_to_expiry, 1.0);
    assert_eq!(calc.risk_free_rate, 0.05);
    assert_eq!(calc.volatility, 0.2);
    assert_eq!(calc.dividend_yield, 0.02);
    assert_eq!(calc.is_call, true);
    assert_eq!(calc.option_price, 8.5);
    assert_eq!(calc.delta, 0.6);
    assert_eq!(calc.gamma, 0.02);
    assert_eq!(calc.theta, -0.03);
    assert_eq!(calc.vega, 0.15);
    assert_eq!(calc.rho, 0.08);
    assert_eq!(calc.timestamp, "2023-01-01T12:00:00Z");
}

#[test]
fn test_market_data_struct() {
    let data = MarketData {
        id: 1,
        symbol: "AAPL".to_string(),
        datetime: "2023-01-01T12:00:00Z".to_string(),
        open: 150.0,
        high: 155.0,
        low: 149.0,
        close: 153.0,
        volume: 1000000.0,
        implied_volatility: Some(0.25),
    };
    
    assert_eq!(data.id, 1);
    assert_eq!(data.symbol, "AAPL");
    assert_eq!(data.datetime, "2023-01-01T12:00:00Z");
    assert_eq!(data.open, 150.0);
    assert_eq!(data.high, 155.0);
    assert_eq!(data.low, 149.0);
    assert_eq!(data.close, 153.0);
    assert_eq!(data.volume, 1000000.0);
    assert_eq!(data.implied_volatility, Some(0.25));
}