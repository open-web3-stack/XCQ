use crate::CallData;
use crate::CallDataTuple;
use crate::ExtensionError;
use crate::ExtensionIdTy;
use crate::Vec;
use fortuples::fortuples;
use parity_scale_codec::Encode;

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
                    return Ok(#Member::metadata().methods[call_index as usize].output.type_info().encode());
                }
            )*
            Err(ExtensionError::UnsupportedExtension)
        }
    }
}
