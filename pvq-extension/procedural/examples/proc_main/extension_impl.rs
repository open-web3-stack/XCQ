use pvq_extension_procedural::extensions_impl;

#[extensions_impl]
mod extensions_impl {
    use crate::extension_decl::{extension_core, extension_fungibles};

    #[extensions_impl::impl_struct]
    pub struct ExtensionsImpl;

    #[extensions_impl::extension]
    impl extension_core::ExtensionCore for ExtensionsImpl {
        type ExtensionId = u64;
        fn has_extension(id: u64) -> bool {
            matches!(id, 0 | 1)
        }
    }

    #[extensions_impl::extension]
    impl extension_fungibles::ExtensionFungibles for ExtensionsImpl {
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
