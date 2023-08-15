use crate::Contract;
use crate::ContractExt;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Gas;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Promise};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Task {
    pub id: usize,
    pub link: String,
    pub description: String,
    pub owner: AccountId,
    pub developer: Developer,
    pub inspector: Inspector,
    pub investment: Investment,
    pub contract_bond: U128,
    pub status: TaskStatus,
    pub grey_list: Vec<AccountId>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Developer {
    pub account_id: AccountId,
    pub approve: bool,
    pub process: Process,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Process {
    Unassigned,
    Todo,
    InProgress,
    WaitReview,
    Done,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Inspector {
    pub account_id: AccountId,
    pub verify: VerifyStatus,
    pub comment: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum VerifyStatus {
    None,
    Reject,
    Done,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Investment {
    pub inspection: U128,
    pub development: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum TaskStatus {
    Open,
    Assigned,
    Bid,
    Verified,
    Close,
}
