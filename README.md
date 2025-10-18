<<<<<<< HEAD
# Trillion Dollar Equation

A comprehensive Rust library for financial options pricing with implementations of the Black-Scholes model and its variants, along with advanced numerical methods for option valuation.

## Features

### Core Models
- **European Options**: Standard Black-Scholes model
- **Futures Options**: Black's model
- **Jump Diffusion**: Merton's model with jumps
- **FX Options**: Garman-Kohlhagen model

### Advanced Pricing Methods
- **Binomial Tree Model**: For American options with early exercise features
- **Monte Carlo Simulation**: For path-dependent options (Asian, Barrier, etc.)
- **Implied Volatility Solver**: Newton-Raphson and bisection methods

### Risk Management
- **Greeks Calculation**: Delta, Gamma, Vega, Theta, Rho
- **Advanced Greeks**: Vanna, Charm, Vomma (planned)

### Data Management
- **Database Integration**: SQLite and PostgreSQL support for storing calculations and market data
- **Data Migration Framework**: Version-controlled schema management
- **ORM Support**: Type-safe database interactions

## Project Structure

```
trillion-dollar-equation/
├── src/                 # Core Rust library
│   ├── models/          # Pricing models implementations
│   ├── utils/           # Utility functions
│   ├── cli/             # Command-line interface
│   ├── lib.rs           # Library entry point
│   └── main.rs          # CLI application
├── tests/               # Comprehensive test suite
├── examples/            # Example applications
├── web/                 # Web frontend (Next.js)
├── api/                 # Web API (Actix-web)
├── migrations/          # Database migration scripts
├── NEXT-IMPLEMENTATION.md    # Implementation roadmap
├── SYSTEM-DESIGN-FOR-EQUATION.MD  # System architecture
├── DATA-MODELING.MD     # Data modeling best practices
├── DATA-MODELING-ALIGNMENT.MD  # Alignment with data modeling principles
├── DATABASE-SETUP.MD    # Database setup and usage guide
├── RUNNING-THE-SYSTEM.md  # Instructions for running the complete system
├── WEB-INTERFACE-ALIGNMENT.MD  # Web interface alignment documentation
└── README.md            # This file
```

## Installation

### Prerequisites
- Rust toolchain (latest stable version)
- Cargo package manager

### Building the Library

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running the CLI

```bash
cargo run
```

## Database Support

The project includes comprehensive database support for storing option calculations and market data:

### Supported Databases
- **SQLite**: Lightweight embedded database for development and single-user applications
- **PostgreSQL**: Production-ready database for multi-user applications and complex analytics

### Schema
The database schema includes:
- `option_calculations`: Stores option pricing results with all parameters and Greeks
- `market_data`: Stores historical market data including prices and implied volatilities
- `user_configurations`: Stores user preferences and settings

### Setup
See [DATABASE-SETUP.MD](DATABASE-SETUP.MD) for detailed instructions on setting up and using the database functionality.

### Migrations
Database schema changes are managed through version-controlled migration scripts in the `migrations/` directory.

## Web Interface

The project includes a professional web-based interface built with Next.js:

### Features
- Interactive option pricing calculator
- Support for multiple pricing models
- Real-time payoff diagrams
- Comprehensive Greeks calculations
- Responsive design for all devices

### Running the Web Interface

1. Navigate to the web directory:
   ```bash
   cd web
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run the development server:
   ```bash
   npm run dev
   ```

4. Open [http://localhost:3000](http://localhost:3000) in your browser

## API

The project includes a RESTful API built with Actix-web that exposes the Rust financial models:

### Running the API

1. Navigate to the API directory:
   ```bash
   cd api
   ```

2. Run the server:
   ```bash
   cargo run
   ```

3. The API will be available at [http://localhost:8080](http://localhost:8080)

### API Endpoints

- `GET /health` - Health check endpoint
- `POST /api/option-price` - Calculate option price using specified model

Example request:
```json
{
  "underlying_price": 100.0,
  "strike_price": 100.0,
  "time_to_expiry": 1.0,
  "risk_free_rate": 0.05,
  "volatility": 0.2,
  "dividend_yield": 0.0,
  "is_call": true,
  "option_type": "european",
  "model": "black-scholes"
}
```

## Models Implementation Status

✅ **Completed Features**:
- European Options (Black-Scholes model)
- Futures Options (Black's model)
- Jump Diffusion Model (Merton)
- FX Options (Garman-Kohlhagen)
- Greeks Calculation
- Implied Volatility Solver
- Binomial Option Pricing Model
- Monte Carlo Simulation
- Database Integration

📅 **Planned Features**:
- Advanced Greeks Calculations
- Exotic Options Models
- Stochastic Volatility Models
- Web API Interface
- Visualization Tools

## Data Modeling Best Practices

This project follows industry best practices for financial modeling as outlined in `DATA-MODELING.MD`:

- **Market IV Usage**: Implied volatility solver calculates market-implied volatility
- **Dividend Adjustments**: All models properly handle dividend yields
- **American Features**: Binomial model supports early exercise
- **FX Carry**: Garman-Kohlhagen model handles domestic/foreign interest rates
- **Futures Modeling**: Black's model for futures options

See `DATA-MODELING-ALIGNMENT.MD` for detailed information on how these principles are implemented.

## Testing

The project includes a comprehensive test suite with:
- Unit tests for each mathematical formula
- Integration tests for end-to-end workflows
- Performance benchmarks
- Edge case validation

Run all tests with:
```bash
cargo test
```

## Examples

The project includes example applications demonstrating various features:

### Database Example
Shows how to use the database functionality:
```bash
cargo run --example database_example
```

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Based on the Black-Scholes equation and its various extensions
- Inspired by financial engineering research and practice
=======
# small-equation
>>>>>>> b03b00542b5daaab505ec5f6ac651f1d7d70cf05
