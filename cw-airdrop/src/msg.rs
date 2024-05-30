use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::AirdropList;
use cosmwasm_std::{Addr, BalanceResponse, Timestamp};
use crate::state::Required;

#[cw_serde]
pub struct InstantiateMsg { 
    pub owner: Addr, 
    pub icon_url: String,
    pub title: String,
    pub description: String,
    pub required: Required
}

#[cw_serde]
pub enum ExecuteMsg {
    ClaimAirdrop {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetAirdropListResponse)]
    GetAirdropList {        
        start_after: Option<u64>,
        limit: Option<u32>
    },
    /// This returns cosmwasm_std::AllBalanceResponse to demo use of the querier
    #[returns(WalletInfoResponse)] // BalanceResponse
    WalletInfo { address: String },

    #[returns(ConfigInfoResponse)] 
    GetConfig {},
}

/// We define a custom struct for each query response
#[cw_serde]
pub struct GetAirdropListResponse {
    pub entries: Vec<AirdropList>,
}

/// We define a custom struct for each query response
#[cw_serde]
pub struct WalletInfoResponse {
    pub balance: BalanceResponse,
    pub bonded: String
}
/// Return config contract
#[cw_serde]
pub struct ConfigInfoResponse {
    pub owner: Addr,
    pub icon_url: String,
    pub title: String,
    pub description: String,
    pub create_date: Timestamp,
    pub required: Required
}