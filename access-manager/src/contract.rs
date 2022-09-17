use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
};
use num_traits::FromPrimitive;

use crate::{
    execute,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg},
    query,
    reply::{instantiate_access_token, ReplyId},
    state::{Config, VideoInfo, CONFIG},
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // todo use entropy
    CONFIG.save(
        deps.storage,
        &Config {
            owner: info.sender,
            access_token_wasm: msg.access_token_wasm,
        },
    )?;

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
            image_url,
        } => execute::new_video(
            deps,
            info,
            env,
            VideoInfo {
                name,
                royalty_info,
                video_url,
                decryption_key,
                price,
                image_url,
            },
        ),
        ExecuteMsg::Receive {
            from, amount, msg, ..
        } => match msg.inner {
            ReceiveMsg::PurchaseVideo { video_id } => {
                let from = deps.api.addr_validate(&from)?;
                execute::purchase_video_snip20(deps, info, from, amount.u128(), video_id)
            }
        },
        ExecuteMsg::PurchaseVideo { video_id } => {
            execute::purchase_video_native(deps, info, video_id)
        }
        ExecuteMsg::WithdrawToken {
            to_address,
            token,
            amount,
        } => execute::withdraw_token(deps, info, to_address, token, amount),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VideoInfo { id } => query::video_info(deps, id),
        QueryMsg::Owner {} => query::owner(deps),
        QueryMsg::ListVideos { page, page_size } => query::list_videos(deps, page, page_size),
    }
}

#[entry_point]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> StdResult<Response> {
    match FromPrimitive::from_u64(reply.id) {
        Some(ReplyId::InstantiateAccessToken) => instantiate_access_token(deps, reply),
        None => Err(StdError::generic_err("invalid reply id")),
    }
}
