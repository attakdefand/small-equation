# Trillion Dollar Equation Web Interface Demo

This is a static HTML demo of what the web interface for the Trillion Dollar Equation project would look like.

## Features

- Responsive design that works on desktop and mobile devices
- Interactive option pricing calculator interface
- Real-time display of results including option price and Greeks
- Model information section with detailed descriptions
- Professional financial application styling

## How to View

Simply open the `index.html` file in any modern web browser.

## Design Elements

### Color Scheme
- Primary gradient: Purple to blue (#667eea to #764ba2)
- Secondary accents: Pink (#f093fb)
- Clean, professional financial application look

### Layout
- Two-column responsive grid layout
- Card-based design with hover effects
- Clear visual hierarchy for important information

### Components
1. **Header** - Project title and description
2. **Model Selection Panel** - Choose option type and pricing model
3. **Input Parameters Panel** - Enter financial parameters
4. **Results Panel** - Display option price and Greeks
5. **Payoff Diagram** - Visualization area (placeholder in demo)
6. **Model Information** - Detailed model descriptions
7. **Footer** - Project information and links

## In a Full Implementation

In a complete implementation, this interface would:

1. Connect to the Rust-based API backend
2. Perform real calculations using the financial models
3. Display interactive charts for payoff diagrams
4. Support all pricing models (Black-Scholes, Binomial, Monte Carlo, etc.)
5. Include user authentication and profile management
6. Provide calculation history and portfolio analysis

## Technology Stack

This demo uses:
- Pure HTML, CSS, and JavaScript (no frameworks)
- Responsive design with CSS Grid and Flexbox
- Modern CSS features (gradients, shadows, transitions)

## Future Enhancements

- Integration with the Actix-web API backend
- Interactive charting with D3.js or Chart.js
- User authentication and profile management
- Calculation history and portfolio tracking
- Mobile app version
- Advanced visualization tools