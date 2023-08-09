use crate::Contract;
use crate::ContractExt;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Promise};

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Task {
    pub id: u128,
    pub link: String,
    pub description: String,
    pub owner: AccountId,
    pub developer: Developer,
    pub inspector: Inspector,
    pub investment: Investment,
    pub contract_bond: u128,
    pub status: TaskStatus,
}

pub struct Developer {
    pub account_id: AccountId,
    pub is_bid: bool,
    pub approve: bool,
    pub process: DevelopmentStatus,
}

pub struct Inspector {
    pub account_id: AccountId,
    pub verify: VerifyStatus,
    pub comment: String,
}

pub struct Investment {
    pub inspection: u128,
    pub development: u128,
}

enum DevelopmentStatus {
    Todo,
    InProgress,
    WaitReview,
    Done,
}

enum VerifyStatus {
    None,
    Reject,
    Pass,
}

enum TaskStatus {
    Open,
    Bidding,
    Assigned,
    Verified,
    Closed,
}
