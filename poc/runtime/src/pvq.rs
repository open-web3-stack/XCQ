#[allow(unused_imports)]
use frame::deps::scale_info::prelude::{format, string::String};

use pvq_extension::metadata::Metadata;
use pvq_extension::{extensions_impl, ExtensionsExecutor, InvokeSource};

#[extensions_impl]
pub mod extensions {
    #[extensions_impl::impl_struct]
    pub struct ExtensionImpl;

    #[extensions_impl::extension]
    impl pvq_extension_core::extension::ExtensionCore for ExtensionImpl {
        type ExtensionId = u64;
        fn has_extension(id: Self::ExtensionId) -> bool {
            id == pvq_extension_core::extension::extension_id()
                || id == pvq_extension_fungibles::extension::extension_id()
        }
    }

    #[extensions_impl::extension]
    impl pvq_extension_fungibles::extension::ExtensionFungibles for ExtensionImpl {
        type AccountId = [u8; 32];
        type Balance = crate::interface::Balance;
        type AssetId = crate::interface::AssetId;
        fn balance(asset: Self::AssetId, who: Self::AccountId) -> Self::Balance {
            crate::Assets::balance(asset, crate::interface::AccountId::from(who))
        }
        fn total_supply(asset: Self::AssetId) -> Self::Balance {
            crate::Assets::total_supply(asset)
        }
    }
}

pub fn execute_query(program: &[u8], args: &[u8], gas_limit: i64) -> pvq_primitives::PvqResult {
    let mut executor = ExtensionsExecutor::<extensions::Extensions, ()>::new(InvokeSource::RuntimeAPI);
    let (result, _) = executor.execute(program, args, Some(gas_limit));
    result
}

pub fn metadata() -> Metadata {
    extensions::metadata()
}
