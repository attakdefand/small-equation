//! Benchmark tests for the Trillion Dollar Equation project

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use trillion_dollar_equation::models::EuropeanCallOption;

fn benchmark_european_call_option_pricing(c: &mut Criterion) {
    c.bench_function("european_call_option_pricing", |b| {
        b.iter(|| {
            let option = EuropeanCallOption::new(
                black_box(100.0),  // Underlying price
                black_box(100.0),  // Strike price
                black_box(1.0),    // Time to expiry (1 year)
                black_box(0.05),   // Risk-free rate (5%)
                black_box(0.2),    // Volatility (20%)
            ).unwrap();
            
            black_box(option.price())
        })
    });
}

criterion_group!(benches, benchmark_european_call_option_pricing);
criterion_main!(benches);