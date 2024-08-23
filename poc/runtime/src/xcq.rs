#[allow(unused_imports)]
use frame::deps::scale_info::prelude::{format, string::String};
use frame::deps::sp_api::decl_runtime_apis;
use frame::prelude::*;

pub type XcqResponse = Vec<u8>;
pub type XcqError = String;
pub type XcqResult = Result<XcqResponse, XcqError>;

use xcq_extension::{impl_extensions, ExtensionsExecutor, Guest, Input, InvokeSource, Method};
use xcq_primitives::metadata::Metadata;
decl_runtime_apis! {
    pub trait XcqApi {
        fn execute_query(query: Vec<u8>, input: Vec<u8>) -> XcqResult;
        fn metadata() -> Vec<u8>;
    }
}

// extension_core impls
pub struct ExtensionImpl;

impl xcq_extension_core::Config for ExtensionImpl {
    type ExtensionId = u64;
}

// extension_fungibles impls
impl xcq_extension_fungibles::Config for ExtensionImpl {
    type AccountId = [u8; 32];
    type Balance = crate::interface::Balance;
    type AssetId = crate::interface::AssetId;
}
impl_extensions! {
    impl xcq_extension_core::ExtensionCore for ExtensionImpl {
        type Config = ExtensionImpl;
        fn has_extension(id: <Self::Config as xcq_extension_core::Config>::ExtensionId) -> bool {
            matches!(id, xcq_extension_core::EXTENSION_ID | xcq_extension_fungibles::EXTENSION_ID)
        }
    }

    impl xcq_extension_fungibles::ExtensionFungibles for ExtensionImpl {
        type Config = ExtensionImpl;
        fn balance(
            asset: <Self::Config as xcq_extension_fungibles::Config>::AssetId,
            who: <Self::Config as xcq_extension_fungibles::Config>::AccountId,
        ) -> <Self::Config as xcq_extension_fungibles::Config>::Balance {
            crate::Assets::balance(asset, crate::interface::AccountId::from(who))
        }
        fn total_supply(asset: <Self::Config as xcq_extension_fungibles::Config>::AssetId) -> <Self::Config as xcq_extension_fungibles::Config>::Balance {
            crate::Assets::total_supply(asset)
        }
    }
}

// guest impls
pub struct GuestImpl {
    pub program: Vec<u8>,
}

impl Guest for GuestImpl {
    fn program(&self) -> &[u8] {
        &self.program
    }
}

pub struct InputImpl {
    pub method: Method,
    pub args: Vec<u8>,
}

impl Input for InputImpl {
    fn method(&self) -> Method {
        self.method.clone()
    }
    fn args(&self) -> &[u8] {
        &self.args
    }
}
pub fn execute_query(query: Vec<u8>, input: Vec<u8>) -> XcqResult {
    let mut executor = ExtensionsExecutor::<Extensions, ()>::new(InvokeSource::RuntimeAPI);
    let guest = GuestImpl { program: query };
    let input = InputImpl {
        method: "main".to_owned(),
        args: input,
    };
    executor.execute_method(guest, input)
}

pub fn metadata() -> Metadata {
    ExtensionImpl::runtime_metadata().into()
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
        let mut data = xcq_extension_fungibles::EXTENSION_ID.encode();
        let method = FungiblesMethod::TotalSupply { asset: 21 };
        data.extend_from_slice(&method.encode());
        dbg!(hex::encode((raw_blob.to_vec(), data).encode()));
    }

    #[test]
    fn call_fungibles_hex() {
        let raw_blob = include_bytes!("../../../output/poc-guest-test-xcq-api.polkavm");
        let alice_public = sr25519::Pair::from_string("//Alice", None)
            .expect("static values are valid; qed")
            .public();
        let alice_account = AccountId::from(alice_public);
        // query num
        let mut data = xcq_extension_fungibles::EXTENSION_ID.encode();
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
