use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, env, PromiseError};

pub mod external;
pub use crate::external::*;

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello From Cross Communication Contract";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    message: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{message: DEFAULT_MESSAGE.to_string()}
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {

    pub fn ft_on_transfer(&mut self, _sender_id: String, amount: String, _memo: Option<String>, _msg: String) -> String {
        log!("Called ft_on_transfer {}", env::current_account_id());
        let _escrow_contract = env::current_account_id();
        
        // let balance_of_first_token = ft::ext(sender_id.parse().unwrap())
        //         .ft_balance_of(escrow_contract.clone())
        //     .then(Self::ext(escrow_contract.clone())
        //             .get_balance_of_callback()
        // );
        
        // let balance_of_second_token = ft::ext(sender_id.parse().unwrap())
        //         .ft_balance_of(escrow_contract.clone())
        //     .then(Self::ext(escrow_contract.clone())
        //             .get_balance_of_callback()
        // );
        //log!("Balance of first token: {} is {:?}", sender_id, balance_of_first_token);    

        amount
    }

    #[private]
    pub fn get_balance_of_callback(&self, #[callback_result] call_result: Result<String, PromiseError>) -> String {
        if call_result.is_err() {
            panic!("There was an error contacting My FT contract");
        }
    
        let balance: String = call_result.unwrap();
        balance
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
}
