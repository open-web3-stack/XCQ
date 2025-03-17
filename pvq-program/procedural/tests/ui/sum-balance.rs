#![no_std]
#![no_main]

#[pvq_program_procedural::program]
mod sum_balance {
    type AccountId = [u8; 32];
    type AssetId = u32;
    type Balance = u64;

    #[program::extension_fn(extension_id = 4071833530116166512u64, fn_index = 1)]
    fn balance(asset: AssetId, who: AccountId) -> Balance {}

    #[program::entrypoint]
    fn sum_balance(asset: AssetId, accounts: alloc::vec::Vec<AccountId>) -> Balance {
        let mut sum = 0;
        for account in accounts {
            sum += balance(asset, account);
        }
        sum
    }
}
