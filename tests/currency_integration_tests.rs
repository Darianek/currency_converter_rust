use currency_converter::{config, convert_currency};

#[tokio::test]
async fn test_conversion() {
    let api_key = config::load_api_key().expect("API key not set. Please set the CURRENCY_CONVERTER_API_KEY environment variable.");
    let result = convert_currency(&api_key, "USD", "EUR", 100.0).await;
    assert!(result.is_ok());
    let (amount, rate) = result.unwrap();
    assert!(amount > 0.0);
    assert!(rate > 0.0);
}