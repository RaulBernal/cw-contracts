#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Order, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin, BankMsg, CosmosMsg};
use cw2::set_contract_version;
use std::ops::Add;


use cw_storage_plus::Bound;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, BurnHistoryResponse};
use crate::state::{Config, CONFIG, BURN_SEQ, BurnHistory, BURN_HISTORY};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-burn";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
// Token info
const DENOM: &str = "ubcna";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;
    BURN_SEQ.save(deps.storage, &0u64)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Burn { amount } => execute::burn_now(deps, env, info, amount),
    }
}

pub mod execute {
    use super::*;

    pub fn burn_now(deps: DepsMut, env: Env, info: MessageInfo, amount: u128) -> Result<Response, ContractError> {
        let owner = CONFIG.load(deps.storage)?.owner;
        // Check if the sender is the owner
        if info.sender != owner {
            return Err(ContractError::Unauthorized {});
        }
        // Burn value
        let coin = Coin {
            amount: amount.into(),
            denom: DENOM.to_string(),
        };
        let amount = [coin.clone()].to_vec();
        // Burn it!
        let burn_msg = BankMsg::Burn { amount };
        // Then we add the message to the response
        let msgs: Vec<CosmosMsg> = vec![burn_msg.into()];

        // Save the burn history
        let id = BURN_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;
        let new_entry = BurnHistory {
            id,
            date: env.block.time,
            denom: DENOM.to_string(),
            amount: coin.clone(),
            address: info.sender.to_string()
        };
        BURN_HISTORY.save(deps.storage, id, &new_entry)?;
        Ok(Response::new()
            .add_attribute("action", "burn_now")
            .add_messages(msgs)
        )
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::BurnHistory { start_after, limit } => to_json_binary(&query::burn_history(deps, start_after, limit)?),
    }
}

pub mod query {
    use super::*;


    // Limits for pagination
    const MAX_LIMIT: u32 = 30;
    const DEFAULT_LIMIT: u32 = 10;

    pub fn burn_history(deps: Deps, start_after: Option<u64>, limit: Option<u32>) -> StdResult<BurnHistoryResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(Bound::exclusive);
        let entries: StdResult<Vec<_>> = BURN_HISTORY
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect();

        let result = BurnHistoryResponse {
            entries: entries?.into_iter().map(|l| l.1).collect(),
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env};

    #[test]
    fn test_date() {
        println!("{:?}", mock_env());
    }
}

