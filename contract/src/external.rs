use near_sdk::{ext_contract, AccountId};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;

#[ext_contract(this_contract)]
trait Callbacks {
  fn get_metadata_callback(&self) -> FungibleTokenMetadata;
  fn transfer_callback(&mut self) -> bool;
}

#[ext_contract(first_token)]
trait FirstToken {
  fn ft_metadata(&self) -> FungibleTokenMetadata;
  fn ft_transfer(&self, receiver_id: AccountId, amount: u128) -> bool;
}

#[ext_contract(second_token)]
trait SecondToken {
  fn ft_metadata(&self) -> FungibleTokenMetadata;
  fn ft_transfer(&self, receiver_id: AccountId, amount: u128) -> bool;
}