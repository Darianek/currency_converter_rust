# Currency Converter

A simple yet powerful command-line tool developed in Rust to convert amounts between different currencies using real-time exchange rates fetched from an [exchangerate-api](https://app.exchangerate-api.com/). This tool is perfect for anyone needing quick currency conversions right from the terminal.

## Project Structure

The `currency_converter` project is organized as follows to ensure readability, maintainability, and ease of expansion:
```
currency_converter/
├── Cargo.toml # Project manifest file for Rust, defining packages and dependencies.
├── Dockerfile # Instructions for Docker to build the application into a container.
├── src/ # Source directory containing all the Rust code.
│ ├── main.rs # Entry point that handles CLI parsing and orchestrates the application flow.
│ ├── lib.rs # Central module file for shared logic or types across modules.
│ ├── cli.rs # Module for CLI argument parsing and validation.
│ ├── api.rs # Module for handling API calls and responses.
│ └── config.rs # Module for application configuration, e.g., loading environment variables.
└── tests/ # Directory for integration tests.
```

## Getting Started

To run this application, you'll need Rust installed on your machine. If you haven't already, you can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

### Setting Up the Environment Variable

Before running the application, set the `CURRENCY_CONVERTER_API_KEY` environment variable to your API key obtained from ExchangeRate-API:

- **To obtain an API key, register at** [ExchangeRate-API](https://app.exchangerate-api.com/sign-up).
- **Linux/macOS**: `export CURRENCY_CONVERTER_API_KEY=your_api_key_here`
- **Windows (Command Prompt)**: `set CURRENCY_CONVERTER_API_KEY=your_api_key_here`
- **Windows (PowerShell)**: `$env:CURRENCY_CONVERTER_API_KEY="your_api_key_here"`

### Running the Application

Navigate to the project directory and run the application using Cargo:

```bash
cargo run -- --source USD --target EUR --amount 100
```

This command converts 100 USD to EUR based on current exchange rates.

## Running with Docker

If you prefer to run the application using Docker, follow these steps:

### Build the Docker Image:

Navigate to the project directory where the Dockerfile is located and run the following command:

```bash
docker build -t currency_converter .
```

### Run the Docker Container:

After building the image, you can run the application in a container. To do simple conversion, replace the source, target, and amount with your desired values:

```bash
docker run -e CURRENCY_CONVERTER_API_KEY=your_api_key_here currency_converter --source USD --target EUR --amount 100
```

If you want to try the interactive mode, use:

```bash
docker run -it -e CURRENCY_CONVERTER_API_KEY=your_api_key_here currency_converter --interactive
```

This setup allows you to run the currency converter without directly installing Rust or other dependencies on your system.