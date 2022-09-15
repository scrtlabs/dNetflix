use cosmwasm_std::{Addr, MessageInfo, StdError, StdResult, Storage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use secret_toolkit::{
    serialization::Json,
    storage::{TypedStore, TypedStoreMut},
    utils::types::{Contract, WasmCode},
};
use serde::{Deserialize, Serialize};

use crate::types::Payment;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub owner: Addr,
    pub access_token_wasm: WasmCode,
}

impl Config {
    const STORAGE_KEY: &'static [u8] = b"config";

    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        TypedStoreMut::attach(storage).store(Self::STORAGE_KEY, self)
    }

    pub fn load(storage: &dyn Storage) -> StdResult<Self> {
        TypedStore::attach(storage).load(Self::STORAGE_KEY)
    }

    pub fn assert_owner(&self, info: &MessageInfo) -> StdResult<()> {
        if self.owner != info.sender {
            return Err(StdError::generic_err(
                "you are not the owner of this contract",
            ));
        }

        Ok(())
    }
}

pub struct VideoID {}

impl VideoID {
    const STORAGE_KEY: &'static [u8] = b"videos_id";

    pub fn _current(storage: &dyn Storage) -> StdResult<u64> {
        TypedStore::attach(storage).load(Self::STORAGE_KEY)
    }

    pub fn load_and_increment(storage: &mut dyn Storage) -> StdResult<u64> {
        let mut id_store = TypedStoreMut::attach(storage);
        let new_id = match id_store.may_load(Self::STORAGE_KEY)? {
            Some(id) => id + 1,
            None => 1,
        };
        id_store.store(Self::STORAGE_KEY, &new_id)?;

        Ok(new_id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct UninitializedVideo {
    pub id: u64,
    pub info: VideoInfo,
}

impl UninitializedVideo {
    const STORAGE_KEY: &'static [u8] = b"uninitialized_video";

    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        TypedStoreMut::attach_with_serialization(storage, Json).store(Self::STORAGE_KEY, self)
    }

    pub fn load(storage: &dyn Storage) -> StdResult<Self> {
        TypedStore::attach_with_serialization(storage, Json).load(Self::STORAGE_KEY)
    }

    pub fn remove(storage: &mut dyn Storage) {
        TypedStoreMut::<Self, Json>::attach_with_serialization(storage, Json)
            .remove(Self::STORAGE_KEY);
    }
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
pub struct Video {
    pub id: u64,
    pub access_token: Contract,
    pub info: VideoInfo,
}

impl Video {
    const STORAGE_KEY: &'static [u8] = b"videos";

    pub fn from_uninitialized(
        storage: &mut dyn Storage,
        access_token: Contract,
    ) -> StdResult<Self> {
        let uninitialized = UninitializedVideo::load(storage)?;
        Ok(Self {
            id: uninitialized.id,
            access_token,
            info: uninitialized.info,
        })
    }

    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        let mut videos_store = PrefixedStorage::new(storage, Self::STORAGE_KEY);
        TypedStoreMut::attach_with_serialization(&mut videos_store, Json)
            .store(&self.id.to_be_bytes(), self)
    }

    pub fn load(storage: &dyn Storage, id: u64) -> StdResult<Self> {
        let videos_store = ReadonlyPrefixedStorage::new(storage, Self::STORAGE_KEY);
        TypedStore::attach_with_serialization(&videos_store, Json).load(&id.to_be_bytes())
    }
}

impl Payment {
    const STORAGE_PREFIX: &'static [u8] = b"snip20";

    pub fn register_snip20(storage: &mut dyn Storage, address: Addr) {
        let mut snip20_store = PrefixedStorage::new(storage, Self::STORAGE_PREFIX);
        match snip20_store.get(address.as_bytes()) {
            Some(_) => {}
            None => snip20_store.set(address.as_bytes(), &[1]),
        }
    }

    pub fn is_snip20_registered(storage: &dyn Storage, address: Addr) -> bool {
        let snip20_store = ReadonlyPrefixedStorage::new(storage, Self::STORAGE_PREFIX);
        snip20_store.get(address.as_bytes()).is_some()
    }
}
