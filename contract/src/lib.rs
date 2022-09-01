use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::{ near_bindgen, AccountId, env};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Escrow {
    escrow_balances: LookupMap<AccountId, FungibleToken>
}

impl Default for Escrow{
    fn default() -> Self{
        Self{
            escrow_balances: LookupMap::new(b"m")
        }
       }
}

// Implement the contract structure
#[near_bindgen]
impl Escrow {
    pub fn get_ft(&self, account: AccountId) -> FungibleToken {
        let ft = self.escrow_balances.get(&account).unwrap();
        ft
    }

    pub fn add_escrow_account(&mut self, ft: FungibleToken) {
        self.escrow_balances.insert(&env::signer_account_id(), &ft);

    }
}
