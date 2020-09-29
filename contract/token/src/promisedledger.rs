use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ env, near_bindgen, AccountId, Balance, Promise, StorageUsage};
use near_sdk::collections::LookupMap;



struct PromisedLock {
    owner_id: AccountId,
    amount: Balance,
}

/**
 * Mikko's Promised Ledger.
 *
 * An accounting data structure that is friendly for NEAR sharded blockchain.
 *
 * This structure is used by the token itself and all smart contracts
 * that send and receive user balance. It allows easy handling
 * of "future promised balances". Promised balances are spendable
 * only by the transaction promise chain itself, not other transactions.
 * This protects against re-entrancy attacks, while still allowing
 * handling of multiple balance transfers in a single transaction.
 *
 *
 * TODO: Convert AccountId to more efficient data type like sha256.
 *
 */
 pub struct PromisedLedger {

    /// sha256(AccountID) -> token balance
    /// This is the actual balances and none of the pending transactions
    pub balances: LookupMap <AccountId, Balance>,

    /// txid -> pending token balance
    /// This is the actual balances and none of the pending transactions
    pub pending_balances: LookupMap <LockupId, Lockup>,

    /// Amount of tokens this ledger has available for total withdraw
    pub total_amount: Balance,

    pub pending_amount: Balance,

    pub next_lockup_id,

}

pub trait Sendable {

    // Resolves to None on successful call, an error message on failure
    fn process_token_received(&self, sender_id: AccountId, amount: Balance, message: Vec<u8>) -> Option<String>;
}


impl PromisedLedger {

    /// Helper method to get the account details for `owner_id`.
    fn get_balance(&self, owner_id: &AccountId) -> u128 {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        match self.balances.get(&owner_id) {
            Some(x) => return x,
            None => return 0,
        }
    }

    /// Helper method to set the account details for `owner_id` to the state.
    fn set_balance(&mut self, owner_id: &AccountId, balance: Balance) {
        self.balances.insert(&owner_id, &balance);
    }

    // Create new balance out of nothing
    fn receive_inflow(&mut self, owner_id: &AccountId, amount: Balance) {
        let balance = self.get_balance(owner_id);
        set_balance(owner_id, balance + amount);
        self.total_supply += balance;
    }

    // Smart contrct that maintains balance of received tokens want to make withdraw
    fn send_further(&mut self, owner_id: &AccountId, amount: Balance): LockupId  {

    }

    /*
     *
     * Because of the currebt NEAR limitation of not being able to secure check parallel
     * promise chains, we limit it so that only one promise chain per account
     * can be activate at the time.
     *
     */
    fn lock_up_for_promise(&mut self, promise_id: PromiseIndex, owner_id: &AccountId, amount: Balance): LockupId  {
        assert!(self.pending_balances.get(promise_id) == 0, "Promise lockup already in progress");
        assert!(promise_id != 0, "PromiseIndex is bad");

        let lockup = Lockup {
            owner_id,
            amount
        }

        let lockup_id = self.next_lockup_id++;
        self.pending_balances.insert(self.next_lockup_id, lockup);
        self.pending_amount += amount;
        return lockup_id;
    }

    fn resolve_promise(&mut self, lockup_id: LockupId, allow_failure: bool) {
        if allow_failure {
            finalize(lockup_id);
        } else {
            if is_promise_success() {
                finalize(lockup_id);
            } else {
                rollback(lockup_id);
            }
        }
        self.pending_balances.remove(lockup_id);
    }

    fn finalize(&mut self, lockup_id: LockupId) {
        assert!(self.pending_balances.get(lockup_id) > 0, "Promise lockup not in progress");
        let lockup = self.pending_balances[lockup_id];
        balances[lockup.owner_id] = balances[lockup.owner_id] + lockup.amount;
    }

    fn rollback(&mut self, lockup_id: LockupId) {
        assert!(self.pending_balances.get(lockup_id) > 0, "Promise lockup not in progress");
        // For rollback we have nothing to do
    }

}
