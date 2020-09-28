use near_sdk::wee_alloc;

mod token;
mod escrow;
mod pool;
mod receiver;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
