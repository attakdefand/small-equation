# Trillion Dollar Equation

A comprehensive Rust implementation of the Black-Scholes model and its various extensions for financial option pricing and risk management.

## Overview

This project implements multiple variants of the Black-Scholes model, which is a cornerstone in quantitative finance used for pricing options and other derivatives. The "trillion-dollar equation" refers to the massive financial markets that rely on these mathematical models for pricing and risk management.

## Features

### Main Components

1. **European Options Pricing**
   - European Call Option pricing using the classic Black-Scholes formula
   - European Put Option pricing using the classic Black-Scholes formula

2. **Black's Model**
   - Pricing model for options on futures contracts
   - Commonly used in energy and commodity markets

3. **Merton Jump Diffusion Model**
   - Extension of Black-Scholes that accounts for market shocks and sudden price movements
   - Useful for modeling earnings announcements, crashes, and other discontinuous events

4. **Garman-Kohlhagen Model**
   - Extension for pricing foreign exchange options
   - Accounts for domestic and foreign interest rates

5. **Greeks Calculator**
   - Risk sensitivity measures for options positions
   - Includes Delta, Gamma, Vega, Theta, and Rho calculations

6. **Implied Volatility Solver**
   - Calculate implied volatility from market option prices
   - Uses Newton-Raphson and Bisection methods for numerical solving
   - Essential for traders to infer market sentiment from option prices

7. **Binomial Option Pricing Model**
   - Multi-step binomial trees for American and European option pricing
   - Early exercise capability for American options
   - Dividend adjustments support

### Mathematical Models

| Model | Formula | Use Case |
|-------|---------|----------|
| 🟢 **European Call Option** | C = S·N(d₁) - K·e^(-rT)·N(d₂) | Right to buy stock at strike price, exercisable only at expiry |
| 🔵 **European Put Option** | P = K·e^(-rT)·N(-d₂) - S·N(-d₁) | Right to sell stock at strike price, exercisable only at expiry |
| 🟡 **Black's Model** | Modified Black-Scholes for futures | Used in energy/commodity markets for futures pricing |
| 🟠 **Merton Jump Diffusion** | Black-Scholes with jump terms | Accounts for market shocks and sudden movements |
| 🔴 **Garman-Kohlhagen** | FX-adjusted Black-Scholes | Used in forex options and global currency trading |
| 🟣 **Implied Volatility Solver** | Reverse use: Solve σ from market price | Used by traders to infer market sentiment from option prices |
| 🟤 **Binomial Model** | Multi-step tree pricing | American options with early exercise, dividend-paying stocks |
| ⚫ **Greeks** | Derivatives of BSM | Risk hedging and position management |

## Project Structure

```
src/
├── main.rs          # Entry point for the CLI application
├── lib.rs           # Library exports
├── models/
│   ├── mod.rs              # Module exports
│   ├── european_options.rs # European Call/Put options
│   ├── black_model.rs      # Black's model for futures options
│   ├── merton_jump_diffusion.rs # Jump diffusion model
│   ├── garman_kohlhagen.rs # FX options model
│   ├── greeks.rs           # Risk sensitivity calculations
│   ├── implied_volatility.rs # Implied volatility solver
│   └── binomial_model.rs   # Binomial option pricing model
├── utils/
│   ├── mod.rs              # Utility functions
│   └── math.rs             # Mathematical functions
└── cli/
    └── mod.rs              # Command-line interface

tests/
├── binomial_model_tests.rs # Tests for binomial model
├── implied_volatility_tests.rs # Tests for implied volatility solver
├── integration_tests.rs        # Integration tests for complete workflow
└── library_usage.rs            # Tests for library usage as dependency
```

## Installation

1. Install Rust using [rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository:
   ```bash
   git clone https://github.com/your-username/trillion-dollar-equation.git
   cd trillion-dollar-equation
   ```

3. Build the project:
   ```bash
   cargo build
   ```

## Usage

### As a Library

Add this to your `Cargo.toml`:
```toml
[dependencies]
trillion-dollar-equation = { path = "path/to/trillion-dollar-equation" }
```

Then use it in your code:
```rust
use trillion_dollar_equation::{
    EuropeanCallOption, 
    models::{ImpliedVolatilitySolver, EuropeanOption, AmericanOption}
};

fn main() {
    // Traditional Black-Scholes pricing
    let call_option = EuropeanCallOption::new(
        100.0,  // Underlying price
        100.0,  // Strike price
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
    );
    
    let market_price = call_option.price();
    
    // Implied volatility calculation
    let solver = ImpliedVolatilitySolver::new();
    match solver.calculate_call_iv(&call_option, market_price) {
        Ok(iv) => println!("Implied Volatility: {:.2}%", iv * 100.0),
        Err(e) => println!("Failed to calculate implied volatility: {:?}", e),
    }
    
    // Binomial model pricing for American options
    let american_put = AmericanOption::new(
        100.0,  // Underlying price
        105.0,  // Strike price (in-the-money)
        1.0,    // Time to expiry (1 year)
        0.05,   // Risk-free rate (5%)
        0.2,    // Volatility (20%)
        false,  // Put option
        0.0,    // No dividends
    ).expect("Failed to create American put option");
    
    let config = trillion_dollar_equation::models::BinomialConfig { num_steps: 500 };
    match american_put.price(Some(config)) {
        Ok(price) => println!("American Put Option Price: ${:.4}", price),
        Err(e) => println!("Failed to price American option: {:?}", e),
    }
}
```

### As a CLI Application

Run the example calculations:
```bash
cargo run
```

## Testing

Run all tests:
```bash
cargo test
```

Run specific test suite:
```bash
# Run only binomial model tests
cargo test binomial_model

# Run implied volatility tests
cargo test implied_volatility

# Run integration tests
cargo test integration

# Run library usage tests
cargo test library_usage
```

## Real-World Applications

| Variant | Use Case |
|---------|----------|
| **European Call/Put** | Pricing stock options like $AAPL or $TSLA |
| **Black's Model** | Oil futures options, energy derivatives |
| **Merton Jump Diffusion** | Options during earnings season or war/political risk events |
| **Garman-Kohlhagen** | Options on EUR/USD, JPY/USD in banks or hedge funds |
| **Implied Volatility** | Risk teams estimating how "expensive" options are |
| **Binomial Model** | American options, employee stock options, dividend-paying stocks |
| **Greeks (Δ, Γ, ν, ρ, Θ)** | Algorithmic hedging, delta-neutral portfolios |

## Dependencies

- `num` - Mathematical functions and constants
- `statrs` - Statistical distributions and functions
- `serde` - Serialization support (optional)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## References

1. Black, F., & Scholes, M. (1973). "The Pricing of Options and Corporate Liabilities". Journal of Political Economy.
2. Merton, R. (1976). "Option Pricing when Underlying Stock Returns are Discontinuous". Journal of Financial Economics.
3. Garman, M. B., & Kohlhagen, S. W. (1983). "Foreign Currency Option Values". Journal of International Money and Finance.
4. Cox, J. C., Ross, S. A., & Rubinstein, M. (1979). "Option Pricing: A Simplified Approach". Journal of Financial Economics.# small-equation
