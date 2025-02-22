#![no_std]
#![no_main]
#[global_allocator]
static GLOBAL: polkavm_derive::LeakingAllocator = polkavm_derive::LeakingAllocator;
#[pvq_api::program]
mod query_total_supply {
    #[pvq::call_def(extension_id = 4071833530116166512u64, call_index = 0)]
    fn total_supply(asset: u32) -> u64 {}

    #[pvq::entrypoint]
    fn get_total_supply(call: TotalSupplyCall) -> u64 {
        call.call()
    }
}
