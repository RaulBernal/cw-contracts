use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Timestamp, Coin, Addr};

use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub icon_url: String,
    pub title: String,
    pub description: String,
    pub create_date: Timestamp,
    pub required: Required
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AirdropList {
    pub id: u64,
    pub date: Timestamp,
    pub amount: Coin, 
    pub to: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum Required {
    AllDelegator,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllDelegator {
    pub minimum: u64 
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const AIRDROP_SEQ: Item<u64> = Item::new("burn_seq");
pub const AIRDROP_HISTORY: Map<u64, AirdropList> = Map::new("burnhistory");