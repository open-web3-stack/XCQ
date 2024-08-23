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
    fn sum_balance(balances: Vec<BalanceCall>, total_supply: TotalSupplyCall) -> u64 {
        let mut sum_balance = 0;
        for call in balances {
            sum_balance += call.call();
        }
        sum_balance * 100 / total_supply.call()
    }
}
