use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ env, near_bindgen, AccountId, Balance, Promise, StorageUsage};
use near_sdk::collections::LookupMap;


/*
 * A simple smart contract that can receive token transfers.
 */
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Escrow {

    // Which token this escrow contract is for
    pub token_id: AccountId,

    // How many tokens the contract has received overall
    pub total_received: Balance,

    // How many tokens we have send out as in fees
    pub total_fees: Balance,

    /// sha256(AccountID) -> Account details.
    pub balances: LookupMap<Vec<u8>, Balance>,

}


impl Default for Escrow {

    fn default() -> Self {
        panic!("Contract should be initialized before usage")
    }
}


impl Escrow {

    /// Initializes the contract with the given total supply owned by the given `owner_id`.
    pub fn new(token_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut escrow = Self {
            token_id,
            total_received: 0,
            total_fees: 0,
            balances: LookupMap::new(b"a".to_vec())
        };
        return escrow;
    }

    /// Helper method to get the account details for `owner_id`.
    fn get_balance(&self, owner_id: &AccountId) -> u128 {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        let account_hash = env::sha256(owner_id.as_bytes());
        match self.balances.get(&account_hash) {
            Some(x) => return x,
            None => return 0,
        }
    }

    /// Helper method to set the account details for `owner_id` to the state.
    fn set_balance(&mut self, owner_id: &AccountId, balance: Balance) {
        let account_hash = env::sha256(owner_id.as_bytes());
        self.balances.insert(&account_hash, &balance);
    }

    /*
     * processtokenReceived() callback is called after the token send() is complete.
     *
     * This simulates a vulnerable contract that would be subject to re-entrancy attack.
     */
    pub fn process_token_received(&mut self, sender_id: AccountId, amount_received: Balance, amount_total: Balance, message: Vec<u8>) {
        // This call needs to come from a supported smart contract
        let sender_id = env::predecessor_account_id();
        if sender_id != self.token_id  {
            env::panic(b"Unsupported token received");
        }

        // A constant fee reduced from the transfer
        let fee = 100u128;

        let escrowed_amount = amount_received - fee;

        self.set_balance(&sender_id, self.get_balance(&sender_id) + escrowed_amount);

        self.total_received += amount_received;
        self.total_fees += fee;
    }

}