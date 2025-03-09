use cosmwasm_std::Addr; // اضافه کردن واردات Addr
use cw_storage_plus::{Item}; // حذف Map به دلیل عدم استفاده
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub owner: Addr,
    pub fee_percentage: u64,
    pub paused: bool,
}

pub const STATE: Item<State> = Item::new("state");

