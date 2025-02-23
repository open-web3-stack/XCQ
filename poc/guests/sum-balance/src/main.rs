#![no_std]
#![no_main]
#[global_allocator]
static GLOBAL: polkavm_derive::LeakingAllocator = polkavm_derive::LeakingAllocator;
use alloc::vec::Vec;
// An example instance of pvq program with specific arg types
#[pvq_api::program]
mod sum_balance {
    #[pvq::call_def(extension_id = 4071833530116166512u64, call_index = 1)]
    fn balance(asset: u32, who: [u8; 32]) -> u64 {}
    #[pvq::entrypoint]
    fn sum_balance(calls: Vec<BalanceCall>) -> u64 {
        let mut sum = 0;
        for call in calls {
            sum += call.call();
        }
        sum
    }
}
