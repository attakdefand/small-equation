-- Initial schema for the Trillion Dollar Equation database

-- Table for storing option calculation results
CREATE TABLE option_calculations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    underlying_price REAL NOT NULL,
    strike_price REAL NOT NULL,
    time_to_expiry REAL NOT NULL,
    risk_free_rate REAL NOT NULL,
    volatility REAL NOT NULL,
    dividend_yield REAL NOT NULL DEFAULT 0.0,
    is_call BOOLEAN NOT NULL,
    option_price REAL NOT NULL,
    delta REAL NOT NULL,
    gamma REAL NOT NULL,
    theta REAL NOT NULL,
    vega REAL NOT NULL,
    rho REAL NOT NULL,
    timestamp TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Index for faster queries on timestamp
CREATE INDEX idx_option_calculations_timestamp ON option_calculations(timestamp);

-- Table for storing market data
CREATE TABLE market_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    datetime TEXT NOT NULL,
    open REAL NOT NULL,
    high REAL NOT NULL,
    low REAL NOT NULL,
    close REAL NOT NULL,
    volume REAL NOT NULL,
    implied_volatility REAL,
    UNIQUE(symbol, datetime)
);

-- Index for faster queries on symbol and datetime
CREATE INDEX idx_market_data_symbol_datetime ON market_data(symbol, datetime);

-- Table for storing user configurations
CREATE TABLE user_configurations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    config_key TEXT NOT NULL,
    config_value TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, config_key)
);

-- Index for faster queries on user_id
CREATE INDEX idx_user_configurations_user_id ON user_configurations(user_id);