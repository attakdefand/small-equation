# Trillion Dollar Equation Web Interface

A professional web-based interface for the Trillion Dollar Equation financial options pricing calculator.

## Features

- Interactive option pricing calculator
- Support for multiple pricing models:
  - Black-Scholes Model
  - Binomial Tree Model
  - Monte Carlo Simulation
  - Merton Jump Diffusion Model
- Real-time payoff diagrams
- Comprehensive Greeks calculations
- Responsive design for all devices

## Tech Stack

- **Frontend**: Next.js 14 (App Router), React, TypeScript
- **Styling**: Tailwind CSS
- **Charts**: Recharts
- **API**: RESTful API (to be implemented)

## Getting Started

### Prerequisites

- Node.js (version 18 or higher)
- npm or yarn

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   ```

2. Navigate to the web directory:
   ```bash
   cd trillion-dollar-equation/web
   ```

3. Install dependencies:
   ```bash
   npm install
   ```

### Development

To run the development server:

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the application.

### Building for Production

To create a production build:

```bash
npm run build
```

To start the production server:

```bash
npm start
```

## Project Structure

```
web/
├── app/                 # Next.js app directory
│   ├── layout.tsx       # Root layout
│   ├── page.tsx         # Home page
│   └── globals.css      # Global styles
├── public/              # Static assets
├── package.json         # Project dependencies
├── tsconfig.json        # TypeScript configuration
└── next.config.js       # Next.js configuration
```

## API Integration

The web interface is designed to connect to a backend API that implements the Rust-based financial models. The API endpoints would include:

- `POST /api/option-price` - Calculate option price using specified model
- `POST /api/implied-volatility` - Calculate implied volatility
- `POST /api/greeks` - Calculate option Greeks
- `POST /api/monte-carlo` - Run Monte Carlo simulation

## Future Enhancements

- Real-time market data integration
- Advanced visualization tools
- User authentication and profiles
- Portfolio analysis features
- Mobile app version

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

- Based on the comprehensive Rust implementation of financial models
- Inspired by the Black-Scholes equation and its various extensions