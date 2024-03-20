// External dependencies
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use crate::GLOBAL_RATE_CACHE;

// Represents the response structure of the API for currencies pair.
#[derive(Debug, Deserialize)]
struct ConversionRateResponse {
    result: String,
    conversion_rate: f64,
    // Add other relevant fields if necessary
}

// Represents the response structure of the API for all currencies.
#[derive(Debug, Deserialize)]
struct AllCurrenciesResponse {
    result: String,
    conversion_rates: HashMap<String, f64>,
    // Add other relevant fields if necessary
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
    let pair = format!("{}_{}", source, target);
    if let Some(rate) = GLOBAL_RATE_CACHE.get(&pair) {
        println!("Rate for {} fetched from cache.", pair);
        return Ok(rate);
    }
    println!("Fetching rate for {} from the API.", pair);

    let url = format!("https://v6.exchangerate-api.com/v6/{}/pair/{}/{}", api_key, source, target);
    let resp = reqwest::get(&url).await.map_err(ApiError::Network)?;

    if !resp.status().is_success() {
        return Err(ApiError::ApiResponseError("Failed to fetch data from the API. One or both of the curriencies are not valid.".into()));
    }

    let data: ConversionRateResponse = resp.json().await.map_err(ApiError::Network)?;
    
    if data.result == "success" {
        GLOBAL_RATE_CACHE.set(pair, data.conversion_rate);
        Ok(data.conversion_rate)
    } else {
        Err(ApiError::ApiResponseError("API responded with an error.".into()))
    }
}

// Fetches a list of all available currencies and their exchange rates.
// 
// # Arguments
// 
// * `api_key` - The API key for authenticating with the currency conversion API.
// 
// # Returns
// 
// A `Result` containing a `HashMap` of currency codes to their exchange rates (`f64`) if successful, or an `ApiError` if an error occurs.
//
// # Examples
//
// ```
// # async fn run_example() -> Result<(), Box<dyn std::error::Error>> {
// let api_key = "your_api_key";
// let currencies = fetch_all_currencies(api_key).await?;
// for (currency, rate) in currencies {
//     println!("{}: {}", currency, rate);
// }
// # Ok(())
// # }
// ```
pub async fn fetch_all_currencies(api_key: &str) -> Result<HashMap<String, f64>, ApiError> {
    println!("Fetching all currency rates from the API.");
    let url = format!("https://v6.exchangerate-api.com/v6/{}/latest/USD", api_key);
    let response = reqwest::get(&url).await.map_err(ApiError::Network)?;

    if !response.status().is_success() {
        let error_msg = response.text().await.unwrap_or_else(|_| "Unknown error occurred.".into());
        return Err(ApiError::ApiResponseError(error_msg));
    }

    let data: AllCurrenciesResponse = response.json().await.map_err(ApiError::Network)?;

    if data.result == "success" {
        Ok(data.conversion_rates)
    } else {
        Err(ApiError::ApiResponseError("API responded with an error.".into()))
    }
}