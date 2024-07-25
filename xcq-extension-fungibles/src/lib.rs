#![cfg_attr(not(feature = "std"), no_std)]
use parity_scale_codec::{Decode, Encode};
use xcq_extension::decl_extensions;

pub trait Config {
    type AssetId: Encode + Decode;
    type AccountId: Encode + Decode;
    type Balance: Encode + Decode;
}
decl_extensions! {
    pub trait ExtensionFungibles {
        // fungibles::Inspect (not extensive)
        // fn total_inssuance(asset: AssetIdFor<Self>) -> BalanceFor<Self>;
        // fn minimum_balance(asset: AssetIdFor<Self>) -> BalanceFor<Self>;
        type Config: Config;
        fn total_supply(asset: <Self::Config as Config>::AssetId) -> <Self::Config as Config>::Balance;
        fn balance(asset: <Self::Config as Config>::AssetId, who: <Self::Config as Config>::AccountId) -> <Self::Config as Config>::Balance;
        // fungibles::InspectEnumerable
        // fn asset_ids() -> Vec<AccountIdFor<Self>>;
        // fn account_balances(who: AccountIdFor<Self>) -> Vec<(AssetIdFor<Self>, BalanceFor<Self>)>;
    }
}
