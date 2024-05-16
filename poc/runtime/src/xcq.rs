use crate::interface::AccountId;
use crate::Balances;
use frame::deps::sp_api::decl_runtime_apis;
use frame::prelude::*;
#[allow(unused_imports)]
use scale_info::prelude::{format, string::String};

pub type XcqResponse = Vec<u8>;
pub type XcqError = String;
pub type XcqResult = Result<XcqResponse, XcqError>;

decl_runtime_apis! {
    pub trait XcqApi {
        fn execute_query(query: Vec<u8>, input: Vec<u8>) -> XcqResult;
    }
}

struct HostFunctions;

impl poc_executor::XcqExecutorContext for HostFunctions {
    fn register_host_functions<T>(&mut self, linker: &mut poc_executor::Linker<T>) {
        linker
            .func_wrap(
                "query_balances",
                move |caller: poc_executor::Caller<_>, variant: u32, accounts_ptr: u32, accounts_len: u32| -> u64 {
                    // variant 0 means free balance
                    // variant 1 means reserved balance
                    // variant 2 means free+reserved
                    let mut sum = 0u64;
                    let account_id_in_bytes = caller
                        .read_memory_into_vec(accounts_ptr, accounts_len)
                        .expect("read_memory_into_vec failed");
                    let account_ids = Vec::<AccountId>::decode(&mut &account_id_in_bytes[..]).expect("decode failed");
                    for account_id in account_ids {
                        if variant == 0 {
                            sum += Balances::free_balance(&account_id)
                        } else if variant == 1 {
                            sum += Balances::reserved_balance(&account_id)
                        } else if variant == 2 {
                            sum += Balances::free_balance(&account_id) + Balances::reserved_balance(&account_id)
                        }
                    }
                    sum
                },
            )
            .unwrap();
    }
}

pub fn execute_query(query: Vec<u8>, input: Vec<u8>) -> XcqResult {
    let mut executor = poc_executor::XcqExecutor::new(Default::default(), HostFunctions);
    executor.execute(&query, &input).map_err(|e| format!("{:?}", e))
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
        let accounts = vec![alice_account, bob_account];
        data.extend_from_slice(&accounts.encode());
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
