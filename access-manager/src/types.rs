use cosmwasm_std::Uint128;
use secret_toolkit::utils::types::Token;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Payment {
    pub token: Token,
    pub amount: Uint128,
}
