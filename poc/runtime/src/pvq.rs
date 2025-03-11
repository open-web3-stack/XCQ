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
            if id == pvq_extension_core::extension::extension_id() {
                true
            } else if id == pvq_extension_fungibles::extension::extension_id() {
                true
            } else {
                false
            }
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
    executor.execute_method(query, input, 0)
}

pub fn metadata() -> Metadata {
    extensions::metadata()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::interface::{AccountId, AssetId};
    use frame::deps::codec::{Decode, Encode};
    use frame::deps::sp_core::{sr25519, Pair};

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
    enum FungiblesMethod {
        TotalSupply { asset: AssetId },
        Balance { asset: AssetId, who: AccountId },
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
    enum CoreMethod {
        HasExtension { id: u64 },
    }
    #[test]
    fn call_transparent_data_hex() {
        let raw_blob = include_bytes!("../../../output/poc-guest-transparent-call.polkavm");
        // call fungible extension
        let mut data = pvq_extension_fungibles::EXTENSION_ID.encode();
        let method = FungiblesMethod::TotalSupply { asset: 21 };
        data.extend_from_slice(&method.encode());
        dbg!(hex::encode((raw_blob.to_vec(), data).encode()));
    }

    #[test]
    fn call_fungibles_hex() {
        let raw_blob = include_bytes!("../../../output/poc-guest-sum-balance.polkavm");
        let alice_public = sr25519::Pair::from_string("//Alice", None)
            .expect("static values are valid; qed")
            .public();
        let alice_account = AccountId::from(alice_public);
        // query num
        let mut data = pvq_extension_fungibles::EXTENSION_ID.encode();
        data.extend_from_slice(&vec![2u8]);
        let method1 = FungiblesMethod::Balance {
            asset: 21,
            who: alice_account.clone().into(),
        };
        let method1_encoded = method1.encode();
        data.extend_from_slice(&vec![method1_encoded.len() as u8]);
        let method2 = FungiblesMethod::Balance {
            asset: 1984,
            who: alice_account.into(),
        };
        let method2_encoded = method2.encode();
        data.extend_from_slice(&method1_encoded);
        data.extend_from_slice(&method2_encoded);
        dbg!(hex::encode((raw_blob.to_vec(), data).encode()));
    }

    #[test]
    fn check_supply() {
        let bytes = hex::decode("2000ca9a3b00000000").unwrap();
        let decoded_bytes = Vec::<u8>::decode(&mut &bytes[..]).unwrap();
        let balance = u64::decode(&mut &decoded_bytes[..]).unwrap();
        assert_eq!(balance, 1_000_000_000);
    }

    #[test]
    fn check_balance_sum() {
        let bytes = hex::decode("200094357700000000").unwrap();
        let decoded_bytes = Vec::<u8>::decode(&mut &bytes[..]).unwrap();
        let balance = u64::decode(&mut &decoded_bytes[..]).unwrap();
        assert_eq!(balance, 2_000_000_000);
    }

    #[test]
    fn check_bool() {
        // paste from e2e result
        let bytes = hex::decode("0401").unwrap();
        let decoded_bytes = Vec::<u8>::decode(&mut &bytes[..]).unwrap();
        let true_value = bool::decode(&mut &decoded_bytes[..]).unwrap();
        assert!(true_value);
    }
}
