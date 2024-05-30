use crate::error::ContractError;
use cosmwasm_std::Coin;

pub fn send_token_before_buy(
    sent: &[Coin],
    required: Option<Coin>,
) -> Result<(), ContractError> {
    if let Some(required_coin) = required {
        let required_amount = required_coin.amount.u128();
        if required_amount > 0 {
            let sent_sufficient_funds = sent.iter().any(|coin| {
                // check if a given sent coin matches denom
                // and has sufficient amount
                coin.denom == required_coin.denom && coin.amount.u128() >= required_amount
            });

            if sent_sufficient_funds {
                return Ok(());
            } else {
                // return Err(ContractError::InsufficientFundsSend {});
                return Err(ContractError::InsufficientFundsSend { required_amount: required_amount.to_string() });
            }
        }
    }
    Ok(())
}
 