use cosmwasm_std::{DepsMut, Reply, Response, StdError, StdResult};
use cw_utils::parse_reply_instantiate_data;
use secret_toolkit::utils::types::Contract;

use crate::state::{Config, Video, VideoID};

#[derive(FromPrimitive, ToPrimitive)]
pub enum ReplyId {
    InstantiateAccessToken = 1,
}

pub fn instantiate_access_token(deps: DepsMut, reply: Reply) -> StdResult<Response> {
    let reply = parse_reply_instantiate_data(reply).map_err(|e| {
        StdError::generic_err(format!("error parsing reply error: {}", e.to_string()))
    })?;

    let config = Config::load(deps.storage)?;
    let last_id = VideoID::current(deps.storage)?;
    Video::load_and_set_token(
        deps.storage,
        last_id,
        Contract {
            address: reply.contract_address.clone(),
            hash: config.access_token_wasm.hash,
        },
    )?;

    Ok(Response::default().add_attribute_plaintext("new_video", reply.contract_address))
}
