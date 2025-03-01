use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, to_binary, CosmosMsg, Uint128};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, Cw20ReceiveMsg};
use serde::{Deserialize, Serialize};
use cosmwasm_storage::{singleton, singleton_read, Bucket};
use cw20::TokenInfo;

const TOKEN_KEY: &[u8] = b"vortex_token";

// ساختار برای اطلاعات توکن
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
}

// تابع برای مشاهده اطلاعات توکن
pub fn query_token_info(deps: DepsMut) -> StdResult<TokenInfo> {
    let info = singleton_read(deps.storage, TOKEN_KEY)?;
    Ok(info)
}

// تابع برای انتقال توکن‌ها
pub fn transfer_token(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> StdResult<Response> {
    let mut token_info = query_token_info(deps)?;
    if token_info.total_supply < amount {
        return Err(StdError::generic_err("Insufficient funds"));
    }

    // کاهش عرضه کل
    token_info.total_supply -= amount;
    singleton(deps.storage, TOKEN_KEY).save(&token_info)?;

    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: recipient,
        msg: to_binary(&Cw20ExecuteMsg::Transfer { recipient, amount })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "transfer_token")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("recipient", recipient)
        .add_attribute("amount", amount.to_string()))
}

// تابع برای مشاهده موجودی توکن‌ها
pub fn query_balance(deps: DepsMut, address: String) -> StdResult<Uint128> {
    let balance = singleton_read(deps.storage, address.as_bytes())?;
    Ok(balance)
}

// تابع برای نصب و راه‌اندازی اولیه توکن
pub fn initialize_token(deps: DepsMut, name: String, symbol: String, total_supply: Uint128) -> StdResult<Response> {
    let token_info = TokenInfo {
        name,
        symbol,
        decimals: 18,
        total_supply,
    };
    singleton(deps.storage, TOKEN_KEY).save(&token_info)?;

    Ok(Response::new().add_attribute("method", "initialize_token"))
}
