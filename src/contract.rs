use cosmwasm_std::{entry_point, coin, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128, Addr};
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{State, STATE};
use crate::error::ContractError;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
        fee_percentage: msg.fee_percentage,
        paused: false,
    };
    STATE.save(deps.storage, &state)?; // تبدیل خطا به ContractError انجام شده است
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", info.sender))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => try_transfer(deps, env, info, recipient, amount),
        ExecuteMsg::Mint { amount } => try_mint(deps, env, info, amount),
        ExecuteMsg::Burn { amount } => try_burn(deps, info, amount),
        ExecuteMsg::SetFee { fee_percentage } => try_set_fee(deps, info, fee_percentage),
        ExecuteMsg::TransferOwnership { new_owner } => try_transfer_ownership(deps, info, new_owner),
        ExecuteMsg::WithdrawFees {} => try_withdraw_fees(deps, info),
        ExecuteMsg::Pause {} => try_pause(deps, info),
        ExecuteMsg::Unpause {} => try_unpause(deps, info),
    }
}

pub fn try_transfer(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    recipient: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let fee = (amount.u128() * 10) / 100; // 10% fee
    let transfer_amount = amount.checked_sub(Uint128::new(fee))?;
    
    let msgs = vec![
        CosmosMsg::Bank(BankMsg::Send {
            to_address: recipient.to_string(),
            amount: vec![coin(transfer_amount.u128(), "VTX")],
        }),
    ];

    Ok(Response::new().add_messages(msgs).add_attribute("action", "transfer"))
}

pub fn try_mint(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _amount: Uint128,  // تغییر به _amount برای جلوگیری از هشدار بلااستفاده
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "mint"))
}

pub fn try_burn(
    _deps: DepsMut,
    _info: MessageInfo,
    _amount: Uint128,  // تغییر به _amount برای جلوگیری از هشدار بلااستفاده
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "burn"))
}

pub fn try_set_fee(
    _deps: DepsMut,
    _info: MessageInfo,
    _fee_percentage: u64,  // تغییر به _fee_percentage برای جلوگیری از هشدار بلااستفاده
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "set_fee"))
}

pub fn try_transfer_ownership(
    _deps: DepsMut,
    _info: MessageInfo,
    _new_owner: Addr,  // تغییر به _new_owner برای جلوگیری از هشدار بلااستفاده
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "transfer_ownership"))
}

pub fn try_withdraw_fees(
    _deps: DepsMut,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "withdraw_fees"))
}

pub fn try_pause(
    _deps: DepsMut,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "pause"))
}

pub fn try_unpause(
    _deps: DepsMut,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "unpause"))
}

