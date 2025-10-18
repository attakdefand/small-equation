🧮 1. How Many Types of the Black-Scholes Formula?
While the core formula is the same, the Black-Scholes model has multiple variants or use cases depending on:

| Type / Variant                                     | Formula Change                                | Use Case / Application                                         |
| -------------------------------------------------- | --------------------------------------------- | -------------------------------------------------------------- |
| 🟢 **European Call Option**                        | $C = S N(d_1) - K e^{-rt} N(d_2)$             | Right to **buy** stock at a strike price, only **at expiry**   |
| 🔵 **European Put Option**                         | $P = K e^{-rt} N(-d_2) - S N(-d_1)$           | Right to **sell** stock at a strike price, only **at expiry**  |
| 🟡 **Black's Model**                               | Modification for futures/options on forwards  | Used in energy/commodity markets for futures pricing           |
| 🟠 **Merton Jump Diffusion**                       | Adds jump terms to account for market shocks  | Used in real markets with sudden movements (crashes, earnings) |
| 🔴 **Garman-Kohlhagen**                            | Adds FX rates, for currency options           | Used in **forex options** and global currency trading          |
| 🟣 **Implied Volatility Solver**                   | Reverse use: Solve $\sigma$ from market price | Used by traders to infer market sentiment from option prices   |
| ⚫ **Greeks Derivation (Delta, Gamma, Vega, etc.)** | Derivatives of BSM                            | Used for **risk hedging** and **position management**          |




| Variant                     | Real-World Use Case Example                                 |
| --------------------------- | ----------------------------------------------------------- |
| **European Call/Put**       | Pricing stock options like \$AAPL or \$TSLA                 |
| **Black's Model**           | Oil futures options, energy derivatives                     |
| **Merton Jump Diffusion**   | Options during earnings season or war/political risk events |
| **Garman-Kohlhagen**        | Options on EUR/USD, JPY/USD in banks or hedge funds         |
| **Implied Volatility**      | Risk teams estimating how "expensive" options are           |
| **Greeks (Δ, Γ, 𝜈, ρ, Θ)** | Algorithmic hedging, delta-neutral portfolios               |
