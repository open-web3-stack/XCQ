use crate::derive_impl;
use parity_scale_codec::Codec;

#[extension_decl]
mod extension_core {
    #[extension_decl::extension]
    pub trait ExtensionCore {
        type ExtensionId: Codec;
        fn has_extension(id: Self::ExtensionId) -> bool;
    }
}

#[extension_decl]
mod extension_fungibles {
    #[extension_decl::extension]
    pub trait ExtensionFungibles {
        type AssetId: Codec;
        type AccountId: Codec;
        type Balance: Codec;
        fn total_supply(asset: Self::AssetId) -> Self::Balance;
        fn balance(asset: Self::AssetId, who: Self::AccountId) -> Self::Balance;
    }
}

#[extensions_impl]
mod extensions_impl {

    #[extensions_impl::extensions]
    pub struct Extensions;

    #[extensions_impl::extension_impl]
    impl extension_core::ExtensionCore for Extensions {
        type ExtensionId = u64;
        fn has_extension(id: u64) -> bool {
            matches!(id, 0 | 1)
        }
    }

    #[extensions_impl::extension_impl]
    impl extension_fungibles::ExtensionFungibles for Extensions {
        type AssetId = u32;
        type AccountId = [u8; 32];
        type Balance = u64;
        fn total_supply(asset: u32) -> u64 {
            200
        }
        fn balance(asset: u32, who: [u8; 32]) -> u64 {
            100
        }
    }
}
