use pvq_executor::{Caller, Linker, PvqExecutorContext};
use scale_info::prelude::marker::PhantomData;

use crate::{
    error::ExtensionError,
    perm_controller::{InvokeSource, PermissionController},
    CallDataTuple,
};

/// Execution context for extensions
///
/// This struct provides the context for executing extensions.
/// It includes the invoke source and user data.
pub struct Context<C: CallDataTuple, P: PermissionController> {
    /// The source of the invocation
    invoke_source: InvokeSource,
    /// User data for the context
    user_data: (),
    /// Marker for the generic parameters
    _marker: PhantomData<(C, P)>,
}

impl<C: CallDataTuple, P: PermissionController> Context<C, P> {
    /// Create a new context
    ///
    /// # Arguments
    ///
    /// * `invoke_source` - The source of the invocation
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

        // Register the host_call function
        linker
            .define_typed(
                "host_call",
                move |caller: Caller<'_, Self::UserData>,
                      extension_id: u64,
                      call_ptr: u32,
                      call_len: u32|
                      -> Result<u64, ExtensionError> {
                    // Read the call data from memory
                    let call_bytes = caller.instance.read_memory(call_ptr, call_len)?;
                    tracing::info!("(host call): call_ptr: {}, call_len: {:?}", call_ptr, call_len);
                    tracing::info!(
                        "(host call): extension_id: {}, call_bytes: {:?}",
                        extension_id,
                        call_bytes
                    );

                    // Check permissions
                    if !P::is_allowed(extension_id, &call_bytes, invoke_source) {
                        return Err(ExtensionError::PermissionError);
                    }

                    // Dispatch the call
                    let res_bytes = C::dispatch(extension_id, &call_bytes)?;
                    tracing::debug!("(host call): res_bytes: {:?}", res_bytes);

                    // Allocate memory for the response
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

                    // Write the response to memory
                    caller.instance.write_memory(res_ptr, &res_bytes)?;

                    // Return the pointer and length
                    Ok(((res_bytes_len as u64) << 32) | (res_ptr as u64))
                },
            )
            .expect("Failed to register host_call function");
    }
}
