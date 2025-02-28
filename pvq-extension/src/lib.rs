#![cfg_attr(not(feature = "std"), no_std)]
use parity_scale_codec::Decode;
use pvq_executor::{Caller, Linker, PvqExecutor, PvqExecutorContext};
use scale_info::prelude::{format, marker::PhantomData, vec::Vec};
pub type PvqResponse = Vec<u8>;
use pvq_primitives::{PvqError, PvqResult};

mod dispatchable;
pub use dispatchable::{DispatchError, Dispatchable};
mod extension_id;
pub mod metadata;
pub use extension_id::{ExtensionId, ExtensionIdTy};
pub use metadata::{CallMetadata, ExtensionImplMetadata};
mod error;
pub use error::ExtensionError;
mod macros;
pub use pvq_extension_procedural::{decl_extensions, impl_extensions};

mod perm_controller;
pub use perm_controller::{InvokeSource, PermissionController};

mod guest;
pub use guest::{Guest, Input, Method};

// Call data
pub trait CallData: Dispatchable + CallMetadata + ExtensionId + Decode {}
impl<T> CallData for T where T: Dispatchable + CallMetadata + ExtensionId + Decode {}

pub trait CallDataTuple {
    fn dispatch(extension_id: ExtensionIdTy, data: &[u8]) -> Result<Vec<u8>, ExtensionError>;
    // TODO: check if use metadata api
    fn return_ty(extension_id: ExtensionIdTy, call_index: u32) -> Result<Vec<u8>, ExtensionError>;
}

struct Context<C: CallDataTuple, P: PermissionController> {
    invoke_source: InvokeSource,
    user_data: (),
    _marker: PhantomData<(C, P)>,
}

impl<C: CallDataTuple, P: PermissionController> Context<C, P> {
    pub fn new(invoke_source: InvokeSource) -> Self {
        Self {
            invoke_source,
            user_data: (),
            _marker: PhantomData,
        }
    }
}

impl<C: CallDataTuple, P: PermissionController> PvqExecutorContext for Context<C, P> {
    type UserData = ();
    type UserError = ExtensionError;
    fn data(&mut self) -> &mut Self::UserData {
        &mut self.user_data
    }
    fn register_host_functions(&mut self, linker: &mut Linker<Self::UserData, Self::UserError>) {
        let invoke_source = self.invoke_source;
        linker
            .define_typed(
                "host_call",
                move |caller: Caller<'_, Self::UserData>,
                      extension_id: u64,
                      call_ptr: u32,
                      call_len: u32|
                      -> Result<u64, ExtensionError> {
                    // useful closure to handle early return
                    let call_bytes = caller.instance.read_memory(call_ptr, call_len)?;
                    tracing::info!("(host call): call_ptr: {}, call_len: {:?}", call_ptr, call_len);
                    tracing::info!(
                        "(host call): extension_id: {}, call_bytes: {:?}",
                        extension_id,
                        call_bytes
                    );
                    if !P::is_allowed(extension_id, &call_bytes, invoke_source) {
                        return Err(ExtensionError::PermissionError);
                    }
                    let res_bytes = C::dispatch(extension_id, &call_bytes)?;
                    tracing::debug!("(host call): res_bytes: {:?}", res_bytes);
                    let res_bytes_len = res_bytes.len();
                    let res_ptr = caller
                        .instance
                        .sbrk(0)
                        .map_err(|_| ExtensionError::MemoryAllocationError)?
                        .ok_or(ExtensionError::MemoryAllocationError)?;
                    caller
                        .instance
                        .sbrk(res_bytes_len as u32)
                        .map_err(|_| ExtensionError::MemoryAllocationError)?
                        .ok_or(ExtensionError::MemoryAllocationError)?;
                    caller.instance.write_memory(res_ptr, &res_bytes)?;
                    Ok(((res_bytes_len as u64) << 32) | (res_ptr as u64))
                },
            )
            .unwrap();
        linker
            .define_typed(
                "return_ty",
                move |caller: Caller<_>, extension_id: u64, call_index: u32| -> Result<u64, ExtensionError> {
                    let res_bytes = C::return_ty(extension_id, call_index)?;
                    tracing::debug!("(host call): res_bytes: {:?}", res_bytes);
                    let res_bytes_len = res_bytes.len();
                    let res_ptr = caller
                        .instance
                        .sbrk(0)
                        .map_err(|_| ExtensionError::MemoryAllocationError)?
                        .ok_or(ExtensionError::MemoryAllocationError)?;
                    caller
                        .instance
                        .sbrk(res_bytes_len as u32)
                        .map_err(|_| ExtensionError::MemoryAllocationError)?
                        .ok_or(ExtensionError::MemoryAllocationError)?;
                    caller.instance.write_memory(res_ptr, &res_bytes)?;
                    Ok(((res_bytes_len as u64) << 32) | (res_ptr as u64))
                },
            )
            .unwrap();
    }
}

pub struct ExtensionsExecutor<C: CallDataTuple, P: PermissionController> {
    executor: PvqExecutor<Context<C, P>>,
}
impl<C: CallDataTuple, P: PermissionController> ExtensionsExecutor<C, P> {
    #[allow(dead_code)]
    pub fn new(source: InvokeSource) -> Self {
        let context = Context::<C, P>::new(source);
        let executor = PvqExecutor::new(Default::default(), context);
        Self { executor }
    }

    #[allow(dead_code)]
    pub fn execute_method(&mut self, query: &[u8], input: &[u8]) -> PvqResult {
        self.executor
            .execute(query, input)
            .map_err(|e| PvqError::Custom(format!("{:?}", e)))
    }
}
