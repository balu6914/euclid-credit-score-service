#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, USER_METRICS};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        admin: deps.api.addr_validate(&msg.admin)?,
        base_score: msg.base_score,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateLiquidity { user, liquidity } => {
            update_liquidity(deps, info, user, liquidity)
        }
        ExecuteMsg::UpdateTransactionCount {
            user,
            transaction_count,
        } => update_transaction_count(deps, info, user, transaction_count),
        ExecuteMsg::UpdateAssetsHeld { user, assets_held } => {
            update_assets_held(deps, info, user, assets_held)
        }
        ExecuteMsg::UpdateProtocolUsage {
            user,
            protocol_usage_count,
        } => update_protocol_usage(deps, info, user, protocol_usage_count),
        ExecuteMsg::CalculateScore { user } => calculate_score(deps, user),
    }
}

pub fn update_liquidity(
    deps: DepsMut,
    info: MessageInfo,
    user: Addr,
    liquidity: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let mut user_metrics = USER_METRICS
        .may_load(deps.storage, &user)?
        .unwrap_or_default();
    user_metrics.liquidity = liquidity;
    USER_METRICS.save(deps.storage, &user, &user_metrics)?;

    Ok(Response::new()
        .add_attribute("method", "update_liquidity")
        .add_attribute("user", user.to_string())
        .add_attribute("liquidity", liquidity.to_string()))
}

pub fn update_transaction_count(
    deps: DepsMut,
    info: MessageInfo,
    user: Addr,
    transaction_count: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let mut user_metrics = USER_METRICS
        .may_load(deps.storage, &user)?
        .unwrap_or_default();
    user_metrics.transaction_count = transaction_count;
    USER_METRICS.save(deps.storage, &user, &user_metrics)?;

    Ok(Response::new()
        .add_attribute("method", "update_transaction_count")
        .add_attribute("user", user.to_string())
        .add_attribute("transaction_count", transaction_count.to_string()))
}

pub fn update_assets_held(
    deps: DepsMut,
    info: MessageInfo,
    user: Addr,
    assets_held: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let mut user_metrics = USER_METRICS
        .may_load(deps.storage, &user)?
        .unwrap_or_default();
    user_metrics.assets_held = assets_held;
    USER_METRICS.save(deps.storage, &user, &user_metrics)?;

    Ok(Response::new()
        .add_attribute("method", "update_assets_held")
        .add_attribute("user", user.to_string())
        .add_attribute("assets_held", assets_held.to_string()))
}

pub fn update_protocol_usage(
    deps: DepsMut,
    info: MessageInfo,
    user: Addr,
    protocol_usage_count: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let mut user_metrics = USER_METRICS
        .may_load(deps.storage, &user)?
        .unwrap_or_default();
    user_metrics.protocol_usage_count = protocol_usage_count;
    USER_METRICS.save(deps.storage, &user, &user_metrics)?;

    Ok(Response::new()
        .add_attribute("method", "update_protocol_usage")
        .add_attribute("user", user.to_string())
        .add_attribute("protocol_usage_count", protocol_usage_count.to_string()))
}

pub fn calculate_score(deps: DepsMut, user: Addr) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let user_metrics = USER_METRICS
        .may_load(deps.storage, &user)?
        .unwrap_or_default();

    // Calculate score based on liquidity, assets held, transaction count, and protocol usage
    let score = config.base_score
        + (user_metrics.liquidity.u128() / 1000) as u64
        + (user_metrics.assets_held.u128() / 1000) as u64
        + user_metrics.transaction_count
        + user_metrics.protocol_usage_count;

    USER_METRICS.update(deps.storage, &user, |existing| -> StdResult<_> {
        let mut metrics = existing.unwrap_or_default();
        metrics.credit_score = score;
        Ok(metrics)
    })?;

    Ok(Response::new()
        .add_attribute("method", "calculate_score")
        .add_attribute("user", user.to_string())
        .add_attribute("score", score.to_string()))
}

/// Query the user score using the user address
pub fn query_user_score(deps: Deps, _env: Env, user_address: Addr) -> StdResult<Binary> {
    let user_metrics = USER_METRICS.load(deps.storage, &user_address)?;
    to_json_binary(&user_metrics)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetScore { user } => query_user_score(deps, _env, user),
    }
}

#[cfg(test)]
mod tests {}
