//! Command-line interface for the Black-Scholes calculator

use crate::models::{
    EuropeanCallOption, EuropeanPutOption, 
    BlacksModel, MertonJumpDiffusion, 
    GarmanKohlhagen, Greeks,
    ImpliedVolatilitySolver,
    AmericanOption, EuropeanOption, BinomialGreeks, BinomialConfig,
    EuropeanMonteCarlo, AsianMonteCarlo, BarrierMonteCarlo, VarianceReducedMonteCarlo, MonteCarloConfig,
    DatabaseConfig, DatabaseFactory, OptionCalculation, MarketData
};

/// Run the CLI application
pub fn run() {
    println!("Trillion Dollar Equation - Black-Scholes Model Calculator");
    println!("=====================================================");
    
    // Example calculations
    example_european_options();
    example_blacks_model();
    example_merton_jump_diffusion();
    example_garman_kohlhagen();
    example_greeks();
    example_implied_volatility();
    example_binomial_model();
    example_monte_carlo();
    example_database_integration();
}

fn example_european_options() {
    println!("\n🟢 European Options Pricing");
    println!("---------------------------");
    
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
    
    println!("European Call Option Price: ${:.4}", call_option.price());
    println!("European Put Option Price: ${:.4}", put_option.price());
}

fn example_blacks_model() {
    println!("\n🟡 Black's Model for Futures Options");
    println!("------------------------------------");
    
    let blacks_model = BlacksModel::new(
        100.0,  // Futures price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.2,    // Volatility (20%)
    );
    
    println!("Futures Call Option Price: ${:.4}", blacks_model.call_price());
    println!("Futures Put Option Price: ${:.4}", blacks_model.put_price());
}

fn example_merton_jump_diffusion() {
    println!("\n🟠 Merton Jump Diffusion Model");
    println!("------------------------------");
    
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
    
    println!("Jump Diffusion Call Option Price: ${:.4}", merton_model.call_price());
}

fn example_garman_kohlhagen() {
    println!("\n🔴 Garman-Kohlhagen Model for FX Options");
    println!("----------------------------------------");
    
    let fx_model = GarmanKohlhagen::new(
        1.2,    // Spot exchange rate (e.g., EUR/USD)
        1.2,    // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Domestic rate (USD rate)
        0.03,   // Foreign rate (EUR rate)
        0.15,   // Volatility (15%)
    );
    
    println!("FX Call Option Price: ${:.4}", fx_model.call_price());
    println!("FX Put Option Price: ${:.4}", fx_model.put_price());
}

fn example_greeks() {
    println!("\n⚫ Greeks for Risk Management");
    println!("-----------------------------");
    
    let greeks = Greeks::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    println!("Call Delta: {:.4}", greeks.delta_call());
    println!("Put Delta: {:.4}", greeks.delta_put());
    println!("Gamma: {:.4}", greeks.gamma());
    println!("Vega: {:.4}", greeks.vega());
    println!("Call Theta: {:.4}", greeks.theta_call());
    println!("Put Theta: {:.4}", greeks.theta_put());
    println!("Call Rho: {:.4}", greeks.rho_call());
    println!("Put Rho: {:.4}", greeks.rho_put());
}

fn example_implied_volatility() {
    println!("\n🟣 Implied Volatility Solver");
    println!("----------------------------");
    
    // Create an option with known volatility
    let call_option = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    // Get the market price (in real scenarios, this would come from market data)
    let market_price = call_option.price();
    println!("Market Price (from model): ${:.4}", market_price);
    
    // Now solve for implied volatility
    let solver = ImpliedVolatilitySolver::new();
    match solver.calculate_call_iv(&call_option, market_price) {
        Ok(iv) => println!("Implied Volatility: {:.2}%", iv * 100.0),
        Err(e) => println!("Failed to calculate implied volatility: {:?}", e),
    }
    
    // Example with a different market price
    let different_market_price = 12.0;
    println!("\nWith market price ${:.2}:", different_market_price);
    match solver.calculate_call_iv(&call_option, different_market_price) {
        Ok(iv) => println!("Implied Volatility: {:.2}%", iv * 100.0),
        Err(e) => println!("Failed to calculate implied volatility: {:?}", e),
    }
}

fn example_binomial_model() {
    println!("\n🟤 Binomial Option Pricing Model");
    println!("-------------------------------");
    
    // Example with European call option
    let european_call = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create European call option");
    
    // Price with different numbers of steps
    let config_100 = BinomialConfig { num_steps: 100 };
    let config_1000 = BinomialConfig { num_steps: 1000 };
    
    let price_100 = european_call.price(Some(config_100)).expect("Failed to price option with 100 steps");
    let price_1000 = european_call.price(Some(config_1000)).expect("Failed to price option with 1000 steps");
    let bs_price = european_call.black_scholes_price();
    
    println!("European Call Option Pricing:");
    println!("  Binomial (100 steps): ${:.4}", price_100);
    println!("  Binomial (1000 steps): ${:.4}", price_1000);
    println!("  Black-Scholes: ${:.4}", bs_price);
    println!("  Difference (1000 steps): ${:.6}", (price_1000 - bs_price).abs());
    
    // Example with American put option
    let american_put = AmericanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        false,  // Put option
        0.0,    // No dividends
    ).expect("Failed to create American put option");
    
    let american_price = american_put.price(Some(config_100)).expect("Failed to price American option");
    
    println!("\nAmerican Put Option Pricing:");
    println!("  Binomial (100 steps): ${:.4}", american_price);
    
    // Compare with European put (should be lower for American due to early exercise)
    let european_put = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        false,  // Put option
        0.0,    // No dividends
    ).expect("Failed to create European put option");
    
    let european_price = european_put.price(Some(config_100)).expect("Failed to price European option");
    
    println!("  European equivalent: ${:.4}", european_price);
    println!("  Early exercise value: ${:.4}", (american_price - european_price).max(0.0));
    
    // Example with dividend-paying stock
    let dividend_call = EuropeanOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.02,   // 2% continuous dividend yield
    ).expect("Failed to create dividend-paying call option");
    
    let dividend_price = dividend_call.price(Some(config_100)).expect("Failed to price dividend option");
    
    println!("\nDividend-Paying Call Option:");
    println!("  Binomial (100 steps): ${:.4}", dividend_price);
    
    // Calculate Greeks using binomial model
    let binomial_greeks = BinomialGreeks::new(european_call);
    
    let delta = binomial_greeks.delta(Some(config_100)).expect("Failed to calculate Delta");
    let gamma = binomial_greeks.gamma(Some(config_100)).expect("Failed to calculate Gamma");
    let theta = binomial_greeks.theta(Some(config_100)).expect("Failed to calculate Theta");
    let vega = binomial_greeks.vega(Some(config_100)).expect("Failed to calculate Vega");
    
    println!("\nBinomial Model Greeks:");
    println!("  Delta: {:.4}", delta);
    println!("  Gamma: {:.4}", gamma);
    println!("  Theta: {:.4}", theta);
    println!("  Vega: {:.4}", vega);
}

fn example_monte_carlo() {
    println!("\n⚪ Monte Carlo Simulation");
    println!("-------------------------");
    
    // Example with European call option
    let european_mc = EuropeanMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create European Monte Carlo simulator");
    
    // Configure Monte Carlo simulation with fewer paths for faster execution
    let config = MonteCarloConfig {
        num_paths: 10_000,  // Reduced for faster execution
        num_steps: 252,     // Trading days in a year
        seed: Some(42),     // Fixed seed for reproducibility
    };
    
    let mc_price = european_mc.price(Some(config)).expect("Failed to price with Monte Carlo");
    
    // Compare with Black-Scholes
    let bs_call = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    let bs_price = bs_call.price();
    
    println!("European Call Option Pricing:");
    println!("  Monte Carlo: ${:.4}", mc_price);
    println!("  Black-Scholes: ${:.4}", bs_price);
    println!("  Difference: ${:.4}", (mc_price - bs_price).abs());
    
    // Example with Asian option
    let asian_mc = AsianMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create Asian Monte Carlo simulator");
    
    let asian_price = asian_mc.price(Some(config)).expect("Failed to price Asian option");
    
    println!("\nAsian Call Option Pricing (Arithmetic Average):");
    println!("  Monte Carlo: ${:.4}", asian_price);
    
    // Example with Barrier option (Knock-out call)
    let barrier_mc = BarrierMonteCarlo::new(
        100.0,      // Underlying price
        100.0,      // Strike price
        120.0,      // Barrier price (knock-out at 120)
        1.0,        // Time to expiry (1 year)
        0.05,       // Risk-free rate (5%)
        0.2,        // Volatility (20%)
        true,       // Call option
        false,      // Knock-out (not knock-in)
        0.0,        // No dividends
    ).expect("Failed to create Barrier Monte Carlo simulator");
    
    let barrier_price = barrier_mc.price(Some(config)).expect("Failed to price Barrier option");
    
    println!("\nBarrier Call Option Pricing (Knock-out at 120):");
    println!("  Monte Carlo: ${:.4}", barrier_price);
    
    // Example with Variance Reduced Monte Carlo
    let variance_reduced_mc = VarianceReducedMonteCarlo::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        true,   // Call option
        0.0,    // No dividends
    ).expect("Failed to create Variance Reduced Monte Carlo simulator");
    
    let vr_mc_price = variance_reduced_mc.price(Some(config)).expect("Failed to price with Variance Reduced Monte Carlo");
    
    println!("\nVariance Reduced Monte Carlo:");
    println!("  Standard MC: ${:.4}", mc_price);
    println!("  Variance Reduced MC: ${:.4}", vr_mc_price);
    println!("  Difference: ${:.4}", (vr_mc_price - mc_price).abs());
}

fn example_database_integration() {
    println!("\n💾 Database Integration Example");
    println!("------------------------------");
    
    // Example database configuration
    let config = DatabaseConfig::default();
    println!("Using database: {} at {}", config.db_type, config.connection_string);
    
    // Example option calculation to store
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
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    
    println!("Prepared to store calculation:");
    println!("  Option Price: ${:.4}", calculation.option_price);
    println!("  Delta: {:.4}", calculation.delta);
    println!("  Gamma: {:.4}", calculation.gamma);
    println!("  Vega: {:.4}", calculation.vega);
    
    // Example market data to store
    let market_data = MarketData {
        id: 0, // Will be set by database
        symbol: "AAPL".to_string(),
        datetime: chrono::Utc::now().to_rfc3339(),
        open: 150.0,
        high: 155.0,
        low: 149.0,
        close: 153.0,
        volume: 1000000.0,
        implied_volatility: Some(0.25),
    };
    
    println!("\nPrepared to store market data:");
    println!("  Symbol: {}", market_data.symbol);
    println!("  Close Price: ${:.2}", market_data.close);
    println!("  Implied Volatility: {:.2}%", market_data.implied_volatility.unwrap_or(0.0) * 100.0);
    
    println!("\n📝 Note: Database functionality is implemented but requires actual database setup to run.");
    println!("   See the database module documentation for setup instructions.");
}
