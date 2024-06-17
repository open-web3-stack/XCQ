#![cfg_attr(not(feature = "std"), no_std)]
use core::marker::PhantomData;
extern crate alloc;
pub use alloc::vec::Vec;

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

pub mod extension_core;
pub mod extension_fungibles;

mod perm_controller;
pub use perm_controller::{InvokeSource, PermController};

mod guest;
pub use guest::{Guest, Input, Method};

// alias trait
trait Extension: Dispatchable + ExtensionId + Decode {}
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
    fn register_host_functions<T>(&mut self, linker: &mut poc_executor::Linker<T>) {
        let invoke_source = self.invoke_source;
        linker
            .func_wrap(
                "call",
                move |mut caller: poc_executor::Caller<_>, extension_id: u64, call_ptr: u32, call_len: u32| -> u64 {
                    // useful closure to handle early return
                    let mut func_with_result = || -> Result<u64, ExtensionError> {
                        let call_bytes = caller
                            .read_memory_into_vec(call_ptr, call_len)
                            .map_err(|_| ExtensionError::PolkavmError)?;
                        if P::is_allowed(extension_id, &call_bytes, invoke_source) {
                            return Err(ExtensionError::PermissionError);
                        }
                        let res_bytes = E::dispatch(extension_id, &call_bytes)?;
                        let res_bytes_len = res_bytes.len();
                        let res_ptr = caller.sbrk(res_bytes_len as u32).ok_or(ExtensionError::PolkavmError)?;
                        caller
                            .write_memory(res_ptr, &res_bytes)
                            .map_err(|_| ExtensionError::PolkavmError)?;
                        Ok(((res_ptr as u64) << 32) | (res_bytes_len as u64))
                    };
                    func_with_result().unwrap_or(0)
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
    // In PoC, guest and input are opaque to the runtime
    // In SDK, we can make them has type
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
    use crate::extension_core::ExtensionCore;
    use crate::extension_fungibles::ExtensionFungibles;

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
        type AccountId = [u8; 32];
        type Balance = u32;
        type AssetId = u32;
    }

    use crate::extension_fungibles::AccountIdFor;
    use crate::extension_fungibles::AssetIdFor;
    use crate::extension_fungibles::BalanceFor;

    impl ExtensionFungibles for ExtensionFungiblesImpl {
        type Config = ExtensionFungiblesConfigImpl;
        // fn total_inssuance(_asset: AssetIdFor<Self>) -> BalanceFor<Self> {
        //     100
        // }
        // fn minimum_balance(_asset: AssetIdFor<Self>) -> BalanceFor<Self> {
        //     0
        // }
        fn balance(_asset: AssetIdFor<Self>, _who: AccountIdFor<Self>) -> BalanceFor<Self> {
            0
        }
        fn total_supply(_asset: AssetIdFor<Self>) -> BalanceFor<Self> {
            100
        }
        // fn asset_ids() -> Vec<AccountIdFor<Self>> {
        //     vec![]
        // }
        // fn account_balances(_who: AccountIdFor<Self>) -> Vec<(AssetIdFor<Self>, BalanceFor<Self>)> {
        //     vec![]
        // }
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

    // TODO: refine the test
    #[test]
    fn extensions_executor_fails() {
        let mut executor = ExtensionsExecutor::<Extensions, ()>::new(InvokeSource::RuntimeAPI);
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

    // TODO: add success test
}
