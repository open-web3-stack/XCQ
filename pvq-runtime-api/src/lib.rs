#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use pvq_primitives::PvqResult;
use sp_api::decl_runtime_apis;

// The runtime API for the PVQ module.
//   - `program`: PVQ binary.
//   - `args`: Query arguments that is SCALE-encoded.
//   - `gas_limit`: Optional gas limit for query execution. When set to `None`, execution is constrained by the default time boundary.
decl_runtime_apis! {
    pub trait PvqApi {
        fn execute_query(program: Vec<u8>, args: Vec<u8>, gas_limit: Option<i64>) -> PvqResult;
        fn metadata() -> Vec<u8>;
    }
}
