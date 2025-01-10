# README.md

# Okta CLI Application

This project is a command-line interface (CLI) application for interacting with the Okta API, built using Rust and Skim.

## Project Structure

- `src/main.rs`: Entry point of the application, initializes the CLI.
- `src/lib.rs`: Contains common library functionality.
- `src/cli/mod.rs`: Defines CLI commands and handlers.
- `src/okta/mod.rs`: Module for Okta-related functionality.
- `src/okta/client.rs`: Handles interactions with the Okta API.
- `src/okta/models.rs`: Defines data models for users and groups.

## Setup Instructions

1. Clone the repository:
   ```
   git clone <repository-url>
   cd okta-cli
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the application:
   ```
   cargo run
   ```

## Usage

Provide usage examples and command descriptions here.

## Contributing

Feel free to submit issues or pull requests for improvements and bug fixes.