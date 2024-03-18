use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub result: String,
    pub conversion_rates: HashMap<String, f64>,
}

pub async fn fetch_exchange_rate(api_key: &str, source: &str, target: &str) -> Result<f64, Box<dyn Error>> {
    let url = format!("https://v6.exchangerate-api.com/v6/{}/latest/{}", api_key, source);
    let response: ApiResponse = reqwest::get(&url).await?.json().await?;
    response.conversion_rates.get(target)
        .cloned()
        .ok_or_else(|| "Currency not found".into())
}