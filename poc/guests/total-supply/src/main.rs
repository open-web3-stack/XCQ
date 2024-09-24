#![no_std]
#![no_main]
#[global_allocator]
static GLOBAL: polkavm_derive::LeakingAllocator = polkavm_derive::LeakingAllocator;
#[xcq_api::program]
mod query_total_supply {
    #[xcq::call_def(extension_id = 10588899351449456541u64, call_index = 0)]
    fn total_supply(asset: u32) -> u64 {}

    #[xcq::entrypoint]
    fn get_total_supply(call: TotalSupplyCall) -> u64 {
        call.call()
    }
}
