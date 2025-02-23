#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use pvq_primitives::PvqResult;
use sp_api::decl_runtime_apis;

// The runtime API for the PVQ module.
// query: the query to be executed, written in polkavm. i.e. query balance of given accounts and sum them up
// input: the input data for the query. i.e. accounts to be queried
decl_runtime_apis! {
    pub trait PvqApi {
        fn execute_query(query: Vec<u8>, input: Vec<u8>) -> PvqResult;
        fn metadata() -> Vec<u8>;
    }
}
