use cosmwasm_std::Uint128;
use secret_toolkit::utils::types::Token;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Payment {
    token: Token,
    amount: Uint128,
}
