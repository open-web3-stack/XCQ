use parity_scale_codec::{Decode, Encode};
use xcq_extension::extension;

pub type AccountIdFor<T> = <<T as ExtensionFungibles>::Config as Config>::AccountId;
pub type BalanceFor<T> = <<T as ExtensionFungibles>::Config as Config>::Balance;
pub type AssetIdFor<T> = <<T as ExtensionFungibles>::Config as Config>::AssetId;

#[extension(1)]
pub trait ExtensionFungibles {
    type Config: Config;
    // fungibles::Inspect (not extensive)
    // fn total_inssuance(asset: AssetIdFor<Self>) -> BalanceFor<Self>;
    // fn minimum_balance(asset: AssetIdFor<Self>) -> BalanceFor<Self>;
    fn total_supply(asset: AssetIdFor<Self>) -> BalanceFor<Self>;
    fn balance(asset: AssetIdFor<Self>, who: AccountIdFor<Self>) -> BalanceFor<Self>;
    // fungibles::InspectEnumerable
    // fn asset_ids() -> Vec<AccountIdFor<Self>>;
    // fn account_balances(who: AccountIdFor<Self>) -> Vec<(AssetIdFor<Self>, BalanceFor<Self>)>;
}

pub trait Config {
    type AccountId: Decode;
    type AssetId: Decode;
    type Balance: Encode;
}
