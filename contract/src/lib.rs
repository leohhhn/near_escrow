use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::{ log, near_bindgen, AccountId, env, Promise, PromiseError};

pub mod external;
pub use crate::external::*;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Escrow {
    first_token: AccountId,
    second_token: AccountId,
    escrow_balances: LookupMap<AccountId, FungibleTokenMetadata>
}

impl Default for Escrow{
    fn default() -> Self{
        Self{
            first_token: "dev-1662044518092-0000000000000".parse().unwrap(),
            second_token: "dev-1662044518092-0000000000000".parse().unwrap(),
            escrow_balances: LookupMap::new(b"m")
        }
       }
}

// Implement the contract structure
#[near_bindgen]
impl Escrow {
    pub fn get_ft(&self, account: AccountId) -> FungibleTokenMetadata {
        let ft = self.escrow_balances.get(&account).unwrap();
        ft
    }

    pub fn add_escrow_account(&mut self, ft: FungibleTokenMetadata) {
        self.escrow_balances.insert(&env::signer_account_id(), &ft);
    }

    pub fn escrow(&mut self, receiver_id: AccountId, ft_to_send: AccountId, ft_to_receive: AccountId, amount: u128) -> Promise{
        self.first_token = ft_to_send;
        self.second_token = ft_to_receive;

        log!("Current acc {}", env::current_account_id());
        log!("Signer acc {}", env::signer_account_id());
        log!("first_token acc {}", self.first_token.clone());
        log!("second_token acc {}", self.second_token.clone());

        let promise = first_token::ext(self.first_token.clone()).ft_metadata();

        //let sendToEscrow: Promise = first_token::ext(env::signer_account_id().clone())
            //.with_attached_deposit(1)
            //.ft_transfer(env::current_account_id(), amount);

        return promise.then(Self::ext(env::current_account_id()).get_metadata_callback())
    }

    #[private]
    pub fn get_metadata_callback(&self, #[callback_result] call_result: Result<FungibleTokenMetadata, PromiseError>) -> FungibleTokenMetadata {
        if call_result.is_err() {
            panic!("There was an error contacting My FT contract");
        }
    
        let metadata: FungibleTokenMetadata = call_result.unwrap();
        metadata
    }

    #[private]
    pub fn transfer_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>) -> bool {
        if call_result.is_err() {
            panic!("There was an error contacting My FT contract");
        }
    
        env::log_str("Transfering token success");
        return true
    }
 }
