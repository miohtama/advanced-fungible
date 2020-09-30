#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use near_sdk::json_types::U128;
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize};
use near_sdk::{ env, near_bindgen, AccountId, Balance, Promise };

// use nep9000_token::receiver::{ Receiver };

// ##[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;



/*
 * A simple smart contract that can receive token transfers.
 *
 * It's called burner pool, because it is one way pool, so
 * mostly useful for testing.
 *
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


/*
 * Handle incoming token transfers.
 *
 */
#[near_bindgen]
impl BurnerPool {

    // This is called by the token contract to identify us as a compatible receiver
    pub fn is_receiver() -> bool {
        env::log(b"is_receover reached");
        return true;
    }

    pub fn on_token_received(&mut self, sender_id: AccountId, amount_received: U128, amount_total: U128, message: Vec<u8>) -> Option<String> {

        assert_eq!(
            self.token_id,
            env::predecessor_account_id(),
            "Pool can only receive the named token {}, got notifier from {}",
            self.token_id, env::predecessor_account_id()
        );
        let amount: u128 = amount_received.into();
        let uint_amount_total = amount_total.into();
        self.total_received += amount;

        env::log(format!("on_token_received, incoming balance {} total {}", amount, self.total_received).as_bytes());

        assert!(self.total_received == uint_amount_total, "Mismatch between token ledger and pool balances");

        // TODO: Add error codes and graceful error handling
        return None;
    }

}

#[near_bindgen]
impl BurnerPool {

    #[init]
    pub fn new(token_id: AccountId) -> Self {

        assert!(!env::state_exists(), "Already initialized");

        assert!(
            env::is_valid_account_id(token_id.as_bytes()),
            format!("{} account ID is invalid", token_id)
        );

        let pool = Self {
            token_id: token_id,
            total_received: 0,
        };

        return pool;
    }

    pub fn get_total_received(self) -> Balance {
        return self.total_received;
    }
}

