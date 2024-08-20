#![no_std]
#![no_main]
#[global_allocator]
static GLOBAL: polkavm_derive::LeakingAllocator = polkavm_derive::LeakingAllocator;
use alloc::vec::Vec;
#[xcq_api::program]
mod query_total_supply {

    #[xcq::call_def]
    fn balance(asset: u32, who: [u8; 32]) -> u64 {}

    #[xcq::call_def]
    fn total_supply(asset: u32) -> u64 {}

    #[xcq::entrypoint]
    fn get_total_supply(call: TotalSupplyCall) -> u64 {
        call.call()
    }
}