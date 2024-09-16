#![no_std]
#![no_main]
#[global_allocator]
static GLOBAL: polkavm_derive::LeakingAllocator = polkavm_derive::LeakingAllocator;
use alloc::vec::Vec;
// An example instance of xcq program with specific arg types
#[xcq_api::program]
mod sum_balance {
    #[xcq::call_def(extension_id = 0x0000, call_index = 0)]
    fn balance(asset: u32, who: [u8; 32]) -> u64 {}
    #[xcq::entrypoint]
    fn sum_balance(calls: Vec<BalanceCall>) -> u64 {
        let mut sum = 0;
        for call in calls {
            sum += call.call();
        }
        sum
    }
}
