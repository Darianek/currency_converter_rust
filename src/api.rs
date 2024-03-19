// External dependencies
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

// ApiResponse struct to deserialize JSON response from the currency conversion API.
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub result: String, // Indicates the success or failure of the API request.
    pub conversion_rates: HashMap<String, f64>, // Stores currency conversion rates.
}

// Custom error type that encapsulates various error kinds
#[derive(Debug)]
pub enum ApiError {
    Network(reqwest::Error),
    ApiResponseError(String),
    CurrencyNotFound,
}

// Implement fmt::Display for ApiError
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Network(e) => write!(f, "Network error: {}", e),
            ApiError::ApiResponseError(msg) => write!(f, "API response error: {}", msg),
            ApiError::CurrencyNotFound => write!(f, "Currency not found in API response"),
        }
    }
}

// Implement std::error::Error for ApiError
impl Error for ApiError {}

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
pub async fn fetch_exchange_rate(api_key: &str, source: &str, target: &str) -> Result<f64, ApiError> {
    let url = format!("https://v6.exchangerate-api.com/v6/{}/latest/{}", api_key, source);
    let response = reqwest::get(&url).await.map_err(ApiError::Network)?;
    
    if !response.status().is_success() {
        let error_msg = response.text().await.unwrap_or("Unknown error occurred".to_string());
        return Err(ApiError::ApiResponseError(error_msg));
    }

    let data: ApiResponse = response.json().await.map_err(ApiError::Network)?;

    data.conversion_rates.get(target)
        .cloned()
        .ok_or(ApiError::CurrencyNotFound)
}