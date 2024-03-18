mod api;
mod cli;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::parse_args();

    let api_key = config::load_api_key()?;
    
    let source_currency = matches.value_of("source").unwrap();
    let target_currency = matches.value_of("target").unwrap();
    let amount: f64 = matches.value_of("amount").unwrap().parse()?;

    let rate = api::fetch_exchange_rate(&api_key, source_currency, target_currency).await?;
    let converted_amount = amount * rate;

    println!("Exchange Rate: {}", rate);
    println!("Converted Amount: {}", converted_amount);

    Ok(())
}