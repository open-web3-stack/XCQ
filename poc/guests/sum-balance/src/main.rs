#![no_std]
#![no_main]
#[global_allocator]
static GLOBAL: polkavm_derive::LeakingAllocator = polkavm_derive::LeakingAllocator;
use alloc::vec::Vec;
#[xcq_api::program]
mod sum_balance {
    #[xcq::call_def]
    fn balance(asset: u32, who: [u8; 32]) -> u64 {}
    #[xcq::call_def]
    fn total_supply(asset: u32) -> u64 {}

    #[xcq::entrypoint]
    fn sum_balance(calls: Vec<BalanceCall>) -> u64 {
        let mut sum = 0;
        for call in calls {
            sum += call.call();
        }
        sum
    }
}
