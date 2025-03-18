#![no_std]
#![no_main]

#[pvq_program::program]
mod sum_balance_percent {
    type AssetId = u32;
    type AccountId = [u8; 32];
    type Balance = u64;
    use alloc::vec::Vec;
    #[program::extension_fn(extension_id = 4071833530116166512u64, fn_index = 1)]
    fn balance(asset: AssetId, who: AccountId) -> Balance {}
    #[program::extension_fn(extension_id = 4071833530116166512u64, fn_index = 0)]
    fn total_supply(asset: AssetId) -> Balance {}

    #[program::entrypoint]
    fn sum_balance(asset: AssetId, accounts: Vec<AccountId>) -> Balance {
        let mut sum_balance = 0;
        for account in accounts {
            sum_balance += balance(asset, account);
        }
        sum_balance * 100 / total_supply(asset)
    }
}
