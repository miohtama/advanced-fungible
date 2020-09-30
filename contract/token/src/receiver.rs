use near_sdk::{ AccountId, Balance, ext_contract };

/* The smart contract interafce that a smart contract needs to implement to be able to handle incoming token transfers.
 *
 */
#[ext_contract(ext_token_receiver)]
pub trait Receiver {

    /// Always return true
    fn is_receiver(self) -> PromiseOrValue<bool>;

    fn on_token_received(&mut self, sender_id: AccountId, amount_received: Balance, amount_total: Balance, message: Vec<u8>) -> Option<String>;
}