use cosmwasm_std::{Addr, StdResult, Storage};
use secret_toolkit::storage::{TypedStore, TypedStoreMut};
use serde::{Deserialize, Serialize};

pub const CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub owner: Addr,
}

impl Config {
    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        TypedStoreMut::attach(storage).store(CONFIG_KEY, self)
    }

    pub fn load(storage: &dyn Storage) -> StdResult<Config> {
        TypedStore::attach(storage).load(CONFIG_KEY)
    }
}
