use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::{UnorderedMap};

mod send_coin;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub beneficiary: AccountId,
  pub send_coins: UnorderedMap<AccountId, u128>,
}

impl Default for Contract {
  fn default() -> Self {
    Self{
      beneficiary: "v1.faucet.nonofficial.testnet".parse().unwrap(),
      send_coins: UnorderedMap::new(b"d"),
    }
  }
}

#[near_bindgen]
impl Contract {
  #[init]
  #[private] // Public - but only callable by env::current_account_id()
  pub fn init(beneficiary: AccountId) -> Self {
    Self {
      beneficiary,
      send_coins: UnorderedMap::new(b"d"),
    }
  }

  // Public - beneficiary getter
  pub fn get_beneficiary(&self) -> AccountId {
    self.beneficiary.clone()
  }

  // Public - but only callable by env::current_account_id(). Sets the beneficiary
  #[private]
  pub fn change_beneficiary(&mut self, beneficiary: AccountId) {
    self.beneficiary = beneficiary;
  }
}
