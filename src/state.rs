use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Configuration struct to store global contract settings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub admin: Addr,     // Admin address authorized to update and manage contract
    pub base_score: u64, // Base score for calculating user scores
}

/// Struct to store individual user's score data based on the credit scoring criteria
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
pub struct UserMetrics {
    pub liquidity: Uint128,        // Liquidity amount provided by user
    pub transaction_count: u64,    // Number of transactions made by user
    pub assets_held: Uint128,      // Total value of assets held by user
    pub protocol_usage_count: u64, // Number of DeFi protocol interactions
    pub credit_score: u64,         // Calculated credit score
}

/// Store configuration settings for the contract
pub const CONFIG: Item<Config> = Item::new("config");

/// Map to store each user's score data by their address
pub const USER_METRICS: Map<&Addr, UserMetrics> = Map::new("user_metrics");
