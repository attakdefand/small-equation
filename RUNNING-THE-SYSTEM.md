# Running the Complete Trillion Dollar Equation System

This document provides instructions for running the complete system including the Rust library, web API, and web interface.

## System Architecture Overview

The complete system consists of three main components:

1. **Rust Library** (`src/`) - Core financial models implementation
2. **Web API** (`api/`) - RESTful API exposing the Rust models
3. **Web Interface** (`web/`) - Next.js frontend for user interaction

```
                    ┌─────────────────┐
                    │   Web Browser   │
                    └─────────────────┘
                             │
                    ┌─────────────────┐
                    │  Web Interface  │
                    │   (Next.js)     │
                    └─────────────────┘
                             │
                    ┌─────────────────┐
                    │    Web API      │
                    │  (Actix-web)    │
                    └─────────────────┘
                             │
                    ┌─────────────────┐
                    │  Rust Library   │
                    │   (Core Models) │
                    └─────────────────┘
```

## Prerequisites

Before running the system, ensure you have the following installed:

- Rust toolchain (latest stable version)
- Cargo package manager
- Node.js (version 18 or higher)
- npm or yarn

## Running the System

### 1. Start the Web API

First, start the Rust-based web API that exposes the financial models:

```bash
cd api
cargo run
```

The API will start on `http://localhost:8080` with the following endpoints:
- `GET /health` - Health check
- `POST /api/option-price` - Option pricing calculations

### 2. Start the Web Interface

In a separate terminal, start the Next.js web interface:

```bash
cd web
npm install  # Only needed for the first time
npm run dev
```

The web interface will be available at `http://localhost:3000`.

### 3. Using the System

1. Open your browser and navigate to `http://localhost:3000`
2. Select an option type and pricing model
3. Enter the required parameters:
   - Underlying Price
   - Strike Price
   - Time to Expiry
   - Risk-Free Rate
   - Volatility
   - Dividend Yield (if applicable)
4. Click "Calculate Option Price"
5. View the results including option price, Greeks, and payoff diagram

## Testing the API

You can test the API directly using curl or a tool like Postman:

```bash
curl -X POST http://localhost:8080/api/option-price \
  -H "Content-Type: application/json" \
  -d '{
    "underlying_price": 100.0,
    "strike_price": 100.0,
    "time_to_expiry": 1.0,
    "risk_free_rate": 0.05,
    "volatility": 0.2,
    "dividend_yield": 0.0,
    "is_call": true,
    "option_type": "european",
    "model": "black-scholes"
  }'
```

Expected response:
```json
{
  "price": 10.450585790229652,
  "delta": 0.6368314634645834,
  "gamma": 0.018761997902065625,
  "vega": 37.52399580413125,
  "theta": -6.414015528296464,
  "rho": 53.23251144762877,
  "implied_volatility": 0.2
}
```

## Development Workflow

### Working on the Rust Library

1. Make changes to the core models in `src/`
2. Run tests to ensure functionality:
   ```bash
   cargo test
   ```
3. The API will automatically use the updated library

### Working on the Web API

1. Make changes to the API in `api/src/main.rs`
2. Run the API:
   ```bash
   cd api
   cargo run
   ```
3. Test endpoints to verify functionality

### Working on the Web Interface

1. Make changes to the frontend in `web/app/`
2. The development server will automatically reload:
   ```bash
   cd web
   npm run dev
   ```
3. View changes in the browser at `http://localhost:3000`

## Building for Production

### Building the Web API

```bash
cd api
cargo build --release
```

The optimized binary will be located at `target/release/trillion-dollar-equation-api`.

### Building the Web Interface

```bash
cd web
npm run build
```

The optimized frontend will be located in the `web/.next/` directory.

## Configuration

### API Configuration

The API can be configured through environment variables:

- `HOST` - API host (default: 127.0.0.1)
- `PORT` - API port (default: 8080)

Example:
```bash
HOST=0.0.0.0 PORT=3001 cargo run
```

### Web Interface Configuration

The web interface can be configured through environment variables:

- `NEXT_PUBLIC_API_URL` - API endpoint (default: http://localhost:8080)

Example:
```bash
NEXT_PUBLIC_API_URL=https://api.example.com npm run dev
```

## Troubleshooting

### Common Issues

1. **Port Conflicts**
   - Error: "Address already in use"
   - Solution: Change the port using environment variables

2. **Dependency Installation Failures**
   - Error: "npm install" fails
   - Solution: Clear npm cache or use a different registry

3. **API Connection Issues**
   - Error: "Failed to fetch" in the web interface
   - Solution: Ensure the API is running and accessible

### Debugging

1. **Enable API Logging**
   ```bash
   RUST_LOG=info cargo run
   ```

2. **Check Web Interface Logs**
   ```bash
   cd web
   npm run dev
   # Check terminal output for errors
   ```

## Performance Considerations

### API Performance

- Monte Carlo simulations are computationally intensive
- Consider limiting the number of paths for web requests
- Use appropriate timeouts for long-running calculations

### Web Interface Performance

- Chart rendering is optimized for up to 100 data points
- Large datasets are automatically sampled for performance
- Lazy loading is used for non-critical components

## Security Considerations

### API Security

- Input validation is performed on all parameters
- CORS is configured for web interface access
- Rate limiting should be implemented in production

### Web Interface Security

- Client-side validation is provided for user experience
- Server-side validation ensures data integrity
- HTTPS should be used in production

## Monitoring and Maintenance

### Health Checks

- API provides `/health` endpoint for monitoring
- Web interface includes status indicators
- Error tracking should be implemented in production

### Updates

- Update dependencies regularly
- Monitor for security vulnerabilities
- Test thoroughly after updates

## Conclusion

The Trillion Dollar Equation system provides a complete solution for financial options pricing with a professional web interface. By following these instructions, you can run the entire system locally and begin exploring the powerful financial models implemented in Rust.