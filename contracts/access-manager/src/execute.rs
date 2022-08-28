use cosmwasm_std::{
    to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg, WasmMsg,
};
use secret_toolkit::snip20;

use crate::{
    reply::ReplyId,
    state::{Config, UninitializedVideo, VideoID, VideoInfo},
    types::Payment,
};

pub fn new_video(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    name: String,
    royalty_info: snip721::royalties::RoyaltyInfo,
    price: Payment,
) -> StdResult<Response> {
    let config = Config::load(deps.storage)?;
    config.assert_owner(&info)?;

    let new_id = VideoID::load_and_increment(deps.storage)?;
    UninitializedVideo {
        id: new_id,
        info: VideoInfo {
            name: name.clone(),
            royalty_info: royalty_info.clone(),
            price: price.clone(),
        },
    }
    .save(deps.storage)?;

    let mut response = Response::default();
    match price.token {
        secret_toolkit::utils::types::Token::Snip20(contract) => {
            let address = deps.api.addr_validate(&contract.address)?;

            if !Payment::is_snip20_registered(deps.storage, address.clone()) {
                Payment::register_snip20(deps.storage, address);

                response
                    .messages
                    .push(SubMsg::new(snip20::register_receive_msg(
                        env.contract.code_hash,
                        None, // No need for padding here since it's going to be public anyway
                        1,    // No need for padding here since it's going to be public anyway
                        contract.hash,
                        contract.address,
                    )?));
            }
        }
        secret_toolkit::utils::types::Token::Native(_) => {}
    }

    Ok(response.add_submessage(SubMsg::reply_on_success(
        CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id: config.access_token_wasm.code_id,
            code_hash: config.access_token_wasm.hash,
            msg: to_binary(&snip721::msg::InitMsg {
                name,
                symbol: "DNFLX-".to_string() + &new_id.to_string(),
                admin: None,             // Defaults to sender i.e. this contract
                entropy: "".to_string(), // todo fix
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
