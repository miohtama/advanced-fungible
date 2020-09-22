/**
 * An advanced fungible token.
 *
 * https://github.com/near/near-sdk-rs/blob/master/examples/fungible-token/src/lib.rs
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::{ env, near_bindgen, AccountId, Balance, Promise, StorageUsage};
use near_sdk::collections::LookupMap;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/**
 * Hold accounting data for one token.
 */
pub struct Ledger {

    /// sha256(AccountID) -> Account details.
    pub balances: LookupMap<Vec<u8>, Balance>,

    /// Total supply of the all token.
    pub total_supply: Balance,
}

impl Ledger {

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
    fn set_balance(&mut self, owner_id: &AccountId, balance: &Balance) {
        let account_hash = env::sha256(owner_id.as_bytes());
        self.balances.insert(&account_hash, &balance);
    }

    fn refund_storage(&self) {
        // TODO
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
 pub struct Metadata {

    // Name of the token
    pub name: String,

    // Symbol of the token
    pub symbol: String,

    // URL to the human readable page about the token
    pub webLink: String,

    // URL to the metadata file with more information about the token, like different icon sets
    pub metadataLink: String,
}


/**
 * Presents on token.
 */
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
            webLink: String::from(""),
            metadataLink: String::from(""),
        };

        let mut token = Self {
            ledger,
            metadata
        };
        token.ledger.set_balance(&owner_id, &total_supply);
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
