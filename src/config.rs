// External dependencies
use std::env;

// Loads the API key from environment variables.
// 
// # Returns
// 
// A Result containing the API key as a String if found, or an error message if not.
pub fn load_api_key() -> Result<String, &'static str> {
    env::var("CURRENCY_CONVERTER_API_KEY")
        .map_err(|_| "CURRENCY_CONVERTER_API_KEY not set")
}