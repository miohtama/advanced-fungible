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


}


impl Default for Escrow {

    fn default() -> Self {
        panic!("Contract should be initialized before usage")
    }
}


impl Escrow {


}