use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,   // Admin address to manage the contract
    pub base_score: u64, // Base score for calculating credit scores
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Update the liquidity for a user
    UpdateLiquidity { user: Addr, liquidity: Uint128 },
    /// Update the transaction count for a user
    UpdateTransactionCount { user: Addr, transaction_count: u64 },
    /// Update the assets held by a user
    UpdateAssetsHeld { user: Addr, assets_held: Uint128 },
    /// Update the protocol usage count for a user
    UpdateProtocolUsage {
        user: Addr,
        protocol_usage_count: u64,
    },
    /// Calculate the credit score for a user
    CalculateScore { user: Addr },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get the credit score for a specific user
    #[returns(UserScoreResponse)]
    GetScore { user: Addr },
}

#[cw_serde]
pub struct UserScoreResponse {
    pub user: Addr,                // User's address
    pub liquidity: Uint128,        // User's liquidity in the system
    pub transaction_count: u64,    // User's transaction count
    pub assets_held: Uint128,      // User's assets held in the system
    pub protocol_usage_count: u64, // User's protocol usage count
    pub score: u64,                // Calculated credit score
}
