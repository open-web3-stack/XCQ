use crate::CallData;
use crate::CallDataTuple;
use crate::ExtensionError;
use crate::ExtensionIdTy;
use crate::Vec;
use fortuples::fortuples;
use parity_scale_codec::Encode;
use scale_info::{PortableRegistry, Registry};
// Use the macro to implement ExtensionTuple for tuples of different lengths
fortuples! {
    impl CallDataTuple for #Tuple where #(#Member: CallData),*{
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        fn dispatch(extension_id: ExtensionIdTy, mut call: &[u8]) -> Result<Vec<u8>, ExtensionError> {
            #(
                if extension_id == #Member::EXTENSION_ID {
                    return #Member::decode(&mut call).map_err(ExtensionError::DecodeError)?.dispatch().map_err(ExtensionError::DispatchError);
                }
            )*
            Err(ExtensionError::UnsupportedExtension)
        }
        #[allow(unused_variables)]
        fn return_ty(extension_id: ExtensionIdTy, call_index: u32) -> Result<Vec<u8>, ExtensionError> {
            #(
                if extension_id == #Member::EXTENSION_ID {
                    let extension_metadata = #Member::call_metadata();
                    //  comparing the registry is equivalent to comparing the type
                    let return_ty = extension_metadata.methods[call_index as usize].output;
                    let mut registry = Registry::new();
                    registry.register_type(&return_ty);
                    let portable_registry: PortableRegistry = registry.into();
                    return Ok(portable_registry.encode());
                }
            )*
            Err(ExtensionError::UnsupportedExtension)
        }
    }
}
