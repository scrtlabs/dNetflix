use cosmwasm_std::{
    to_binary, CosmosMsg, DepsMut, MessageInfo, Response, StdResult, SubMsg, Uint128, WasmMsg,
};

use crate::{
    reply::ReplyId,
    state::{Config, Video, VideoID, VideoInfo},
};

pub fn new_video(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    royalty_info: snip721::royalties::RoyaltyInfo,
    price: Uint128,
) -> StdResult<Response> {
    let config = Config::load(deps.storage)?;
    config.assert_owner(&info)?;

    let new_id = VideoID::load_and_increment(deps.storage)?;
    Video::new(
        new_id,
        VideoInfo {
            name: name.clone(),
            royalty_info: royalty_info.clone(),
            price,
        },
    )
    .save(deps.storage)?;

    Ok(Response::default().add_submessage(SubMsg::reply_on_success(
        CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id: config.access_token_wasm.code_id,
            code_hash: config.access_token_wasm.hash,
            msg: to_binary(&snip721::msg::InitMsg {
                name,
                symbol: "DNFLX-".to_string() + &new_id.to_string(),
                admin: None,             // Defaults to sender i.e. this contract
                entropy: "".to_string(), // Irrelevant
                royalty_info: Some(royalty_info),
                config: None,
                post_init_callback: None,
            })?,
            funds: vec![],
            label: "dNetflix-access-".to_string() + &new_id.to_string(),
        }),
        ReplyId::InstantiateAccessToken as u64,
    )))
}
