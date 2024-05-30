use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use crate::state::BurnHistory;

/// InstantiateMsg is the message to instantiate the contract
#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr, 
}

/// ExecuteMsg is the message to execute a command on the contract
#[cw_serde]
pub enum ExecuteMsg {
    Burn { amount: u128},
}

/// QueryMsg is the message to execute a query
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(BurnHistoryResponse)]
    BurnHistory {         
        start_after: Option<u64>,
        limit: Option<u32>
    },
}

/// We define a custom struct for each query response
#[cw_serde]
pub struct BurnHistoryResponse {
    pub entries: Vec<BurnHistory>,
}
