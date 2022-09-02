use near_sdk::{ext_contract, AccountId};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;

#[ext_contract(this_contract)]
trait Callbacks {
  fn get_metadata_callback(&self) -> FungibleTokenMetadata;
  fn get_balance_of_callback(&self) -> String;
  fn on_transfer(&mut self, owner_id: AccountId, token_contract: AccountId, amount: String) -> bool;
}

#[ext_contract(ft)]
trait FT {
  fn ft_balance_of(&self, account_id: AccountId) -> String;
  fn transfer_from(&self, owner_id: AccountId, new_owner_id: AccountId, amount: String) -> Result<(), &'static str>;
}