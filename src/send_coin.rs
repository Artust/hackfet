use crate::Contract;
use crate::ContractExt;

use near_sdk::serde::Serialize;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Promise, Balance};
use near_sdk::json_types::U128;

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SendCoin {
  pub account_id: AccountId, 
  pub total_amount: U128,
}

#[near_bindgen]
impl Contract {
  #[payable] // Public - People can attach money
  pub fn send_coin(&mut self) -> U128 {
    // Get who is calling the method and how much $NEAR they attached
    let sender: AccountId = env::predecessor_account_id();
    let send_amount: Balance = env::attached_deposit();

    let mut sent_so_far = self.send_coins.get(&sender).unwrap_or(0);

    let to_transfer: Balance = if sent_so_far == 0 {
      // This is the user's first send, lets register it, which increases storage
      assert!(send_amount > STORAGE_COST, "Attach at least {} yoctoNEAR when send amount is {}", STORAGE_COST, send_amount);

      // Subtract the storage cost to the amount to transfer
      send_amount - STORAGE_COST
    }else{
      send_amount
    };

    // Persist in storage the amount sent so far
    sent_so_far += send_amount;
    self.send_coins.insert(&sender, &sent_so_far);
    
    log!("Thank you {} for senting {}! You sent a total of {}", sender.clone(), send_amount, sent_so_far);
    
    // Send the money to the beneficiary
    Promise::new(self.beneficiary.clone()).transfer(to_transfer);

    // Return the total amount sent so far
    U128(sent_so_far)
  }

  // Public - get send by account ID
  pub fn get_send_for_account(&self, account_id: AccountId) -> SendCoin {
    SendCoin {
      account_id: account_id.clone(),
      total_amount: U128(self.send_coins.get(&account_id).unwrap_or(0))
    }
  }

  // Public - get total number of senders
  pub fn number_of_senders(&self) -> u64 {
    self.send_coins.len()
  }

  // Public - paginate through all send_coins on the contract
  pub fn get_send_coins(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<SendCoin> {
    //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
    let start = u128::from(from_index.unwrap_or(U128(0)));

    //iterate through send
    self.send_coins.keys()
      //skip to the index we specified in the start variable
      .skip(start as usize) 
      //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
      .take(limit.unwrap_or(50) as usize) 
      .map(|account| self.get_send_for_account(account))
      //since we turned map into an iterator, we need to turn it back into a vector to return
      .collect()
  }
}