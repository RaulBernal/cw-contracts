use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{ Map, Item };
use cosmwasm_std::{ Addr, Coin };
use cosmwasm_std::Timestamp;

 


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub owner: Addr, 
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct BurnHistory {
    pub id: u64,
    pub date: Timestamp,
    pub amount: Coin,
    pub denom: String,
    pub address: String
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const BURN_SEQ: Item<u64> = Item::new("burn_seq");
pub const BURN_HISTORY: Map<u64, BurnHistory> = Map::new("burnhistory");