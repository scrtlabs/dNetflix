use cosmwasm_std::{to_binary, Binary, Deps, StdError, StdResult};

use crate::{
    msg::QueryAnswer,
    state::{CONFIG, VIDEOS},
};

pub fn video_info(deps: Deps, id: u64) -> StdResult<Binary> {
    let video = match VIDEOS.get(deps.storage, &id) {
        Some(v) => v,
        None => {
            return Err(StdError::generic_err(format!(
                "Video with id {} not found",
                id
            )))
        }
    };

    to_binary(&QueryAnswer::VideoInfo {
        id,
        access_token: deps.api.addr_validate(&video.access_token.address)?,
        name: video.info.name,
        royalty_info: video.info.royalty_info,
        price: video.info.price,
    })
}

pub fn owner(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;

    to_binary(&QueryAnswer::Owner {
        address: config.owner,
    })
}
