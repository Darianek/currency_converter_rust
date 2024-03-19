// External dependencies
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

// ApiResponse struct to deserialize JSON response from the currency conversion API.
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub result: String, // Indicates the success or failure of the API request.
    pub conversion_rates: HashMap<String, f64>, // Stores currency conversion rates.
}

// Asynchronously fetches the exchange rate between two currencies.
// 
// # Arguments
// 
// * `api_key` - API key for authenticating with the currency conversion API.
// * `source` - Source currency code.
// * `target` - Target currency code.
// 
// # Returns
// 
// A Result containing the exchange rate as f64 if successful, or an Error if not.

pub async fn fetch_exchange_rate(api_key: &str, source: &str, target: &str) -> Result<f64, Box<dyn Error>> {
    let url = format!("https://v6.exchangerate-api.com/v6/{}/latest/{}", api_key, source);
    let response: ApiResponse = reqwest::get(&url).await?.json().await?;
    response.conversion_rates.get(target)
        .cloned()
        .ok_or_else(|| "Currency not found".into())
}