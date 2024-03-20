// Module declarations
pub mod api;
pub mod config;

// External dependencies
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;

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

// RateCache struct for caching exchange rates.
pub struct RateCache {
    rates: Mutex<HashMap<String, (f64, Instant)>>,
    ttl: Duration,
}

// Implementation of RateCache
impl RateCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            rates: Mutex::new(HashMap::new()),
            ttl,
        }
    }

    // Get the exchange rate for a given currency pair.
    pub fn get(&self, pair: &str) -> Option<f64> {
        let rates = self.rates.lock().unwrap();
        rates.get(pair).and_then(|(rate, timestamp)| {
            if timestamp.elapsed() < self.ttl {
                Some(*rate)
            } else {
                None
            }
        })
    }

    // Set the exchange rate for a given currency pair.
    pub fn set(&self, pair: String, rate: f64) {
        let mut rates = self.rates.lock().unwrap();
        rates.insert(pair, (rate, Instant::now()));
    }
}

// Global rate cache with a TTL of 2 hours
pub static GLOBAL_RATE_CACHE: Lazy<RateCache> = Lazy::new(|| RateCache::new(Duration::from_secs(7200)));