use cosmwasm_std::{
    coins, to_binary, Addr, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, SubMsg, Uint128, WasmMsg,
};
use primitive_types::U256;
use secret_toolkit::{
    snip20,
    snip721::{Authentication, Extension, MediaFile, Metadata, Mint},
    utils::types::{Contract, Token},
};

use crate::{
    constants::BLOCK_SIZE,
    reply::ReplyId,
    state::{Config, UninitializedVideo, Video, VideoID, VideoInfo},
    types::Payment,
};

// todo add a decryption key
pub fn new_video(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    video_info: VideoInfo,
) -> StdResult<Response> {
    let config = Config::load(deps.storage)?;
    config.assert_owner(&info)?;

    let new_id = VideoID::load_and_increment(deps.storage)?;
    UninitializedVideo {
        id: new_id,
        info: video_info.clone(),
    }
    .save(deps.storage)?;

    let mut response = Response::default();
    if let Token::Snip20(contract) = video_info.price.token {
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

    Ok(response.add_submessage(SubMsg::reply_on_success(
        CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id: config.access_token_wasm.code_id,
            code_hash: config.access_token_wasm.hash,
            msg: to_binary(&snip721::types::InitMsg {
                name: video_info.name,
                symbol: "DNFLX-".to_string() + &new_id.to_string(),
                admin: None,             // Defaults to sender i.e. this contract
                entropy: "".to_string(), // todo fix
                royalty_info: Some(video_info.royalty_info),
                config: None,
                post_init_callback: None,
            })?,
            funds: vec![],
            label: "dNetflix-access-".to_string() + &new_id.to_string(),
        }),
        ReplyId::InstantiateAccessToken as u64,
    )))
}

pub fn purchase_video_snip20(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    _sender: Addr,
    from: Addr,
    amount: u128,
    video_id: u64,
) -> StdResult<Response> {
    let video = Video::load(deps.storage, video_id)?;

    // Validate payment method
    if video.info.price.amount.u128() != amount {
        return Err(StdError::generic_err("invalid amount"));
    }
    let royalty_distribution = if let Token::Snip20(contract) = &video.info.price.token {
        let payment_address = deps.api.addr_validate(&contract.address)?;
        if payment_address != info.sender {
            Err(StdError::generic_err("invalid payment method"))
        } else {
            create_royalty_distribution_snip20(&video.info.royalty_info, amount, contract)
        }
    } else {
        Err(StdError::generic_err("invalid payment method"))
    }?;

    Ok(purchase_video_impl(&video, &from)?.add_messages(royalty_distribution))
}

pub fn purchase_video_native(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    video_id: u64,
) -> StdResult<Response> {
    let video = Video::load(deps.storage, video_id)?;

    let royalty_distribution = if let Token::Native(denom) = &video.info.price.token {
        let payment = info.funds.iter().find(|c| c.denom == *denom);
        if let Some(payment) = payment {
            if payment.amount != video.info.price.amount {
                Err(StdError::generic_err("invalid amount"))
            } else {
                Ok(create_royalty_distribution_native(
                    &video.info.royalty_info,
                    payment.amount.u128(),
                    denom,
                ))
            }
        } else {
            Err(StdError::generic_err("invalid payment method"))
        }
    } else {
        Err(StdError::generic_err("invalid payment method"))
    }?;

    Ok(purchase_video_impl(&video, &info.sender)?.add_messages(royalty_distribution))
}

// todo put the create_royalty_distribution() functions in a more appropriate place
// (e.g. in as impl{} block of RoylatyInfo)
fn create_royalty_distribution_snip20(
    royalties: &snip721::types::RoyaltyInfo,
    amount: u128,
    token: &Contract,
) -> StdResult<Vec<CosmosMsg>> {
    let mut messages = vec![];
    for royalty in &royalties.royalties {
        let amount = U256::from(amount) * U256::from(royalty.rate)
            / U256::from(10u128).pow(U256::from(royalties.decimal_places_in_rates));
        messages.push(snip20::send_msg(
            royalty.recipient.to_string(),
            Uint128::from(amount.as_u128()),
            None,
            None,
            None,
            BLOCK_SIZE,
            token.hash.clone(),
            token.address.clone(),
        )?);
    }

    Ok(messages)
}

fn create_royalty_distribution_native(
    royalties: &snip721::types::RoyaltyInfo,
    amount: u128,
    denom: &String,
) -> Vec<CosmosMsg> {
    let mut messages = vec![];
    for royalty in &royalties.royalties {
        let amount = U256::from(amount) * U256::from(royalty.rate)
            / U256::from(10u128).pow(U256::from(royalties.decimal_places_in_rates));
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: royalty.recipient.to_string(),
            amount: coins(amount.as_u128(), denom),
        }));
    }

    messages
}

fn purchase_video_impl(video: &Video, purchaser: &Addr) -> StdResult<Response> {
    Ok(
        Response::default().add_message(secret_toolkit::snip721::batch_mint_nft_msg(
            vec![Mint {
                token_id: None,
                owner: Some(purchaser.into()),
                public_metadata: Some(Metadata {
                    token_uri: None,
                    extension: None,
                }),
                private_metadata: Some(Metadata {
                    token_uri: None,
                    extension: Some(Extension {
                        image: None,
                        image_data: None,
                        external_url: None,
                        description: None,
                        name: None,
                        attributes: None,
                        background_color: None,
                        animation_url: None,
                        youtube_url: None,
                        media: Some(vec![MediaFile {
                            file_type: Some("video".to_string()),
                            extension: Some("mp4".to_string()),
                            authentication: Some(Authentication {
                                key: Some(video.info.decryption_key.clone()),
                                user: None,
                            }),
                            url: video.info.video_url.clone(),
                        }]),
                        protected_attributes: None,
                    }),
                }),
                memo: None,
            }],
            None,
            BLOCK_SIZE,
            video.access_token.hash.clone(),
            video.access_token.address.clone(),
        )?),
    )
}
