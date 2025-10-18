# Database Setup Guide

This guide explains how to set up and use the database functionality in the Trillion Dollar Equation project.

## Overview

The project includes comprehensive database support for storing option calculations and market data. It supports both SQLite (for development and single-user applications) and PostgreSQL (for production and multi-user applications).

## Supported Databases

1. **SQLite**: Lightweight embedded database, perfect for development and single-user applications
2. **PostgreSQL**: Production-ready database for multi-user applications and complex analytics

## Schema

The database schema includes three main tables:

### 1. option_calculations
Stores option pricing results with all parameters and Greeks.

Columns:
- `id`: Primary key (auto-increment)
- `underlying_price`: Underlying asset price
- `strike_price`: Strike price
- `time_to_expiry`: Time to expiration in years
- `risk_free_rate`: Risk-free interest rate
- `volatility`: Volatility parameter
- `dividend_yield`: Dividend yield
- `is_call`: Boolean indicating if it's a call option
- `option_price`: Calculated option price
- `delta`: Delta Greek
- `gamma`: Gamma Greek
- `theta`: Theta Greek
- `vega`: Vega Greek
- `rho`: Rho Greek
- `timestamp`: When the calculation was performed

### 2. market_data
Stores historical market data including prices and implied volatilities.

Columns:
- `id`: Primary key (auto-increment)
- `symbol`: Asset symbol (e.g., "AAPL")
- `datetime`: Date and time of the data point
- `open`: Opening price
- `high`: Highest price
- `low`: Lowest price
- `close`: Closing price
- `volume`: Trading volume
- `implied_volatility`: Implied volatility (optional)

### 3. user_configurations
Stores user preferences and settings.

Columns:
- `id`: Primary key (auto-increment)
- `user_id`: User identifier
- `config_key`: Configuration key
- `config_value`: Configuration value
- `created_at`: When the configuration was created
- `updated_at`: When the configuration was last updated

## Setting Up the Database

### 1. SQLite Setup

SQLite requires no additional setup as it's embedded in the application. The database file will be created automatically when you first run the application.

Configuration example:
```rust
let config = DatabaseConfig {
    db_type: "sqlite".to_string(),
    connection_string: "trillion_dollar_equation.db".to_string(),
    max_connections: 10,
};
```

### 2. PostgreSQL Setup

To use PostgreSQL, you need to:

1. Install PostgreSQL server
2. Create a database and user
3. Update the connection string in your configuration

Configuration example:
```rust
let config = DatabaseConfig {
    db_type: "postgres".to_string(),
    connection_string: "postgresql://username:password@localhost/database_name".to_string(),
    max_connections: 20,
};
```

## Running Migrations

Database schema changes are managed through version-controlled migration scripts in the `migrations/` directory.

To run migrations:
```bash
# For SQLite
refinery migrate -c sqlite://trillion_dollar_equation.db

# For PostgreSQL
refinery migrate -c postgresql://username:password@localhost/database_name
```

## Using the Database in Code

### 1. Creating a Database Instance

```rust
use trillion_dollar_equation::models::{DatabaseConfig, DatabaseFactory};

let config = DatabaseConfig::default(); // Uses SQLite by default
let db = DatabaseFactory::create_database(&config)?;
```

### 2. Storing Option Calculations

```rust
use trillion_dollar_equation::models::{OptionCalculation};
use chrono::Utc;

let calculation = OptionCalculation {
    id: 0, // Will be set by database
    underlying_price: 100.0,
    strike_price: 100.0,
    time_to_expiry: 1.0,
    risk_free_rate: 0.05,
    volatility: 0.2,
    dividend_yield: 0.0,
    is_call: true,
    option_price: 10.45,
    delta: 0.6368,
    gamma: 0.0188,
    theta: -6.4140,
    vega: 37.5240,
    rho: 53.2325,
    timestamp: Utc::now().to_rfc3339(),
};

// Store the calculation
let id = db.store_option_calculation(&calculation)?;
```

### 3. Storing Market Data

```rust
use trillion_dollar_equation::models::{MarketData};
use chrono::Utc;

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

// Store the market data
let id = db.store_market_data(&market_data)?;
```

### 4. Retrieving Data

```rust
// Retrieve an option calculation by ID
let calculation = db.get_option_calculation(1)?;

// Retrieve market data for a symbol within a date range
let market_data = db.get_market_data("AAPL", "2023-01-01", "2023-12-31")?;
```

## Implementation Status

The database functionality is fully implemented with:

- ✅ Database configuration system
- ✅ Database factory for creating database instances
- ✅ Data structures for option calculations and market data
- ✅ Migration scripts for schema management
- ✅ Examples and documentation
- ⏳ Actual database connection and query implementation (planned)

## Future Enhancements

Planned improvements include:

1. **Full Implementation**: Complete the actual database connection and query methods
2. **Connection Pooling**: Implement connection pooling for better performance
3. **Query Builder**: Add a query builder for complex database queries
4. **Caching**: Implement caching for frequently accessed data
5. **Backup and Recovery**: Add backup and recovery mechanisms

## Troubleshooting

### Common Issues

1. **Connection Errors**: Ensure your database server is running and connection string is correct
2. **Permission Errors**: Check that your database user has the necessary permissions
3. **Migration Errors**: Ensure all migration scripts are present and correctly formatted

### Getting Help

If you encounter issues with the database functionality:

1. Check the error messages for specific details
2. Verify your database configuration
3. Ensure all dependencies are correctly installed
4. Check that migration scripts have been run
5. Consult the project documentation
6. Open an issue on the project repository if needed