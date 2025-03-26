#[allow(unused_imports)]
use frame::deps::scale_info::prelude::{format, string::String};
use frame::deps::sp_api::decl_runtime_apis;
use frame::prelude::*;

use pvq_extension::metadata::Metadata;
pub use pvq_primitives::PvqResult;

use pvq_extension::{extensions_impl, ExtensionsExecutor, InvokeSource};
decl_runtime_apis! {
    pub trait PvqApi {
        fn execute_query(query: Vec<u8>, input: Vec<u8>) -> PvqResult;
        fn metadata() -> Vec<u8>;
    }
}

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

pub fn execute_query(query: &[u8], input: &[u8]) -> PvqResult {
    let mut executor = ExtensionsExecutor::<extensions::Extensions, ()>::new(InvokeSource::RuntimeAPI);
    let (result, _) = executor.execute_method(query, input, None);
    result
}

pub fn metadata() -> Metadata {
    extensions::metadata()
}
