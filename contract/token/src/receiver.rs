use near_sdk::{ AccountId, Balance };

pub trait Receiver {
    fn on_token_received(&mut self, sender_id: AccountId, amount: Balance, message: Vec<u8>) -> Option<String>;
}