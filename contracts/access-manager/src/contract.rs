use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
};
use num_traits::FromPrimitive;

use crate::{
    execute::{new_video, purchase_video_native, purchase_video_snip20},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    reply::{instantiate_access_token, ReplyId},
    state::Config,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    Config {
        owner: info.sender,
        access_token_wasm: msg.access_token_wasm,
    }
    .save(deps.storage)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::NewVideo {
            name,
            royalty_info,
            price,
            video_url,
            decryption_key,
        } => new_video(
            deps,
            info,
            env,
            name,
            royalty_info,
            video_url,
            decryption_key,
            price,
        ),
        ExecuteMsg::Receive {
            sender,
            from,
            amount,
            msg,
        } => match msg.inner {
            crate::msg::ReceiveMsg::PurchaseVideo { video_id } => {
                let sender = deps.api.addr_validate(&sender)?;
                let from = deps.api.addr_validate(&from)?;
                purchase_video_snip20(deps, info, env, sender, from, amount.u128(), video_id)
            }
        },
        ExecuteMsg::PurchaseVideo { video_id } => purchase_video_native(deps, info, env, video_id),
    }
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[entry_point]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> StdResult<Response> {
    match FromPrimitive::from_u64(reply.id) {
        Some(ReplyId::InstantiateAccessToken) => instantiate_access_token(deps, reply),
        None => Err(StdError::generic_err("invalid reply id")),
    }
}
