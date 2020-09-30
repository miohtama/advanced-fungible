#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use near_sdk::wee_alloc;

pub mod token;
pub mod receiver;
pub mod utils;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
