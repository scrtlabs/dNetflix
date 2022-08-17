use cosmwasm_std::Uint128;
use secret_toolkit::utils::types::WasmCode;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub access_token_wasm: WasmCode,
    pub entropy: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    NewVideo {
        name: String,
        royalty_info: snip721::royalties::RoyaltyInfo,
        price: Uint128,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}
