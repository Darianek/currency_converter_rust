# Currency Converter

A simple yet powerful command-line tool developed in Rust to convert amounts between different currencies using real-time exchange rates fetched from an [exchangerate-api](https://app.exchangerate-api.com/). This tool is perfect for anyone needing quick currency conversions right from the terminal.

## Project Structure

The `currency_converter` project is organized as follows to ensure readability, maintainability, and ease of expansion:
```
currency_converter/
├── Cargo.toml # Project manifest file for Rust, defining packages and dependencies.
├── src/ # Source directory containing all the Rust code.
│ ├── main.rs # Entry point that handles CLI parsing and orchestrates the application flow.
│ ├── lib.rs # Central module file for shared logic or types across modules.
│ ├── cli.rs # Module for CLI argument parsing and validation.
│ ├── api.rs # Module for handling API calls and responses.
│ └── config.rs # Module for application configuration, e.g., loading environment variables.
└── tests/ # Directory for integration tests.
└── api_tests.rs # Example integration tests for the API functionality.
```

## Getting Started

To run this application, you'll need Rust installed on your machine. If you haven't already, you can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

### Setting Up the Environment Variable

Before running the application, set the `CURRENCY_CONVERTER_API_KEY` environment variable to your API key obtained from ExchangeRate-API:

- **Linux/macOS**: `export CURRENCY_CONVERTER_API_KEY=your_api_key_here`
- **Windows (Command Prompt)**: `set CURRENCY_CONVERTER_API_KEY=your_api_key_here`
- **Windows (PowerShell)**: `$env:CURRENCY_CONVERTER_API_KEY="your_api_key_here"`

### Running the Application

Navigate to the project directory and run the application using Cargo:

```bash
cargo run -- --source USD --target EUR --amount 100
```

This command converts 100 USD to EUR based on current exchange rates.
