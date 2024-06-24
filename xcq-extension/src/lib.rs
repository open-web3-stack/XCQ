#![cfg_attr(not(feature = "std"), no_std)]
use core::marker::PhantomData;
extern crate alloc;
pub use alloc::vec::Vec;

use parity_scale_codec::Decode;
#[cfg(not(feature = "std"))]
use scale_info::prelude::{format, string::String};
use xcq_executor::{Caller, Linker, XcqExecutor, XcqExecutorContext};
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

mod perm_controller;
pub use perm_controller::{InvokeSource, PermController};

mod guest;
pub use guest::{Guest, Input, Method};

// alias trait
pub trait Extension: Dispatchable + ExtensionId + Decode {}
impl<T> Extension for T where T: Dispatchable + ExtensionId + Decode {}

pub trait ExtensionTuple {
    fn dispatch(extension_id: ExtensionIdTy, data: &[u8]) -> Result<Vec<u8>, ExtensionError>;
}

struct Context<E: ExtensionTuple, P: PermController> {
    invoke_source: InvokeSource,
    _marker: PhantomData<(E, P)>,
}

impl<E: ExtensionTuple, P: PermController> Context<E, P> {
    pub fn new(invoke_source: InvokeSource) -> Self {
        Self {
            invoke_source,
            _marker: PhantomData,
        }
    }
}

impl<E: ExtensionTuple, P: PermController> XcqExecutorContext for Context<E, P> {
    fn register_host_functions<T>(&mut self, linker: &mut Linker<T>) {
        let invoke_source = self.invoke_source;
        linker
            .func_wrap(
                "call",
                move |mut caller: Caller<_>, extension_id: u64, call_ptr: u32, call_len: u32| -> u64 {
                    // useful closure to handle early return
                    let mut func_with_result = || -> Result<u64, ExtensionError> {
                        let call_bytes = caller
                            .read_memory_into_vec(call_ptr, call_len)
                            .map_err(|_| ExtensionError::PolkavmError)?;
                        tracing::trace!(
                            "(host call): extension_id: {}, call_bytes: {:?}",
                            extension_id,
                            call_bytes
                        );
                        if !P::is_allowed(extension_id, &call_bytes, invoke_source) {
                            return Err(ExtensionError::PermissionError);
                        }
                        let res_bytes = E::dispatch(extension_id, &call_bytes)?;
                        tracing::trace!("(host call): res_bytes: {:?}", res_bytes);
                        let res_bytes_len = res_bytes.len();
                        let res_ptr = caller.sbrk(res_bytes_len as u32).ok_or(ExtensionError::PolkavmError)?;
                        caller
                            .write_memory(res_ptr, &res_bytes)
                            .map_err(|_| ExtensionError::PolkavmError)?;
                        Ok(((res_ptr as u64) << 32) | (res_bytes_len as u64))
                    };
                    let result = func_with_result();
                    tracing::trace!("(host call): result: {:?}", result);
                    result.unwrap_or(0)
                },
            )
            .unwrap();
    }
}

pub struct ExtensionsExecutor<E: ExtensionTuple, P: PermController> {
    executor: XcqExecutor<Context<E, P>>,
}
impl<E: ExtensionTuple, P: PermController> ExtensionsExecutor<E, P> {
    #[allow(dead_code)]
    pub fn new(source: InvokeSource) -> Self {
        let context = Context::<E, P>::new(source);
        let executor = XcqExecutor::new(Default::default(), context);
        Self { executor }
    }

    #[allow(dead_code)]
    pub fn execute_method<G: Guest, I: Input>(&mut self, guest: G, input: I) -> XcqResult {
        self.executor
            .execute(guest.program(), input.method(), input.args())
            .map_err(|e| format!("{:?}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parity_scale_codec::{Decode, Encode};
    use xcq_extension_core::ExtensionCore;
    use xcq_extension_fungibles::{AccountIdFor, AssetIdFor, BalanceFor, ExtensionFungibles};

    // extension_core impls
    pub struct ExtensionCoreImpl;

    pub struct ExtensionCoreConfigImpl;
    impl xcq_extension_core::Config for ExtensionCoreConfigImpl {
        type ExtensionId = u64;
    }

    impl ExtensionCore for ExtensionCoreImpl {
        type Config = ExtensionCoreConfigImpl;
        fn has_extension(id: <Self::Config as xcq_extension_core::Config>::ExtensionId) -> bool {
            matches!(id, 0 | 1)
        }
    }

    // extension_fungibles impls
    pub struct ExtensionFungiblesImpl;
    pub struct ExtensionFungiblesConfigImpl;

    impl xcq_extension_fungibles::Config for ExtensionFungiblesConfigImpl {
        type AccountId = [u8; 32];
        type Balance = u32;
        type AssetId = u64;
    }

    impl ExtensionFungibles for ExtensionFungiblesImpl {
        type Config = ExtensionFungiblesConfigImpl;
        fn balance(_asset: AssetIdFor<Self>, _who: AccountIdFor<Self>) -> BalanceFor<Self> {
            0
        }
        fn total_supply(_asset: AssetIdFor<Self>) -> BalanceFor<Self> {
            100
        }
    }

    type Extensions = (
        xcq_extension_core::Call<ExtensionCoreImpl>,
        xcq_extension_fungibles::Call<ExtensionFungiblesImpl>,
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

    #[derive(Encode, Decode)]
    enum CoreMethod {
        HasExtension { id: u64 },
    }

    #[derive(Encode, Decode)]
    enum FungiblesMethod {
        Balance { asset: u64, who: [u8; 32] },
        TotalSupply { asset: u64 },
    }
    #[test]
    fn call_core_works() {
        let blob = include_bytes!("../../output/poc-guest-transparent-call.polkavm");
        let mut executor = ExtensionsExecutor::<Extensions, ()>::new(InvokeSource::RuntimeAPI);
        let guest = GuestImpl { program: blob.to_vec() };
        let method = CoreMethod::HasExtension { id: 0 };
        let mut input_data = 0u64.encode();
        input_data.extend_from_slice(&method.encode());
        let input = InputImpl {
            method: "main".to_string(),
            args: input_data,
        };
        let res = executor.execute_method(guest, input).unwrap();
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn call_fungibles_works() {
        let blob = include_bytes!("../../output/poc-guest-transparent-call.polkavm");
        let mut executor = ExtensionsExecutor::<Extensions, ()>::new(InvokeSource::RuntimeAPI);
        let guest = GuestImpl { program: blob.to_vec() };
        let method = FungiblesMethod::TotalSupply { asset: 1u64 };
        let mut input_data = 1u64.encode();
        input_data.extend_from_slice(&method.encode());
        let input = InputImpl {
            method: "main".to_string(),
            args: input_data,
        };
        let res = executor.execute_method(guest, input).unwrap();
        assert_eq!(res, vec![100u8, 0u8, 0u8, 0u8]);
    }
}
