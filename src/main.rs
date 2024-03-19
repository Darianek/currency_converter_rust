// Local module imports
mod cli;
use currency_converter::{convert_currency, config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::parse_args();

    let api_key = config::load_api_key()?;
    
    let source_currency = matches.value_of("source").unwrap();
    let target_currency = matches.value_of("target").unwrap();
    let amount: f64 = matches.value_of("amount").unwrap().parse()?;

    let (converted_amount, rate) = convert_currency(&api_key, source_currency, target_currency, amount).await?;

    println!("Exchange Rate: {}", rate);
    println!("Converted Amount: {}", converted_amount);

    Ok(())
}