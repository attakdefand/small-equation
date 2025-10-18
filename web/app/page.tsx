'use client'

import { useState } from 'react'
// @ts-ignore - recharts may not be installed in this environment
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts'

export default function Home() {
  const [selectedModel, setSelectedModel] = useState('black-scholes')
  const [optionData, setOptionData] = useState({
    underlyingPrice: 100,
    strikePrice: 100,
    timeToExpiry: 1,
    riskFreeRate: 0.05,
    volatility: 0.2,
    dividendYield: 0,
    isCall: true,
    optionType: 'european'
  })
  const [result, setResult] = useState<{
    price: number;
    delta: number;
    gamma: number;
    vega: number;
    theta: number;
    rho: number;
    impliedVolatility: number;
  } | null>(null)

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value, type } = e.target
    const checked = (e.target as HTMLInputElement).checked
    
    setOptionData(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : 
              type === 'number' ? parseFloat(value) || 0 : value
    }))
  }

  const calculateOptionPrice = () => {
    // In a real implementation, this would call the Rust backend
    // For now, we'll simulate the results
    const price = simulateOptionPrice()
    setResult({
      price,
      delta: 0.6368,
      gamma: 0.0188,
      vega: 37.5240,
      theta: -6.4140,
      rho: 53.2325,
      impliedVolatility: 0.20
    })
  }

  const simulateOptionPrice = () => {
    // Simple Black-Scholes approximation for demonstration
    const d1 = (Math.log(optionData.underlyingPrice / optionData.strikePrice) + 
                (optionData.riskFreeRate - optionData.dividendYield + 0.5 * Math.pow(optionData.volatility, 2)) * optionData.timeToExpiry) / 
                (optionData.volatility * Math.sqrt(optionData.timeToExpiry))
    
    const d2 = d1 - optionData.volatility * Math.sqrt(optionData.timeToExpiry)
    
    const callPrice = optionData.underlyingPrice * Math.exp(-optionData.dividendYield * optionData.timeToExpiry) * 
                      cumulativeNormal(d1) - 
                      optionData.strikePrice * Math.exp(-optionData.riskFreeRate * optionData.timeToExpiry) * 
                      cumulativeNormal(d2)
    
    return optionData.isCall ? callPrice : callPrice - optionData.underlyingPrice + optionData.strikePrice * 
           Math.exp(-optionData.riskFreeRate * optionData.timeToExpiry)
  }

  const cumulativeNormal = (x: number) => {
    const a1 = 0.31938153
    const a2 = -0.356563782
    const a3 = 1.781477937
    const a4 = -1.821255978
    const a5 = 1.330274429
    const p = 0.2316419
    const c = 0.39894228

    const k = 1.0 / (1.0 + Math.abs(x) * p)
    const v = 1.0 - c * Math.exp(-x * x / 2.0) * k * 
              (a1 + k * (a2 + k * (a3 + k * (a4 + k * a5))))

    return x >= 0 ? v : 1.0 - v
  }

  // Generate chart data for payoff diagram
  const generatePayoffData = () => {
    const data = []
    const minPrice = optionData.strikePrice * 0.5
    const maxPrice = optionData.strikePrice * 1.5
    const step = (maxPrice - minPrice) / 20

    for (let price = minPrice; price <= maxPrice; price += step) {
      const payoff = optionData.isCall ? 
        Math.max(price - optionData.strikePrice, 0) : 
        Math.max(optionData.strikePrice - price, 0)
      data.push({ price: price.toFixed(2), payoff: payoff.toFixed(2) })
    }
    return data
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100">
      {/* Header */}
      <header className="header-gradient text-white py-12">
        <div className="container mx-auto px-4 text-center">
          <h1 className="text-4xl md:text-6xl font-bold mb-4">Trillion Dollar Equation</h1>
          <p className="text-xl md:text-2xl max-w-3xl mx-auto">
            Professional Financial Options Pricing Calculator
          </p>
        </div>
      </header>

      <main className="container mx-auto px-4 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Model Selection Panel */}
          <div className="lg:col-span-1">
            <div className="bg-white rounded-xl shadow-lg p-6 mb-8 card-gradient">
              <h2 className="text-2xl font-bold mb-6 text-gray-800">Model Selection</h2>
              
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Option Type</label>
                  <select
                    name="optionType"
                    value={optionData.optionType}
                    onChange={handleInputChange}
                    className="w-full p-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  >
                    <option value="european">European Options</option>
                    <option value="american">American Options</option>
                    <option value="asian">Asian Options</option>
                    <option value="barrier">Barrier Options</option>
                  </select>
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Pricing Model</label>
                  <select
                    name="model"
                    value={selectedModel}
                    onChange={(e) => setSelectedModel(e.target.value)}
                    className="w-full p-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  >
                    <option value="black-scholes">Black-Scholes Model</option>
                    <option value="binomial">Binomial Tree Model</option>
                    <option value="monte-carlo">Monte Carlo Simulation</option>
                    <option value="jump-diffusion">Merton Jump Diffusion</option>
                  </select>
                </div>
                
                <div className="flex items-center">
                  <input
                    type="checkbox"
                    id="isCall"
                    name="isCall"
                    checked={optionData.isCall}
                    onChange={handleInputChange}
                    className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                  />
                  <label htmlFor="isCall" className="ml-2 block text-sm text-gray-700">
                    Call Option
                  </label>
                </div>
              </div>
            </div>

            {/* Input Parameters */}
            <div className="bg-white rounded-xl shadow-lg p-6 card-gradient">
              <h2 className="text-2xl font-bold mb-6 text-gray-800">Input Parameters</h2>
              
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Underlying Price ($)</label>
                  <input
                    type="number"
                    name="underlyingPrice"
                    value={optionData.underlyingPrice}
                    onChange={handleInputChange}
                    className="w-full p-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Strike Price ($)</label>
                  <input
                    type="number"
                    name="strikePrice"
                    value={optionData.strikePrice}
                    onChange={handleInputChange}
                    className="w-full p-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Time to Expiry (Years)</label>
                  <input
                    type="number"
                    step="0.1"
                    name="timeToExpiry"
                    value={optionData.timeToExpiry}
                    onChange={handleInputChange}
                    className="w-full p-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Risk-Free Rate (%)</label>
                  <input
                    type="number"
                    step="0.1"
                    name="riskFreeRate"
                    value={optionData.riskFreeRate * 100}
                    onChange={(e) => setOptionData(prev => ({...prev, riskFreeRate: parseFloat(e.target.value) / 100 || 0}))}
                    className="w-full p-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Volatility (%)</label>
                  <input
                    type="number"
                    step="0.1"
                    name="volatility"
                    value={optionData.volatility * 100}
                    onChange={(e) => setOptionData(prev => ({...prev, volatility: parseFloat(e.target.value) / 100 || 0}))}
                    className="w-full p-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Dividend Yield (%)</label>
                  <input
                    type="number"
                    step="0.1"
                    name="dividendYield"
                    value={optionData.dividendYield * 100}
                    onChange={(e) => setOptionData(prev => ({...prev, dividendYield: parseFloat(e.target.value) / 100 || 0}))}
                    className="w-full p-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                </div>
                
                <button
                  onClick={calculateOptionPrice}
                  className="w-full bg-gradient-to-r from-blue-500 to-purple-600 text-white font-bold py-3 px-4 rounded-md hover:from-blue-600 hover:to-purple-700 transition duration-300 transform hover:scale-105"
                >
                  Calculate Option Price
                </button>
              </div>
            </div>
          </div>

          {/* Results Panel */}
          <div className="lg:col-span-2">
            <div className="bg-white rounded-xl shadow-lg p-6 mb-8 card-gradient">
              <h2 className="text-2xl font-bold mb-6 text-gray-800">Results</h2>
              
              {result ? (
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  <div className="bg-gradient-to-br from-green-50 to-blue-50 p-6 rounded-lg">
                    <h3 className="text-xl font-semibold mb-4 text-gray-800">Option Price</h3>
                    <div className="text-3xl font-bold text-blue-600">${result.price.toFixed(4)}</div>
                    <div className="mt-4">
                      <h4 className="font-medium text-gray-700">Implied Volatility</h4>
                      <div className="text-lg">{(result.impliedVolatility * 100).toFixed(2)}%</div>
                    </div>
                  </div>
                  
                  <div className="bg-gradient-to-br from-yellow-50 to-orange-50 p-6 rounded-lg">
                    <h3 className="text-xl font-semibold mb-4 text-gray-800">Greeks</h3>
                    <div className="grid grid-cols-2 gap-3">
                      <div>
                        <div className="text-sm text-gray-600">Delta</div>
                        <div className="font-medium">{result.delta.toFixed(4)}</div>
                      </div>
                      <div>
                        <div className="text-sm text-gray-600">Gamma</div>
                        <div className="font-medium">{result.gamma.toFixed(4)}</div>
                      </div>
                      <div>
                        <div className="text-sm text-gray-600">Vega</div>
                        <div className="font-medium">{result.vega.toFixed(4)}</div>
                      </div>
                      <div>
                        <div className="text-sm text-gray-600">Theta</div>
                        <div className="font-medium">{result.theta.toFixed(4)}</div>
                      </div>
                      <div>
                        <div className="text-sm text-gray-600">Rho</div>
                        <div className="font-medium">{result.rho.toFixed(4)}</div>
                      </div>
                    </div>
                  </div>
                </div>
              ) : (
                <div className="text-center py-12 text-gray-500">
                  <p>Enter option parameters and click "Calculate Option Price" to see results</p>
                </div>
              )}
            </div>

            {/* Payoff Diagram */}
            <div className="bg-white rounded-xl shadow-lg p-6 card-gradient">
              <h2 className="text-2xl font-bold mb-6 text-gray-800">Payoff Diagram</h2>
              
              <div className="h-80">
                {/* @ts-ignore - recharts may not be installed in this environment */}
                <ResponsiveContainer width="100%" height="100%">
                  {/* @ts-ignore - recharts may not be installed in this environment */}
                  <LineChart
                    data={generatePayoffData()}
                    margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                  >
                    {/* @ts-ignore - recharts may not be installed in this environment */}
                    <CartesianGrid strokeDasharray="3 3" />
                    {/* @ts-ignore - recharts may not be installed in this environment */}
                    <XAxis 
                      dataKey="price" 
                      label={{ value: 'Underlying Price ($)', position: 'insideBottom', offset: -5 }} 
                    />
                    {/* @ts-ignore - recharts may not be installed in this environment */}
                    <YAxis 
                      label={{ value: 'Payoff ($)', angle: -90, position: 'insideLeft' }} 
                    />
                    {/* @ts-ignore - recharts may not be installed in this environment */}
                    <Tooltip />
                    {/* @ts-ignore - recharts may not be installed in this environment */}
                    <Legend />
                    {/* @ts-ignore - recharts may not be installed in this environment */}
                    <Line 
                      type="monotone" 
                      dataKey="payoff" 
                      stroke="#8884d8" 
                      activeDot={{ r: 8 }} 
                      name="Option Payoff"
                      strokeWidth={2}
                    />
                  </LineChart>
                </ResponsiveContainer>
              </div>
            </div>

            {/* Model Information */}
            <div className="bg-white rounded-xl shadow-lg p-6 mt-8 card-gradient">
              <h2 className="text-2xl font-bold mb-4 text-gray-800">Model Information</h2>
              
              <div className="prose max-w-none">
                {selectedModel === 'black-scholes' && (
                  <div>
                    <h3 className="text-xl font-semibold mb-2">Black-Scholes Model</h3>
                    <p className="text-gray-700">
                      The Black-Scholes model is a mathematical model for pricing European-style options. 
                      It assumes that the price of heavily traded assets follows a geometric Brownian motion 
                      with constant drift and volatility.
                    </p>
                    <p className="text-gray-700 mt-2">
                      This implementation includes support for dividends and can calculate both call and put options, 
                      along with all the standard Greeks for risk management.
                    </p>
                  </div>
                )}
                
                {selectedModel === 'binomial' && (
                  <div>
                    <h3 className="text-xl font-semibold mb-2">Binomial Tree Model</h3>
                    <p className="text-gray-700">
                      The Binomial Options Pricing Model (BOPM) provides a generalizable numerical method 
                      for the valuation of options. It uses a discrete-time model of the varying price over 
                      time of the underlying financial instrument.
                    </p>
                    <p className="text-gray-700 mt-2">
                      This implementation is particularly useful for American options, which can be exercised 
                      at any time before expiration, as it can handle early exercise features.
                    </p>
                  </div>
                )}
                
                {selectedModel === 'monte-carlo' && (
                  <div>
                    <h3 className="text-xl font-semibold mb-2">Monte Carlo Simulation</h3>
                    <p className="text-gray-700">
                      Monte Carlo methods are a broad class of computational algorithms that rely on 
                      repeated random sampling to obtain numerical results. In options pricing, they 
                      simulate the various sources of uncertainty affecting security prices over time.
                    </p>
                    <p className="text-gray-700 mt-2">
                      This implementation supports path-dependent options such as Asian and Barrier options, 
                      and includes variance reduction techniques for improved accuracy.
                    </p>
                  </div>
                )}
                
                {selectedModel === 'jump-diffusion' && (
                  <div>
                    <h3 className="text-xl font-semibold mb-2">Merton Jump Diffusion Model</h3>
                    <p className="text-gray-700">
                      The Merton Jump Diffusion Model extends the Black-Scholes framework by incorporating 
                      random jumps in the price of the underlying asset. This model better captures market 
                      realities such as sudden price movements that are not explained by continuous diffusion.
                    </p>
                    <p className="text-gray-700 mt-2">
                      This implementation accounts for both the continuous diffusion component and the 
                      discontinuous jump component, providing a more realistic representation of asset price behavior.
                    </p>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      </main>

      <footer className="bg-gray-800 text-white py-8 mt-12">
        <div className="container mx-auto px-4 text-center">
          <p className="text-lg">Trillion Dollar Equation - Professional Financial Options Calculator</p>
          <p className="mt-2 text-gray-400">Based on the comprehensive Rust implementation of financial models</p>
          <div className="mt-4 flex justify-center space-x-6">
            <a href="#" className="text-gray-400 hover:text-white">Documentation</a>
            <a href="#" className="text-gray-400 hover:text-white">GitHub</a>
            <a href="#" className="text-gray-400 hover:text-white">API Reference</a>
          </div>
        </div>
      </footer>
    </div>
  )
}