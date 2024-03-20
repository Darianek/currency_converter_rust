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
              .takes_value(true)) 
         .arg(Arg::with_name("target")
              .short('t')
              .long("target")
              .value_name("TARGET")
              .help("Target currency code (e.g., EUR)")
              .takes_value(true)) 
         .arg(Arg::with_name("amount")
              .short('a')
              .long("amount")
              .value_name("AMOUNT")
              .help("Amount to be converted")
              .allow_hyphen_values(true) 
              .takes_value(true)) 
         .arg(Arg::with_name("list")
              .long("list")
              .help("Lists all available currencies and their current exchange rates"))
         .arg(Arg::with_name("interactive")
              .long("interactive")
              .short('i')
              .help("Run the program in interactive mode"))
         .get_matches()
 }