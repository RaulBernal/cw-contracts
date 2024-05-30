use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient funds sent (error {required_amount})")]
    InsufficientFundsSend { required_amount: String },
    
    #[error("You do not have enough bounded tokens to claim your airdrop (Needed: xxxx)")]
    UnauthorizedBoundRules {  },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
