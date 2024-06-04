#![cfg_attr(not(feature = "std"), no_std)]
use core::marker::PhantomData;

use parity_scale_codec::Decode;
use poc_executor::{XcqExecutor, XcqExecutorContext};
#[cfg(not(feature = "std"))]
use scale_info::prelude::{format, string::String};
pub type XcqResponse = Vec<u8>;
pub type XcqError = String;
pub type XcqResult = Result<XcqResponse, XcqError>;

mod dispatchable;
pub use dispatchable::{DispatchError, Dispatchable};
mod extension_id;
pub use extension_id::{ExtensionId, ExtensionIdTy};
mod error;
pub use error::ExtensionError;
mod macros;

mod extension_core;
mod extension_fungibles;

mod guest;
pub use guest::{Guest, Input, Method};

// alias trait
trait Extension: Dispatchable + ExtensionId + Decode {}
impl<T> Extension for T where T: Dispatchable + ExtensionId + Decode {}

trait ExtensionTuple {
    fn dispatch(extension_id: ExtensionIdTy, data: Vec<u8>) -> Result<Vec<u8>, ExtensionError>;
}

struct HostFunctions<E: ExtensionTuple> {
    phantom: PhantomData<E>,
}

impl<E: ExtensionTuple> XcqExecutorContext for HostFunctions<E> {
    fn register_host_functions<T>(&mut self, linker: &mut poc_executor::Linker<T>) {
        linker
            .func_wrap(
                "_",
                |mut caller: poc_executor::Caller<_>,
                 extension_id: u64,
                 call_ptr: u32,
                 call_len: u32,
                 res_ptr: u32|
                 -> u32 {
                    // useful closure to handle early return
                    let mut func_with_result = || -> Result<u32, ExtensionError> {
                        // TODO: first check if the caller has the permission to call the extension
                        let call_bytes = caller
                            .read_memory_into_vec(call_ptr, call_len)
                            .map_err(|_| ExtensionError::PolkavmError)?;
                        let res_bytes = E::dispatch(extension_id, call_bytes)?;
                        caller
                            .write_memory(res_ptr, &res_bytes[..])
                            .map_err(|_| ExtensionError::PolkavmError)?;
                        Ok(res_bytes.len() as u32)
                    };
                    func_with_result().unwrap_or(0)
                },
            )
            .unwrap();
    }
}

struct ExtensionsExecutor<E: ExtensionTuple> {
    executor: XcqExecutor<HostFunctions<E>>,
}
impl<E: ExtensionTuple> ExtensionsExecutor<E> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let host_functions = HostFunctions::<E> {
            phantom: core::marker::PhantomData,
        };
        let executor = XcqExecutor::new(Default::default(), host_functions);
        Self { executor }
    }
    // In PoC, guest and input are opaque to the runtime
    // In SDK, we can make them has type
    #[allow(dead_code)]
    fn execute_method<G: Guest, I: Input>(&mut self, guest: G, input: I) -> XcqResult {
        self.executor
            .execute(guest.program(), input.method(), input.args())
            .map_err(|e| format!("{:?}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extension_core::ExtensionCore;
    use crate::extension_fungibles::ExtensionFungibles;
    use parity_scale_codec::{Decode, Encode};

    // extension_core impls
    pub struct ExtensionCoreImpl;

    #[derive(Encode, Decode)]
    pub struct ArgsImpl {
        pub a: u32,
        pub b: u32,
    }

    pub struct ConfigImpl;
    impl extension_core::Config for ConfigImpl {
        // this associated type is generated by the macro
        type ArgsOfSomeHostFunction = ArgsImpl;
        type ResultOfSomeHostFunction = u32;
    }

    impl ExtensionId for ExtensionCoreImpl {
        const EXTENSION_ID: ExtensionIdTy = 0u64;
    }

    impl ExtensionCore for ExtensionCoreImpl {
        type Config = ConfigImpl;
        fn some_host_function(
            args: <Self::Config as extension_core::Config>::ArgsOfSomeHostFunction,
        ) -> <Self::Config as extension_core::Config>::ResultOfSomeHostFunction {
            args.a + args.b
        }
    }

    // extension_fungibles impls
    pub struct ExtensionFungiblesImpl;

    impl ExtensionId for ExtensionFungiblesImpl {
        const EXTENSION_ID: ExtensionIdTy = 1u64;
    }

    impl ExtensionFungibles for ExtensionFungiblesImpl {
        fn free_balance_of(_who: [u8; 32]) -> u32 {
            100
        }
        fn reserved_balance_of(_who: [u8; 32]) -> u32 {
            42
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
    impl ExtensionId for GuestImpl {
        const EXTENSION_ID: ExtensionIdTy = 0u64;
    }
    // TODO: refine the test
    #[test]
    fn extensions_executor_fails() {
        let mut executor = ExtensionsExecutor::<Extensions>::new();
        let guest = GuestImpl {
            program: vec![0, 1, 2, 3],
        };
        let input = InputImpl {
            method: "main".to_string(),
            args: vec![0, 1, 2, 3],
        };
        let res = executor.execute_method(guest, input);
        assert!(res.is_err())
    }
}
