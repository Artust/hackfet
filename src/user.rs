use crate::Contract;
use crate::ContractExt;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{log, near_bindgen, AccountId};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Supervisor {
    pub account_id: AccountId,
    pub role: Role,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Role {
    Admin,
    Inspector,
    User,
}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn add_supervisor(&mut self, account: String, role: String) {
        let account_id = match AccountId::try_from(account) {
            Ok(result) => result,
            Err(err) => {
                log!("Error parse account_id: {}", err);
                return;
            }
        };
        match role.as_str() {
            "Admin" => {
                log!("Add supervisor {} with role {:?}", account_id, Role::Admin);
                self.supervisors.insert(&account_id, &Role::Admin);
            }
            "Inspector" => {
                log!(
                    "Add supervisor {} with role {:?}",
                    account_id,
                    Role::Inspector
                );
                self.supervisors.insert(&account_id, &Role::Inspector);
            }
            _ => {
                // Handle the case when the input string doesn't match any variant
                log!("Unknown role: {}", role);
            }
        }
    }

    pub fn get_role_for_account(&self, account_id: AccountId) -> Supervisor {
        Supervisor {
            account_id: account_id.clone(),
            role: self.supervisors.get(&account_id).unwrap_or(Role::User),
        }
    }

    pub fn get_supervisors(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Supervisor> {
        let start = u128::from(from_index.unwrap_or(U128(0)));
        self.supervisors
            .keys()
            .skip(start as usize)
            .take(limit.unwrap_or(10) as usize)
            .map(|account| self.get_role_for_account(account))
            .collect()
    }
}
