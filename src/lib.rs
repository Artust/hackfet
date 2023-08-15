use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, AccountId};

mod user;
mod task;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub supervisors: UnorderedMap<AccountId, user::Role>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            supervisors: UnorderedMap::new(b"s"),
        }
    }
}
