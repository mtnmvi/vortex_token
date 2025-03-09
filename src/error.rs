use cosmwasm_std::{StdError, OverflowError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError), // تبدیل StdError به ContractError

    #[error("{0}")]
    OverflowError(#[from] OverflowError), // تبدیل OverflowError به ContractError
}

