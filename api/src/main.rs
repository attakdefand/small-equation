use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger, Result};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use trillion_dollar_equation::{
    EuropeanCallOption, EuropeanPutOption,
    models::{
        ImpliedVolatilitySolver, 
        EuropeanOption, AmericanOption, BinomialConfig,
        EuropeanMonteCarlo, MonteCarloConfig,
        Greeks
    }
};

#[derive(Deserialize)]
struct OptionRequest {
    underlying_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    volatility: f64,
    dividend_yield: f64,
    is_call: bool,
    option_type: String,
    model: String,
    num_steps: Option<usize>,
    num_paths: Option<usize>,
    num_mc_steps: Option<usize>,
}

#[derive(Serialize)]
struct OptionResponse {
    price: f64,
    delta: Option<f64>,
    gamma: Option<f64>,
    vega: Option<f64>,
    theta: Option<f64>,
    rho: Option<f64>,
    implied_volatility: Option<f64>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

async fn health() -> Result<HttpResponse> {
    let response = HealthResponse {
        status: "OK".to_string(),
        version: "0.1.0".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}

async fn calculate_option_price(option_req: web::Json<OptionRequest>) -> Result<HttpResponse> {
    let price = match option_req.model.as_str() {
        "black-scholes" => {
            if option_req.is_call {
                let option = EuropeanCallOption::new(
                    option_req.underlying_price,
                    option_req.strike_price,
                    option_req.time_to_expiry,
                    option_req.risk_free_rate,
                    option_req.volatility,
                );
                option.price()
            } else {
                let option = EuropeanPutOption::new(
                    option_req.underlying_price,
                    option_req.strike_price,
                    option_req.time_to_expiry,
                    option_req.risk_free_rate,
                    option_req.volatility,
                );
                option.price()
            }
        },
        "binomial" => {
            if option_req.option_type == "american" {
                let option = AmericanOption::new(
                    option_req.underlying_price,
                    option_req.strike_price,
                    option_req.time_to_expiry,
                    option_req.risk_free_rate,
                    option_req.volatility,
                    option_req.is_call,
                    option_req.dividend_yield,
                ).map_err(|_| actix_web::error::ErrorBadRequest("Invalid parameters"))?;
                
                let config = BinomialConfig {
                    num_steps: option_req.num_steps.unwrap_or(100),
                };
                option.price(Some(config)).map_err(|_| actix_web::error::ErrorInternalServerError("Calculation error"))?
            } else {
                let option = EuropeanOption::new(
                    option_req.underlying_price,
                    option_req.strike_price,
                    option_req.time_to_expiry,
                    option_req.risk_free_rate,
                    option_req.volatility,
                    option_req.is_call,
                    option_req.dividend_yield,
                ).map_err(|_| actix_web::error::ErrorBadRequest("Invalid parameters"))?;
                
                let config = BinomialConfig {
                    num_steps: option_req.num_steps.unwrap_or(100),
                };
                option.price(Some(config)).map_err(|_| actix_web::error::ErrorInternalServerError("Calculation error"))?
            }
        },
        "monte-carlo" => {
            let option = EuropeanMonteCarlo::new(
                option_req.underlying_price,
                option_req.strike_price,
                option_req.time_to_expiry,
                option_req.risk_free_rate,
                option_req.volatility,
                option_req.is_call,
                option_req.dividend_yield,
            ).map_err(|_| actix_web::error::ErrorBadRequest("Invalid parameters"))?;
            
            let config = MonteCarloConfig {
                num_paths: option_req.num_paths.unwrap_or(10000),
                num_steps: option_req.num_mc_steps.unwrap_or(252),
                seed: Some(42),
            };
            option.price(Some(config)).map_err(|_| actix_web::error::ErrorInternalServerError("Calculation error"))?
        },
        _ => return Err(actix_web::error::ErrorBadRequest("Unsupported model"))
    };

    // Calculate Greeks for Black-Scholes model
    let (delta, gamma, vega, theta, rho, implied_volatility) = if option_req.model == "black-scholes" {
        let greeks = Greeks::new(
            option_req.underlying_price,
            option_req.strike_price,
            option_req.time_to_expiry,
            option_req.risk_free_rate,
            option_req.volatility,
        );
        
        if option_req.is_call {
            let option = EuropeanCallOption::new(
                option_req.underlying_price,
                option_req.strike_price,
                option_req.time_to_expiry,
                option_req.risk_free_rate,
                option_req.volatility,
            );
            let solver = ImpliedVolatilitySolver::new();
            let iv = solver.calculate_call_iv(&option, price).ok();
            (Some(greeks.delta_call()), Some(greeks.gamma()), Some(greeks.vega()), Some(greeks.theta_call()), Some(greeks.rho_call()), iv)
        } else {
            let option = EuropeanPutOption::new(
                option_req.underlying_price,
                option_req.strike_price,
                option_req.time_to_expiry,
                option_req.risk_free_rate,
                option_req.volatility,
            );
            let solver = ImpliedVolatilitySolver::new();
            let iv = solver.calculate_put_iv(&option, price).ok();
            (Some(greeks.delta_put()), Some(greeks.gamma()), Some(greeks.vega()), Some(greeks.theta_put()), Some(greeks.rho_put()), iv)
        }
    } else {
        (None, None, None, None, None, None)
    };

    let response = OptionResponse {
        price,
        delta,
        gamma,
        vega,
        theta,
        rho,
        implied_volatility,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("Starting Trillion Dollar Equation API server...");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .route("/health", web::get().to(health))
            .route("/api/option-price", web::post().to(calculate_option_price))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}