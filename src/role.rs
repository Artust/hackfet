use crate::Contract;
use crate::ContractExt;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::serde::ser::SerializeSeq;
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen, AccountId};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Supervisors {
    pub admins: Vec<User>,
    pub inspectors: Vec<User>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    pub account_id: AccountId,
    pub role: Role,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Role {
    Root,
    Admin,
    Inspector,
    User,
}

#[near_bindgen]
impl Contract {
    // #[private]
    // pub fn add_account(&mut self, role: String, account_supervisor: String) {
    //     let account_id = match AccountId::try_from(account_supervisor) {
    //         Ok(account_id) => account_id,
    //         Err(error) => {
    //             println!("Error parse accountId: {:?}", error);
    //             return;
    //         }
    //     };
    //     log!("Account: {}", account_id);
    //     match role.as_str() {
    //         "Admin" => {
    //             let user = User {
    //                 role: Role::Admin,
    //                 account_id: account_id,
    //             };
    //             self.supervisor.admins.push(&user)
    //         }
    //         "Inspector" => {
    //             let user = User {
    //                 role: Role::Inspector,
    //                 account_id: account_id,
    //             };
    //             self.supervisor.inspectors.push(&user)
    //         }
    //         _ => {
    //             println!("Unknown role: {}", role);
    //             return; // Early return if role is unknown
    //         }
    //     };
    // }

    pub fn get_supervisors(&mut self) -> Supervisors {
        // self.supervisors.admins.push(User { account_id: "test".parse().unwrap(), role: Role::Admin })
        let clone_admins = self.supervisors.admins.to_vec();
        let clone_inspectors = self.supervisors.inspectors.to_vec();
        Supervisors {
            admins: clone_admins,
            inspectors: clone_inspectors,
        }
    }

    pub fn get_contract_address() -> AccountId {
        env::current_account_id()
    }
}
