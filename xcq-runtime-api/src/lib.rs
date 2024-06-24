#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use sp_api::decl_runtime_apis;
use xcq_primitives::XcqResult;

// The runtime API for the XCQ module.
// query: the query to be executed, written in polkavm. i.e. query balance of given accounts and sum them up
// input: the input data for the query. i.e. accounts to be queried
decl_runtime_apis! {
    pub trait XcqApi {
        fn execute_query(query: Vec<u8>, input: Vec<u8>) -> XcqResult;
    }
}
