use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
   Addr
};

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.,
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);
 
