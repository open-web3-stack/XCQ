use crate::interface::AccountId;
use crate::Balances;
use frame::deps::codec::{Decode, Encode};
#[allow(unused_imports)]
use frame::deps::scale_info::prelude::{format, string::String};
use frame::deps::sp_api::decl_runtime_apis;
use frame::prelude::*;

pub type XcqResponse = Vec<u8>;
pub type XcqError = String;
pub type XcqResult = Result<XcqResponse, XcqError>;

use poc_extensions::extension_core::{self, ExtensionCore};
use poc_extensions::extension_fungibles::{self, ExtensionFungibles};
use poc_extensions::{ExtensionsExecutor, Guest, Input, InvokeSource, Method};
decl_runtime_apis! {
    pub trait XcqApi {
        fn execute_query(query: Vec<u8>, input: Vec<u8>) -> XcqResult;
    }
}

// extension_core impls
pub struct ExtensionCoreImpl;

pub struct ExtensionCoreConfigImpl;
impl extension_core::Config for ExtensionCoreConfigImpl {
    type ExtensionId = u64;
}

impl ExtensionCore for ExtensionCoreImpl {
    type Config = ExtensionCoreConfigImpl;
    fn has_extension(id: <Self::Config as extension_core::Config>::ExtensionId) -> bool {
        matches!(id, 0 | 1)
    }
}

// extension_fungibles impls
pub struct ExtensionFungiblesImpl;

pub struct ExtensionFungiblesConfigImpl;

impl extension_fungibles::Config for ExtensionFungiblesConfigImpl {
    type AccountId = crate::interface::AccountId;
    type Balance = crate::interface::Balance;
    type AssetId = crate::interface::AssetId;
}

type AccountIdFor<T> = <<T as ExtensionFungibles>::Config as extension_fungibles::Config>::AccountId;
type BalanceFor<T> = <<T as ExtensionFungibles>::Config as extension_fungibles::Config>::Balance;
type AssetIdFor<T> = <<T as ExtensionFungibles>::Config as extension_fungibles::Config>::AssetId;

impl ExtensionFungibles for ExtensionFungiblesImpl {
    type Config = ExtensionFungiblesConfigImpl;
    fn balance(asset: AssetIdFor<Self>, who: AccountIdFor<Self>) -> BalanceFor<Self> {
        crate::Assets::balance(asset, who)
    }
    fn total_supply(asset: AssetIdFor<Self>) -> BalanceFor<Self> {
        crate::Assets::total_supply(asset)
    }
}

type Extensions = (
    extension_core::Call<ExtensionCoreImpl>,
    extension_fungibles::Call<ExtensionFungiblesImpl>,
);

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

#[cfg(test)]
mod tests {

    use super::*;
    use frame::deps::sp_core::crypto::AccountId32;
    use frame::deps::sp_core::{sr25519, Pair};
    #[test]
    fn get_data_hex() {
        let raw_blob = include_bytes!("../../../output/poc-guest-query-balance.polkavm");

        let alice_public = sr25519::Pair::from_string("//Alice", None)
            .expect("static values are valid; qed")
            .public();
        let bob_public = sr25519::Pair::from_string("//Bob", None)
            .expect("static values are valid; qed")
            .public();
        let alice_account: AccountId32 = AccountId32::from(alice_public);
        let bob_account: AccountId32 = AccountId32::from(bob_public);
        let mut data = vec![0u8];
        data.extend_from_slice(&alice_account.encode());
        data.extend_from_slice(&bob_account.encode());
        dbg!(hex::encode((raw_blob.to_vec(), data).encode()));
    }
    #[test]
    fn check_balance() {
        // paste from e2e result
        let bytes = hex::decode("200000e8890423c78a").unwrap();
        let decoded_bytes = Vec::<u8>::decode(&mut &bytes[..]).unwrap();
        let u64_array = <[u8; 8]>::try_from(decoded_bytes).unwrap();
        let res = u64::from_le_bytes(u64_array);
        assert_eq!(res, 10000000000000000000);
    }
}
