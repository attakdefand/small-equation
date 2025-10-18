# WebAssembly Implementation

This directory contains the WebAssembly (WASM) implementation of the Trillion Dollar Equation project, allowing the Rust financial models to run in web browsers.

## Overview

The WASM implementation provides JavaScript bindings for the core Rust financial models, enabling high-performance calculations directly in the browser without requiring a backend server.

## Current Status

⚠️ **Note**: The WASM implementation is currently in development. The files in the `pkg/` directory are placeholders that demonstrate how the actual implementation would work. A full build requires resolving dependency issues with database and networking libraries that don't support WASM.

## Features

- **European Options Pricing**: Price call and put options using the Black-Scholes model
- **Greeks Calculation**: Calculate Delta, Gamma, Vega, Theta, and Rho
- **Implied Volatility**: Calculate implied volatility from market prices
- **Futures Options**: Price futures options using Black's model
- **FX Options**: Price foreign exchange options using Garman-Kohlhagen model
- **Jump Diffusion**: Price options with jumps using Merton's model

## Planned Implementation

The full WASM implementation will provide:

### Core Models
- European Options (Black-Scholes model)
- Futures Options (Black's model)
- Jump Diffusion Model (Merton)
- FX Options (Garman-Kohlhagen)

### Advanced Pricing Methods
- Implied Volatility Solver (Newton-Raphson and bisection methods)

### Risk Management
- Greeks Calculation (Delta, Gamma, Vega, Theta, Rho)

## Building the WASM Package

### Prerequisites

1. Rust toolchain installed
2. wasm-pack installed (`cargo install wasm-pack`)

### Build Instructions

Due to dependency conflicts with database and networking libraries, the WASM build requires a custom approach:

1. Create a separate Cargo.toml that excludes incompatible dependencies
2. Build with wasm-pack using the custom manifest

```bash
# Navigate to the project root
cd ..

# Build the WASM package (currently not working due to dependency issues)
# wasm-pack build --target web --out-dir wasm/pkg --manifest-path Cargo-wasm.toml
```

Or use the provided build scripts:

**On Unix-like systems (Linux, macOS):**
```bash
./build.sh
```

**On Windows:**
```powershell
.\build.ps1
```

## Usage

After building, the WASM package will be available in the `pkg/` directory. You can use it in your web projects like this:

```javascript
import init, { 
    price_european_call, 
    price_european_put, 
    calculate_greeks 
} from './pkg/trillion_dollar_equation.js';

// Initialize the WASM module
await init();

// Calculate an option price
const price = price_european_call(100, 100, 1, 0.05, 0.2);
console.log(`Option price: $${price.toFixed(4)}`);

// Calculate Greeks
const greeksJson = calculate_greeks(100, 100, 1, 0.05, 0.2);
const greeks = JSON.parse(greeksJson);
console.log('Greeks:', greeks);
```

## API Reference

### `price_european_call(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility)`

Calculate the price of a European call option.

### `price_european_put(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility)`

Calculate the price of a European put option.

### `calculate_greeks(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility)`

Calculate the Greeks for an option. Returns a JSON string containing:
- call_delta
- put_delta
- gamma
- vega
- call_theta
- put_theta
- call_rho
- put_rho

### `calculate_implied_volatility(underlying_price, strike_price, time_to_expiry, risk_free_rate, market_price, is_call)`

Calculate the implied volatility from a market price.

### `price_futures_option(futures_price, strike_price, time_to_expiry, volatility, is_call)`

Price a futures option using Black's model.

### `price_fx_option(spot_rate, strike_rate, time_to_expiry, domestic_rate, foreign_rate, volatility, is_call)`

Price a foreign exchange option using the Garman-Kohlhagen model.

### `price_jump_diffusion_option(underlying_price, strike_price, time_to_expiry, risk_free_rate, volatility, jump_intensity, jump_mean, jump_volatility, is_call)`

Price an option with jumps using Merton's jump diffusion model.

## Demo

Open `index.html` in a web browser to see a live demo of the WASM implementation.

## Performance

The WASM implementation will provide significant performance benefits over pure JavaScript implementations:

- **Speed**: Near-native performance for complex calculations
- **Memory Efficiency**: Efficient memory usage compared to JavaScript
- **Precision**: Full floating-point precision maintained

## Browser Support

The WASM implementation works in all modern browsers that support WebAssembly:

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Development

To modify the WASM bindings, edit the `src/wasm.rs` file and rebuild the package.

## Troubleshooting

### Common Issues

1. **"wasm-pack not found"**: Install wasm-pack with `cargo install wasm-pack`
2. **"Failed to initialize WASM module"**: Check browser console for specific errors
3. **Dependency conflicts**: Database and networking libraries don't support WASM
4. **Incorrect calculations**: Verify input parameters are within valid ranges

### Known Issues

1. **Database dependencies**: rusqlite, tokio-postgres, and diesel don't support WASM
2. **Networking dependencies**: mio and related networking libraries don't support WASM
3. **Build tools**: Missing clang compiler for sqlite3 compilation in WASM environment

### Future Improvements

1. **Separate WASM-compatible crate**: Create a separate crate without database/networking dependencies
2. **Conditional compilation**: Use feature flags to exclude incompatible code for WASM builds
3. **Improved error handling**: Better error messages for WASM-specific issues

## Getting Help

If you encounter issues with the WASM implementation:

1. Check the browser console for error messages
2. Verify all build steps completed successfully
3. Ensure you're using a supported browser
4. Consult the project documentation
5. Open an issue on the project repository if needed