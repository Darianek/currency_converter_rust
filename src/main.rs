// Local module imports
mod cli;
use crate::cli::parse_args;
use currency_converter::{convert_currency, config, api::{self, ApiError}};
use std::error::Error;
use std::io::{self};

// Main function to run the program.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = parse_args();

    // Load the API key from environment variables.
    let api_key = config::load_api_key().expect("API key not set. Please set the CURRENCY_CONVERTER_API_KEY environment variable.");

    // Check if the list argument is present, indicating the user wants to list all available currencies.
    if matches.is_present("list") {
        // Attempt to fetch and list all available currencies.
        match api::fetch_all_currencies(&api_key).await {
            Ok(rates) => {
                for (currency, rate) in rates {
                    println!("{}: {}", currency, rate);
                }
            },
            Err(e) => handle_api_error(Box::new(e)),
        }
    } else if matches.is_present("interactive") || matches.value_of("source").is_none() {
        // Run the program in interactive mode if the interactive argument is present or the source argument is not present.
        let api_key = config::load_api_key()?;
        run_interactive_mode(&api_key).await?;
    } else {
        // Proceed with currency conversion if the list argument is not present.
        let source_currency = matches.value_of("source").unwrap();
        let target_currency = matches.value_of("target").unwrap();
        let amount: f64 = matches.value_of("amount").unwrap().parse().expect("Failed to parse the amount as a number.");

        match convert_currency(&api_key, source_currency, target_currency, amount).await {
            Ok((converted_amount, rate)) => {
                println!("Exchange Rate: {}", rate);
                println!("Converted Amount: {}", converted_amount);
            },
            Err(e) => handle_api_error(e),
        }
    }

    Ok(())
}

// Function to run the program in interactive mode.
async fn run_interactive_mode(api_key: &str) -> Result<(), Box<dyn Error>> {
    let mut source = String::new();
    let mut target = String::new();
    let mut amount = String::new();

    println!("Do you want to print all available currencies and their exchange rates? (yes/no)");
    let mut list = String::new();
    io::stdin().read_line(&mut list)?;
    if list.trim().to_lowercase() == "yes" {
        match api::fetch_all_currencies(api_key).await {
            Ok(rates) => {
                for (currency, rate) in rates {
                    println!("{}: {}", currency, rate);
                }
            },
            Err(e) => handle_api_error(Box::new(e)),
        }
    }
    println!("Enter source currency code (e.g., USD):");
    io::stdin().read_line(&mut source)?;
    println!("Enter target currency code (e.g., EUR):");
    io::stdin().read_line(&mut target)?;
    println!("Enter amount to be converted:");
    io::stdin().read_line(&mut amount)?;

    let amount: f64 = amount.trim().parse().expect("Please enter a valid number for amount.");

    match convert_currency(api_key, source.trim(), target.trim(), amount).await {
        Ok((converted_amount, rate)) => {
            println!("Exchange Rate: {}", rate);
            println!("Converted Amount: {}", converted_amount);
        },
        Err(e) => handle_api_error(e),
    }

    Ok(())
}

// Function to handle errors returned by the API in a unified way.
fn handle_api_error(e: Box<dyn Error>) {
    if let Some(api_error) = e.downcast_ref::<ApiError>() {
        match api_error {
            ApiError::Network(_) => eprintln!("Failed to reach the currency conversion API. Please check your network connection."),
            ApiError::ApiResponseError(msg) => eprintln!("API Error: {}", msg),
            ApiError::CurrencyNotFound => eprintln!("One or both specified currencies are not supported."),
        }
    } else {
        eprintln!("An unexpected error occurred: {}", e);
    }
}