use cosmwasm_schema::write_api;

// Import messages directly from the package name instead of using `crate::`.
use credit_score::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
