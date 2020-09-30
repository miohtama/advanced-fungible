use near_sdk::{ AccountId, Balance, ext_contract };
use near_sdk::json_types::U128;

/* The smart contract interafce that a smart contract needs to implement to be able to handle incoming token transfers.
 *
 */
#[ext_contract(ext_token_receiver)]
pub trait Receiver {

    /// Interface check promise to check if the receiver contract is able to handle Advanced Fungible
    /// Always return true
    fn is_receiver(self) -> PromiseOrValue<bool>;

    // Notified after the balance transfer is complete. Must return true to finalise the transaction.
    fn on_token_received(&mut self, sender_id: AccountId, amount_received: U128, amount_total: U128, message: Vec<u8>) -> PromiseOrValue<bool>;
}