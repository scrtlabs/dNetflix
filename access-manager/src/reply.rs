use cosmwasm_std::{DepsMut, Response, StdError, StdResult, SubMsgResponse};
use secret_toolkit::utils::types::Contract;

use crate::state::{Video, CONFIG, UNINIT_VID, VIDEOS};

#[derive(FromPrimitive, ToPrimitive)]
pub enum ReplyId {
    InstantiateAccessToken = 1,
}

pub fn instantiate_access_token(deps: DepsMut, reply: SubMsgResponse) -> StdResult<Response> {
    if reply.events.is_empty() {
        return Err(StdError::generic_err(
            "Init didn't response with contract address",
        ));
    }

    if reply.events[0].attributes.is_empty() {
        return Err(StdError::generic_err(
            "Init didn't response with contract address",
        ));
    }

    if reply.events[0].attributes[0].key != "contract_address" {
        return Err(StdError::generic_err(format!(
            "Init didn't response with contract address, key was {:?}",
            reply.events[0].attributes[0].key,
        )));
    }

    let contract_address = reply.events[0].attributes[0].value.clone();
    let config = CONFIG.load(deps.storage)?;
    let video = Video::from_uninitialized(
        deps.storage,
        Contract {
            address: contract_address,
            hash: config.access_token_wasm.hash,
        },
    )?;
    VIDEOS.insert(deps.storage, &video.id, &video)?;
    UNINIT_VID.remove(deps.storage);

    Ok(Response::default().add_attribute_plaintext("new_video_id", video.id.to_string()))
}
