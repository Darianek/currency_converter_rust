// Local module imports
mod cli;
use crate::cli::parse_args;
use currency_converter::{convert_currency, config, api::ApiError};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = parse_args();

    let api_key = config::load_api_key().expect("API key not set. Please set the CURRENCY_CONVERTER_API_KEY environment variable.");

    let source_currency = matches.value_of("source").unwrap();
    let target_currency = matches.value_of("target").unwrap();
    let amount: f64 = matches.value_of("amount").unwrap().parse().expect("Failed to parse the amount as a number.");

    match convert_currency(&api_key, source_currency, target_currency, amount).await {
        Ok((converted_amount, rate)) => {
            println!("Exchange Rate: {}", rate);
            println!("Converted Amount: {}", converted_amount);
        },
        Err(e) => {
            match e.downcast_ref::<ApiError>() {
                Some(ApiError::Network(_)) => eprintln!("Failed to reach the currency conversion API. Please check your network connection."),
                Some(ApiError::ApiResponseError(msg)) => eprintln!("API Error: {}", msg),
                Some(ApiError::CurrencyNotFound) => eprintln!("One or both specified currencies are not supported."),
                _ => eprintln!("An unexpected error occurred: {}", e),
            }
        }
    }

    Ok(())
}