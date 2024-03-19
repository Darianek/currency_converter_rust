// External dependencies
use clap::{App, Arg, ArgMatches};

// Parses command-line arguments using clap.
// 
// # Returns
// 
// A structure containing the matched command-line arguments.
pub fn parse_args() -> ArgMatches {
    App::new("Currency Converter")
        .version("1.0")
        .author("Darianek")
        .about("Converts amounts between different currencies using real-time exchange rates.")
        .arg(Arg::with_name("source")
             .short('s')
             .long("source")
             .value_name("SOURCE")
             .help("Source currency code (e.g., USD)")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("target")
             .short('t')
             .long("target")
             .value_name("TARGET")
             .help("Target currency code (e.g., EUR)")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("amount")
             .short('a')
             .long("amount")
             .value_name("AMOUNT")
             .help("Amount to be converted")
             .required(true)
             .takes_value(true))
        .get_matches()
}