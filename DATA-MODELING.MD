4) Data & modeling gotchas (the “don’t get burned” list)

Use market IV, not historical σ. BSM is a mapping between price and σ; markets quote σ.

Dividends matter. Equity options need dividend yield or explicit dividend schedule.

American features. BSM is European; early exercise (deep ITM puts with carry) needs binomial/finite-diff or Barone-Adesi–Whaley approx.

Skew/smile. One σ doesn’t fit all strikes/tenors—use the surface everywhere.

Rates/FX carry. In FX, two curves (domestic/foreign). In commodities, convenience yield and seasonality can sneak in via the futures curve.