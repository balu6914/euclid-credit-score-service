// Import necessary modules
pub mod contract;
mod error;
pub mod helpers;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

// Add necessary imports for environment loading
use dotenv::dotenv;
use std::env;

/// Loads environment variables for the project.
/// This function retrieves the Archway wallet configuration from the .env file.
pub fn load_env_variables() {
    // Load environment variables from `.env` file
    dotenv().ok();

    // Fetch the Archway wallet details from the environment
    let wallet_address =
        env::var("ARCHWAY_WALLET_ADDRESS").expect("ARCHWAY_WALLET_ADDRESS not set");
    let wallet_name = env::var("ARCHWAY_WALLET_NAME").expect("ARCHWAY_WALLET_NAME not set");
    let chain_id = env::var("ARCHWAY_CHAIN_ID").expect("ARCHWAY_CHAIN_ID not set");

    // Display loaded environment variables
    println!("Using Wallet Address: {}", wallet_address);
    println!("Wallet Name: {}", wallet_name);
    println!("Chain ID: {}", chain_id);
}

// Test function to verify `load_env_variables`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_env() {
        load_env_variables(); // This will load and print environment variables
    }
}
