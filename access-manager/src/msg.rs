use cosmwasm_std::{Addr, Uint128};
use secret_toolkit::{
    serialization::Base64JsonOf,
    utils::types::{Contract, Token, WasmCode},
};
use serde::{Deserialize, Serialize};

use crate::{state::Video, types::Payment};

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
        royalty_info: snip721::types::RoyaltyInfo,
        image_url: String,
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

    // Owner
    WithdrawToken {
        to_address: String,
        token: Token,
        amount: Uint128,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Owner {},
    VideoInfo { id: u64 },
    ListVideos { page: u32, page_size: u32 },
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    VideoInfo { video: PublicVideo },
    Owner { address: Addr },
    ListVideos { videos: Vec<PublicVideo> },
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PublicVideo {
    id: u64,
    access_token: Contract,
    name: String,
    image: String,
    royalty_info: snip721::types::RoyaltyInfo,
    price: Payment,
}

impl PublicVideo {
    pub fn from(video: Video) -> Self {
        Self {
            id: video.id,
            access_token: video.access_token,
            name: video.info.name,
            image: video.info.image_url,
            royalty_info: video.info.royalty_info,
            price: video.info.price,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveMsg {
    PurchaseVideo { video_id: u64 },
}
