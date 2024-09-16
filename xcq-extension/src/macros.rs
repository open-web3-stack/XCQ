use crate::Extension;
use crate::ExtensionError;
use crate::ExtensionIdTy;
use crate::ExtensionTuple;
use crate::Vec;

// Use the macro to implement ExtensionTuple for tuples of different lengths
#[impl_trait_for_tuples::impl_for_tuples(10)]
#[tuple_types_custom_trait_bound(Extension)]
impl ExtensionTuple for Tuple {
    fn dispatch(extension_id: ExtensionIdTy, mut call: &[u8]) -> Result<Vec<u8>, ExtensionError> {
        for_tuples!(
            #(
                if extension_id == Tuple::EXTENSION_ID {
                    return Tuple::decode(&mut call).map_err(ExtensionError::DecodeError)?.dispatch().map_err(ExtensionError::DispatchError);
                }
            )*
        );
        Err(ExtensionError::UnsupportedExtension)
    }
    fn return_ty(extension_id: ExtensionIdTy, call_index: u32) -> Result<Vec<u8>, ExtensionError> {
        for_tuples!(
            #(
                if extension_id == Tuple::EXTENSION_ID {
                    return Ok(Tuple::metadata().methods[call_index as usize].output.type_id().encode());
                }
            )*
        );
        Err(ExtensionError::UnsupportedExtension)
    }
}
