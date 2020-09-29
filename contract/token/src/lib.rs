use near_sdk::wee_alloc;

pub mod token;
pub mod receiver;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
