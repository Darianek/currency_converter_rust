// Module declarations
pub mod api;
pub mod config;

// External dependencies
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

// ApiResponse struct for deserializing API responses.
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub result: String, // Result of the API request ('success' or 'failure').
    pub conversion_rates: HashMap<String, f64>, // Conversion rates keyed by currency code.
}

// Converts a given amount from one currency to another.
// 
// # Arguments
// 
// * `api_key` - The API key for the currency conversion service.
// * `source` - The source currency code.
// * `target` - The target currency code.
// * `amount` - The amount to convert.
// 
// # Returns
// 
// A Result containing a tuple of the converted amount and the exchange rate used for conversion, or an error.
pub async fn convert_currency(api_key: &str, source: &str, target: &str, amount: f64) -> Result<(f64, f64), Box<dyn Error>> {
    let rate = api::fetch_exchange_rate(api_key, source, target).await?;
    Ok((amount * rate, rate))
}