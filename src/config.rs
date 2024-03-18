use std::env;

pub fn load_api_key() -> Result<String, &'static str> {
    env::var("CURRENCY_CONVERTER_API_KEY")
        .map_err(|_| "CURRENCY_CONVERTER_API_KEY not set")
}