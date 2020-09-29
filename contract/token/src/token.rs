/**
 * An advanced fungible token.
 *
 * https://github.com/near/near-sdk-rs/blob/master/examples/fungible-token/src/lib.rs
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize};
use near_sdk::{ env, near_bindgen, ext_contract, AccountId, Balance, Promise, StorageUsage};
use near_sdk::collections::LookupMap;

// Prepaid gas for making a single simple call.
const SINGLE_CALL_GAS: u64 = 200000000000000;

/**
 * Hold accounting data for one token.
 */
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Ledger {

    /// sha256(AccountID) -> Account details.
    pub balances: LookupMap<AccountId, Balance>,

    /// Total supply of the all token.
    pub total_supply: Balance,
}


#[ext_contract(token_receiver)]
pub trait ExtTokenReceiver {

    // Resolves to None on successful call, an error message on failure
    fn process_token_received(&self, sender_id: AccountId, amount: Balance, message: Vec<u8>) -> Option<String>;
}


impl Ledger {

    /// Helper method to get the account details for `owner_id`.
    fn get_balance(&self, owner_id: &AccountId) -> u128 {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        match self.balances.get(owner_id) {
            Some(x) => return x,
            None => return 0,
        }
    }

    /// Helper method to set the account details for `owner_id` to the state.
    fn set_balance(&mut self, owner_id: &AccountId, balance: Balance) {
        self.balances.insert(owner_id, &balance);
    }

    /// Transfers the `amount` of tokens from `owner_id` to the `new_owner_id`.
    /// Requirements:
    /// * `amount` should be a positive integer.
    /// * `owner_id` should have balance on the account greater or equal than the transfer `amount`.
    /// * If this function is called by an escrow account (`owner_id != predecessor_account_id`),
    ///   then the allowance of the caller of the function (`predecessor_account_id`) on
    ///   the account of `owner_id` should be greater or equal than the transfer `amount`.
    /// * Caller of the method has to attach deposit enough to cover storage difference at the
    ///   fixed storage price defined in the contract.
    pub fn transfer(&mut self, owner_id: AccountId, new_owner_id: AccountId, amount: Balance, message: Vec<u8>, notify: bool) {
        // let initial_storage = env::storage_usage();
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

        // Checking and updating unlocked balance
        if source_balance < amount {
            env::panic(b"Not enough balance");
        }
        self.set_balance(&owner_id, source_balance - amount);

        // Deposit amount to the new owner and save the new account to the state.
        let target_balance = self.get_balance(&new_owner_id);
        let new_target_balance = target_balance + amount;
        self.set_balance(&new_owner_id, new_target_balance);

        self.notify_receiver(new_owner_id, amount, new_target_balance);
    }

    fn notify_receiver(&mut self, new_owner_id: AccountId, amount_received: Balance, amount_total: Balance) {
        //token_receiver::process_token_receiver(amount_received, amount_total, [], &new_owner_id, 0, SINGLE_CALL_GAS);
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
            balances: LookupMap::new(b"a".to_vec()),
            total_supply,
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

    /// Returns balance of the `owner_id` account.
    pub fn get_name(&self) -> &str {
        return &self.metadata.name;
    }

    #[payable]
    pub fn transfer(&mut self, new_owner_id: AccountId, amount: Balance, message: Vec<u8>, notify: bool) {
        self.ledger.transfer(env::predecessor_account_id(), new_owner_id, amount, message, notify);
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

    // Test sending between two normal (no code accounts)
    #[test]
    fn test_send_normal() {
        let context = get_context(alice());
        testing_env!(context);
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Token::new(alice(), total_supply.into());
        let amount = 5_000u128;

        // Context has the sender account as alice
        contract.transfer(bob(), amount, vec![], false);

        assert_eq!(contract.get_balance(alice()), total_supply - amount);
        assert_eq!(contract.get_balance(bob()), amount);
    }

    #[test]
    fn test_send_smart_contract() {
        let context = get_context(alice());
        testing_env!(context);
        let total_supply = 1_000_000_000_000_000u128;
        let mut contract = Token::new(alice(), total_supply.into());
        let amount = 5_000u128;

        // Context has the sender account as alice
        contract.transfer(bob(), amount, vec![], false);

        assert_eq!(contract.get_balance(alice()), total_supply - amount);
        assert_eq!(contract.get_balance(bob()), amount);
    }
}
