#![cfg_attr(not(feature = "std"), no_std)]
use pvq_extension::extension_decl;

#[extension_decl]
pub mod extension {
    #[extension_decl::extension]
    pub trait ExtensionFungibles {
        type AssetId;
        type Balance;
        type AccountId;
        fn total_supply(asset: Self::AssetId) -> Self::Balance;
        fn balance(asset: Self::AssetId, who: Self::AccountId) -> Self::Balance;
    }
}
