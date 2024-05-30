#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Order, Coin, BankMsg, BankQuery, BalanceResponse, QuerierWrapper, StdError, Uint128, AllDenomMetadataResponse, PageRequest};
use cw2::set_contract_version;
use std::ops::Add;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetAirdropListResponse, InstantiateMsg, QueryMsg, WalletInfoResponse};
use crate::state::{Config, CONFIG, AIRDROP_HISTORY, AIRDROP_SEQ, AirdropList};
use cw_storage_plus::{Bound};
use crate::contract::query::{get_bonded};
 
use crate::msg;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-airdrop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Token info
const DENOM: &str = "udao"; 
const AIRDROP_AMOUNT: u128 = 1000000; 

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg, 
) -> Result<Response, ContractError> {

    let config = Config {
        owner: msg.owner.clone(),
        icon_url: msg.icon_url.clone(),
        title: msg.title.clone(),
        description: msg.description.clone(),
        create_date: env.block.time,
        required: msg.required.clone()
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;
    AIRDROP_SEQ.save(deps.storage, &0u64)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", msg.owner.clone()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ClaimAirdrop {} => execute::claim_airdrop(deps, env, info)
    }
}

pub mod execute {
    use super::*;

    pub fn claim_airdrop(deps: DepsMut, env: Env, info: MessageInfo,) -> Result<Response, ContractError> {

        let bonded = get_bonded(&deps.querier, info.sender.to_string())?;
        if bonded.to_string() == "0" {
            return Err(ContractError::UnauthorizedBoundRules {});
        }

        let amount = Coin {
            denom: DENOM.to_string(),
            amount: AIRDROP_AMOUNT.into()
        };
        let airdrop_msg = BankMsg::Send { 
            to_address: info.sender.to_string(),
            amount: vec![amount.clone()]
        };
        let id = AIRDROP_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

        AIRDROP_HISTORY.save(deps.storage, id, &AirdropList {
            id,
            date: env.block.time,
            amount,
            to: info.sender.to_string()
        })?;

        Ok(Response::new()
            .add_attribute("action", "claim_airdrop")
            .add_message(airdrop_msg)
        )
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAirdropList { start_after, limit } => to_json_binary(&query::airdrop_history(deps, start_after, limit)?),
        QueryMsg::WalletInfo { address } => to_json_binary(&query::wallet_info(deps, address)?),
        QueryMsg::GetConfig { } => to_json_binary(&query::get_config(deps)?),
    }
}

pub mod query {
    use super::*;

    // Limits for pagination
    const MAX_LIMIT: u32 = 30;
    const DEFAULT_LIMIT: u32 = 10;

    pub fn airdrop_history(deps: Deps, start_after: Option<u64>, limit: Option<u32>) -> StdResult<GetAirdropListResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(Bound::exclusive);
        let entries: StdResult<Vec<_>> = AIRDROP_HISTORY
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect();

        let result = GetAirdropListResponse {
            entries: entries?.into_iter().map(|l| l.1).collect(),
        };
        Ok(result)
    }
    pub fn wallet_info(deps: Deps, address: String) -> StdResult<WalletInfoResponse> {
        // Request balance
        let request_balance = BankQuery::Balance {
            address: address.to_string(),
            denom: DENOM.to_string(),
        }.into();

        let res_balance: BalanceResponse = deps.querier.query(&request_balance)?;
        let bonded = get_bonded(&deps.querier, address.to_string())?;

        Ok(msg::WalletInfoResponse { 
            balance: res_balance,
            bonded: bonded.to_string()
        })
    }
    pub fn get_config(deps: Deps) -> StdResult<WalletInfoResponse> {
        // Request balance
        let request_balance = BankQuery::Balance {
            address: address.to_string(),
            denom: DENOM.to_string(),
        }.into();

        let res_balance: BalanceResponse = deps.querier.query(&request_balance)?;
        let bonded = get_bonded(&deps.querier, address.to_string())?;

        Ok(msg::WalletInfoResponse { 
            balance: res_balance,
            bonded: bonded.to_string()
        })
    }
    // get_bonded returns the total amount of delegations from contract
    // it ensures they are all the same denom
    pub fn get_bonded(querier: &QuerierWrapper, contract_addr: impl Into<String>) -> StdResult<Uint128> {
        let bonds = querier.query_all_delegations(contract_addr)?;
        if bonds.is_empty() {
            return Ok(Uint128::new(0));
        }
        let denom = bonds[0].amount.denom.as_str();
        bonds.iter().try_fold(Uint128::zero(), |acc, d| {
            if d.amount.denom.as_str() != denom {
                Err(StdError::generic_err(format!(
                    "different denoms in bonds: '{}' vs '{}'",
                    denom, &d.amount.denom
                )))
            } else {
                Ok(acc + d.amount.amount)
            }
        })
    }
    pub fn get_all_tokens(querier: &QuerierWrapper) -> StdResult<AllDenomMetadataResponse> {
        const PAGE_SIZE: u32 = 10;
        let all_denom = querier.query_all_denom_metadata(PageRequest {
            key: None,
            limit: PAGE_SIZE,
            reverse: false,
        })?;
        Ok(all_denom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json, BalanceResponse};
 


    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(1000000000000, "ubcna"));

         let state = State {
            owner: info.sender.clone() 
        };
 
        let _ = STATE.save(&mut deps.storage, &state);
        let _ = AIRDROP_SEQ.save(&mut deps.storage, &0u64);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // beneficiary can release it
        let info_user = mock_info("anyone", &coins(2000000, "ubcna"));
        let msg_claim = ExecuteMsg::ClaimAirdrop {};
        let res = execute(deps.as_mut(), mock_env(), info_user.clone(), msg_claim).unwrap();
        println!("{}", serde_json::to_string_pretty(&res).unwrap()); 

        println!("Info user {:?}", info_user.sender.to_string() );

        // it worked, let's query the state
        let res_2 = query(deps.as_ref(), mock_env(), QueryMsg::OtherBalance { address: info_user.sender.to_string() }).unwrap();
        let value: BalanceResponse = from_json(&res_2).unwrap();
        println!("{:?}", value.amount);
  
        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetAirdropList { start_after: Some(0), limit: Some(30) }).unwrap();
        let value: GetAirdropListResponse = from_json(&res).unwrap();
        println!("{:?}", value.entries);
        // assert_eq!(17, value.count);

        // let test = deps.querier.query_all_balances("test".to_string())?;
        println!("{:#?}", deps.as_ref());
    }

    /* #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(18, value.count);
    } */

}
