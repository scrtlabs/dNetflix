use cosmwasm_std::{DepsMut, Reply, Response, StdError, StdResult};
use cw_utils::parse_reply_instantiate_data;
use secret_toolkit::utils::types::Contract;

use crate::state::{Config, UninitializedVideo, Video};

#[derive(FromPrimitive, ToPrimitive)]
pub enum ReplyId {
    InstantiateAccessToken = 1,
}

pub fn instantiate_access_token(deps: DepsMut, reply: Reply) -> StdResult<Response> {
    let reply = parse_reply_instantiate_data(reply)
        .map_err(|e| StdError::generic_err(format!("error parsing reply error: {}", e)))?;

    let config = Config::load(deps.storage)?;
    let video = Video::from_uninitialized(
        deps.storage,
        Contract {
            address: reply.contract_address,
            hash: config.access_token_wasm.hash,
        },
    )?;
    video.save(deps.storage)?;
    UninitializedVideo::remove(deps.storage);

    Ok(Response::default().add_attribute_plaintext("new_video_id", video.id.to_string()))
}
