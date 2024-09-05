#![no_std]
#![no_main]
#[global_allocator]
static GLOBAL: polkavm_derive::LeakingAllocator = polkavm_derive::LeakingAllocator;
use alloc::vec::Vec;
#[xcq_api::program]
mod sum_balance {
    #[xcq::call_def(extension_id = , extern_types = [Balance, AccountId, AssetId])]
    fn balance(asset: AssetId, who: AccountId) -> Balance {}
    #[xcq::entrypoint]
    fn sum_balance(calls: Vec<BalanceCall>) -> u64 {
        let mut sum = 0;
        for call in calls {
            let (ty, data) = call.call();
            if ty == 0 {
                sum += u8::from_le_bytes(data) as u64;
            } else if ty == 1 {
                sum += u16::from_le_bytes(data) as u64;
            } else if ty == 2 {
                sum += u32::from_le_bytes(data) as u64;
            } else if ty == 3 {
                sum += u64::from_le_bytes(data) as u64;
            } else if ty == 4 {
                sum += u128::from_le_bytes(data) as u64;
            } else {
                sum += 0;
            }
        }
        sum
    }
}
