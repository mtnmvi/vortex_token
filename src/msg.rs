use cosmwasm_std::{Addr, Uint128};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub fee_percentage: u64, // Percentage of the fee to be deducted from each transfer
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    Transfer { recipient: Addr, amount: Uint128 },
    Mint { amount: Uint128 },
    Burn { amount: Uint128 },
    SetFee { fee_percentage: u64 },
    TransferOwnership { new_owner: Addr },
    WithdrawFees {},
    Pause {},
    Unpause {},
}

