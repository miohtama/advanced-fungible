use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::{ env, near_bindgen, AccountId, Balance, Promise };

use crate::receiver::{ Receiver };
use crate::token::{ Token };

/*
 * A simple smart contract that can receive token transfers.
 */
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BurnerPool {

    // Which token this escrow contract is for
    pub token_id: AccountId,

    // How many tokens the contract has received overall
    pub total_received: Balance,

}


impl Default for BurnerPool {

    fn default() -> Self {
        panic!("Contract should be initialized before usage")
    }
}


#[near_bindgen]
impl Receiver for BurnerPool {

    fn on_token_received(&mut self, sender_id: AccountId, amount: Balance, _message: Vec<u8>) -> Option<String> {
        assert!(sender_id == self.token_id, "Pool can only receive the named token");

        self.total_received += amount;

        // This transfer can never fail
        return None;
    }

}

#[near_bindgen]
impl BurnerPool {

    #[init]
    fn new(token_id: AccountId) -> Self {

        assert!(!env::state_exists(), "Already initialized");

        let pool = Self {
            token_id: token_id,
            total_received: 0,
        };

        return pool;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn alice() -> AccountId {
        "alice.near".to_string()
    }

    fn bob() -> AccountId {
        "bob.near".to_string()
    }

    fn carol() -> AccountId {
        "carol.near".to_string()
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContext {
            current_account_id: alice(),
            signer_account_id: bob(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 1_000_000_000_000_000_000_000_000_000u128,
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn test_new() {

        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 1_000_000_000_000_000u128;
        let token = Token::new(bob(), total_supply.into());

    }
}
