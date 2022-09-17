use cosmwasm_std::{Addr, MessageInfo, StdError, StdResult, Storage};
use secret_toolkit::{
    serialization::Json,
    storage::{Item, Keymap},
    utils::types::{Contract, WasmCode},
};
use serde::{Deserialize, Serialize};

use crate::types::Payment;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub owner: Addr,
    pub access_token_wasm: WasmCode,
}

pub const CONFIG: Item<Config> = Item::new(b"config");

impl Config {
    pub fn assert_owner(&self, info: &MessageInfo) -> StdResult<()> {
        if self.owner != info.sender {
            return Err(StdError::generic_err(
                "you are not the owner of this contract",
            ));
        }

        Ok(())
    }
}

const VIDEO_ID: Item<u64> = Item::new(b"videos_id");

pub fn get_next_video_id(storage: &mut dyn Storage) -> StdResult<u64> {
    let new_id = match VIDEO_ID.may_load(storage)? {
        Some(id) => id + 1,
        None => 1,
    };
    VIDEO_ID.save(storage, &new_id)?;

    Ok(new_id)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VideoInfo {
    pub name: String,
    pub royalty_info: snip721::types::RoyaltyInfo,
    pub image_url: String,
    pub video_url: String,
    pub decryption_key: String,
    pub price: Payment,
}

#[derive(Serialize, Deserialize)]
pub struct UninitializedVideo {
    pub id: u64,
    pub info: VideoInfo,
}

pub const UNINIT_VID: Item<UninitializedVideo, Json> = Item::new(b"uninitialized_video");

#[derive(Serialize, Deserialize)]
pub struct Video {
    pub id: u64,
    pub access_token: Contract,
    pub info: VideoInfo,
}

pub const VIDEOS: Keymap<u64, Video, Json> = Keymap::new(b"videos");

impl Video {
    pub fn from_uninitialized(
        storage: &mut dyn Storage,
        access_token: Contract,
    ) -> StdResult<Self> {
        let uninitialized = UNINIT_VID.load(storage)?;
        Ok(Self {
            id: uninitialized.id,
            access_token,
            info: uninitialized.info,
        })
    }
}

const REGISTERED_PAYMENT: Item<u8> = Item::new(b"registered_payment");

impl Payment {
    pub fn register_snip20(storage: &mut dyn Storage, address: Addr) -> StdResult<()> {
        REGISTERED_PAYMENT
            .add_suffix(address.as_bytes())
            .save(storage, &1)
    }

    pub fn is_snip20_registered(storage: &dyn Storage, address: Addr) -> StdResult<bool> {
        Ok(REGISTERED_PAYMENT
            .add_suffix(address.as_bytes())
            .may_load(storage)?
            .is_some())
    }
}
