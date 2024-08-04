# Blockchain Application

This is a simple blockchain application implemented in Rust. It allows you to start a blockchain node, create accounts, transfer funds between accounts, and check account balances.

## Prerequisites

- Rust and Cargo (latest stable version)
- Git

## Setup

1. Clone the repository:

   ```
   git clone https://github.com/georgethoppil/blockchain.git
   cd blockchain-main
   ```

2. Set up the configuration:

   - Ensure there's a `configuration` directory in the project root.
   - Inside the `configuration` directory, create two files:
     - `base.yaml`: Contains the base configuration.
     - `local.yaml`: Contains local development configuration.

   Example `base.yaml`:

   ```yaml
   application:
     port: 8080
     host: "127.0.0.1"
     mining_timeout: 5
   ```

   Example `local.yaml` (override any base settings as needed):

   ```yaml
   application:
     port: 8000
   ```

3. Set the environment variable(optional):
   ```
   export APP_ENVIRONMENT=local
   ```

## Development

To run development version of the app:

1. Start the blockchain node:

   ```
   cargo run -- start-node
   ```

2. Create a new accounts:

   ```
   cargo run -- create-account alice 1000
   cargo run -- create-account bob 1000
   ```

3. Transfer funds between accounts:

   ```
   cargo run -- transfer alice bob 500
   ```

4. Check account balance:
   ```
   cargo run -- balance alice
   ```

## Release build

To build the application, run:

```
cargo build --release
```

This will create an executable in the `target/release` directory.
After building, you can run the application using the `b` command (assuming you've included the config directory). Here are some example commands:

1. Start the blockchain node:

   ```
   b start-node
   ```

2. Create a new accounts:

   ```
   b create-account alice 1000
   b create-account bob 1000
   ```

3. Transfer funds between accounts:

   ```
   b transfer alice bob 500
   ```

4. Check account balance:
   ```
   b balance alice
   ```

## Command Line Interface

The application uses a command-line interface with the following structure:

- `b`: The main command (short for "blockchain")
  - `start-node`: Starts the blockchain node
  - `create-account`: Creates a new account
    - `--id`: ID of the account
    - `--balance`: Starting balance of the account
  - `transfer`: Transfers funds between accounts
    - `--from-account`: ID of the source account
    - `--to-account`: ID of the destination account
    - `--amount`: Amount to transfer
  - `balance`: Gets the balance of an account
    - `--account`: ID of the account to query

## Testing

To run the tests for the application:

```
cargo test
```
