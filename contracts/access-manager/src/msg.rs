use cosmwasm_std::Uint128;
use secret_toolkit::{serialization::Base64JsonOf, utils::types::WasmCode};
use serde::Deserialize;

use crate::types::Payment;

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
        video_url: String,
        decryption_key: String,
        price: Payment,
    },
    PurchaseVideo {
        video_id: u64,
    },

    // Receiver interface
    Receive {
        sender: String,
        from: String,
        amount: Uint128,
        msg: Base64JsonOf<ReceiveMsg>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveMsg {
    PurchaseVideo { video_id: u64 },
}
