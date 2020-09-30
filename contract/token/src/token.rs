/**
 * An advanced fungible token implementation.
 *
 */

use near_sdk::serde_json::{self, json};
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize};
use near_sdk::{ env, near_bindgen, ext_contract, AccountId, Balance, Promise, StorageUsage};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;

use crate::receiver::{ ext_token_receiver };
use crate::utils::{ is_promise_success };


// TODO: All gas stipends are more or less random - check througfully
const SINGLE_CALL_GAS: u64 = 200000000000000;

/**
 * A helper class to implement rollbackable promise transactions.
 */
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Ledger {

    // Total balances, including locked, for each user
    pub balances: LookupMap<AccountId, Balance>,

    /// Account has a pending promise chain in progress
    /// and balance locked is this chain cannot be withdawn.
    /// If a promise chain is succesful free the locked balance.
    /// If a promise chain fails, then the send() gets undoed
    pub locked_balances: LookupMap<AccountId, Balance>,

    /// Total supply of the token
    pub total_supply: Balance,

    /// Helper counter for testing to diagnose
    /// how many rollbacks have occured
    pub rollbacks: u64,
}


impl Ledger {

    /// Helper method to get the account details for `owner_id`.
    fn get_balance(&self, owner_id: &AccountId) -> u128 {
        match self.balances.get(owner_id) {
            Some(x) => return x,
            None => return 0,
        }
    }

    /// Helper method to set the account details for `owner_id` to the state.
    fn set_balance(&mut self, owner_id: &AccountId, balance: Balance) {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        self.balances.insert(owner_id, &balance);
    }

    /// Helper method to get the account details for `owner_id`.
    fn get_locked_balance(&self, owner_id: &AccountId) -> Balance {
        match self.locked_balances.get(owner_id) {
            Some(x) => return x,
            None => return 0,
        }
    }

    /**
     * Send tokens to a new owner.
     *
     * message is an optional byte data that is passed to the receiving smart contract.
     * notify is a flag to tell if we are going to call a smart contract, because this cannot be currently resolved run-time
     * within NEAR smart contract.
     */
    pub fn send(&mut self, owner_id: AccountId, new_owner_id: AccountId, amount: Balance, message: Vec<u8>) {

        assert!(
            env::is_valid_account_id(new_owner_id.as_bytes()),
            "New owner's account ID is invalid"
        );
        let amount = amount.into();
        if amount == 0 {
            env::panic(b"Can't transfer 0 tokens");
        }
        assert_ne!(
            owner_id, new_owner_id,
            "The new owner should be different from the current owner"
        );
        // Retrieving the account from the state.
        let source_balance = self.get_balance(&owner_id);
        let source_lock = self.get_locked_balance(&owner_id);

        // Checking and updating unlocked balance
        if source_balance < amount {
            env::panic(format!("Not enough balance, need {}, has {}", amount, source_balance).as_bytes());
        }

        // Checking and updating unlocked balance
        if source_balance < amount + source_lock {
            env::panic(format!("Cannot send {} tokens, as account has {} and in tx lock {}", amount, source_balance, source_lock).as_bytes());
        }
        self.set_balance(&owner_id, source_balance - amount);

        // Deposit amount to the new owner and save the new account to the state.
        let target_balance = self.get_balance(&new_owner_id);
        let new_target_balance = target_balance + amount;
        self.set_balance(&new_owner_id, new_target_balance);

        // This much of user balance is lockedup in promise chains
        self.set_balance(&new_owner_id, new_target_balance);

        let target_lock = self.get_locked_balance(&new_owner_id);
        self.locked_balances.insert(&new_owner_id, &(target_lock +  amount));

        let promise0 = env::promise_create(
            new_owner_id.clone(),
            b"is_receiver",
            &[],
            0,
            SINGLE_CALL_GAS/3,
        );

        let promise1 = env::promise_then(
            promise0,
            env::current_account_id(),
            b"handle_receiver",
            json!({
                "old_owner_id": owner_id,
                "new_owner_id": new_owner_id,
                "amount_received": amount.to_string(),
                "amount_total": new_target_balance.to_string(),
                "message": message,
            }).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS/3,
        );

        env::promise_return(promise1);
    }

    /// All promise chains have been successful, release balance from the lock
    /// and consider the promise chain final.
    pub fn finalise(&mut self, new_owner_id: AccountId, amount: Balance) {
        let target_lock = self.get_locked_balance(&new_owner_id);

        assert!(
            target_lock >= amount,
            "Locked balance cannot go to negative"
        );

        let new_amount = target_lock -  amount;

        self.locked_balances.insert(&new_owner_id, &new_amount);

    }

    /// Smart contract call failed. We need to roll back the balance update
    pub fn rollback(&mut self, old_owner_id: AccountId, new_owner_id: AccountId, amount: Balance) {
        let target_lock = self.get_locked_balance(&new_owner_id);
        let target_balance = self.get_balance(&new_owner_id);
        let source_balance = self.get_balance(&old_owner_id);

        env::log(format!("Rolling back back send of {}, from {} to {}, currently locked {}", amount, old_owner_id, new_owner_id, target_lock).as_bytes());
        env::log(format!("New owner balance {}, old owner balance {}", target_balance, source_balance).as_bytes());

        assert!(
            target_lock >= amount,
            "Locked balance cannot go to negative"
        );

        // Roll back lock
        let new_amount = target_lock - amount;
        self.locked_balances.insert(&new_owner_id, &new_amount);
        self.balances.insert(&new_owner_id, &new_amount);

        // Rollback new owner
        let new_target_balance = target_balance - amount;
        self.set_balance(&new_owner_id, new_target_balance);

        // Rollback old owner
        let new_source_balance = source_balance + amount;
        self.set_balance(&old_owner_id, new_source_balance);

        let target_balance = self.get_balance(&new_owner_id);
        let source_balance = self.get_balance(&old_owner_id);

        self.rollbacks += 1;
    }
}


/*
 * Information about the token.
 *
 * We hold the name, symbol and homepage readibly available on chain, but other information must be
 * from the JSON data. This way we do not bloat the chain size and also make upgrading the information
 * somewhat easier.
 *
 * All metadata fields are optional.
 */
#[derive(BorshDeserialize, BorshSerialize)]
 pub struct Metadata {

    // Name of the token
    pub name: String,

    // Symbol of the token
    pub symbol: String,

    // URL to the human readable page about the token
    pub web_link: String,

    // URL to the metadata file with more information about the token, like different icon sets
    pub metadata_link: String,
}


/**
 * Presents on token.
 */
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {

    pub ledger: Ledger,

    pub metadata: Metadata,
}


impl Default for Token {

    fn default() -> Self {
        panic!("Token should be initialized before usage")
    }
}

#[near_bindgen]
impl Token {

    /// Initializes the contract with the given total supply owned by the given `owner_id`.
    #[init]
    pub fn new(owner_id: AccountId, total_supply: Balance) -> Self {

        assert!(!env::state_exists(), "Already initialized");

        let total_supply = total_supply.into();

        // Initialize the ledger with the initial total supply
        let ledger = Ledger {
            balances: LookupMap::new(b"bal".to_vec()),
            locked_balances: LookupMap::new(b"lck".to_vec()),
            total_supply,
            rollbacks: 0,
        };

        // Currently the constructor does not support passing of metadata.
        // Start with empty metadata, owner needs to initialize this
        // after the token has been created in another transaction
        let metadata = Metadata {
            name: String::from(""),
            symbol: String::from(""),
            web_link: String::from(""),
            metadata_link: String::from(""),
        };

        let mut token = Self {
            ledger,
            metadata
        };
        token.ledger.set_balance(&owner_id, total_supply);
        return token;
    }

    /// Returns total supply of tokens.
    pub fn get_total_supply(&self) -> Balance {
        self.ledger.total_supply.into()
    }

    /// Returns balance of the `owner_id` account.
    pub fn get_balance(&self, owner_id: AccountId) -> Balance {
        self.ledger.get_balance(&owner_id).into()
    }

    /// Returns balance lockedin pending transactions
    pub fn get_locked_balance(&self, owner_id: AccountId) -> Balance {
        self.ledger.get_locked_balance(&owner_id).into()
    }

    //// How many rollbacks we have had
    pub fn get_rollback_count(&self) -> u64 {
        self.ledger.rollbacks
    }

    /// Returns balance of the `owner_id` account.
    pub fn get_name(&self) -> &str {
        return &self.metadata.name;
    }

    /// Send owner's tokens to another person or a smart contract
    #[payable]
    pub fn send(&mut self, new_owner_id: AccountId, amount: Balance, message: Vec<u8>) {
        self.ledger.send(env::predecessor_account_id(), new_owner_id, amount, message);
    }

    /**
     * After trying to call receiving smart contract if it reports it can receive tokens.
     *
     * We gpt the interface test promise back. If the account was not smart contract, finalise the transaction.
     * Otherwise trigger the smart contract notifier.
     */
    pub fn handle_receiver(&mut self, old_owner_id: AccountId, new_owner_id: AccountId, amount_received: U128, amount_total: U128, message: Vec<u8>) {
        // Only callable by self
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        env::log(b"handle_receiver reached");

        let uint_amount_received: u128 = amount_received.into();
        let uint_amount_total: u128 = amount_total.into();

        if is_promise_success() {

            // The send() was destined to a compatible receiver smart contract.
            // Build another promise that notifies the smart contract
            // that is has received new tokens.

            env::log(b"Constructing smart contract notifier promise");

            let promise0 = env::promise_create(
                new_owner_id.clone(),
                b"on_token_received",
                json!({
                    "sender_id": old_owner_id,
                    "amount_received": amount_received,
                    "amount_total": amount_total,
                    "message": message,
                }).to_string().as_bytes(),
                0,
                SINGLE_CALL_GAS/10,
            );

            // Construct the promise that calls back the
            // token contract to finalise the transaction
            let promise1 = env::promise_then(
                promise0,
                env::current_account_id(),
                b"handle_token_received",
                json!({
                    "old_owner_id": old_owner_id,
                    "new_owner_id": new_owner_id,
                    "amount_received": amount_received,
                }).to_string().as_bytes(),
                0,
                SINGLE_CALL_GAS/10,
            );

            env::promise_return(promise1);
        } else {
            // Non-code account
            // Finalise transaction now.
            self.ledger.finalise(new_owner_id, uint_amount_received);
        }
    }

    /// Smart contract notify succeed, free up any locked balance
    /// TODO: Add functionality so that the smart contract that received tokens can trigger a new promise chain here
    pub fn handle_token_received(&mut self, old_owner_id: AccountId, new_owner_id: AccountId, amount_received: U128) {
        // Only callable by self
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
        env::log(b"Checking for the need to rollback smart contract transaction");

        let amount_received: u128 = amount_received.into();

        // TODO: Have some nice error code logic here
        if is_promise_success() {
            self.ledger.finalise(new_owner_id, amount_received);
        } else {
            self.ledger.rollback(old_owner_id, new_owner_id, amount_received);
        }
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
        let contract = Token::new(bob(), total_supply.into());
        assert_eq!(contract.get_total_supply(), total_supply);
        assert_eq!(contract.get_balance(bob()), total_supply);
    }

}
