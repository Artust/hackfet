use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::Vector;
use near_sdk::{env, log, near_bindgen, AccountId};

mod role;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub supervisors: role::Supervisors,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            supervisors: role::Supervisors {
                admins: Vec::new(),
                inspectors: Vec::new()
            },
        }
    }
}
