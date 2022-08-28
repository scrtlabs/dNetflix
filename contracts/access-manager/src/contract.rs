use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
};
use num_traits::FromPrimitive;

use crate::{
    execute::new_video,
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
        } => new_video(deps, info, env, name, royalty_info, price),
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
