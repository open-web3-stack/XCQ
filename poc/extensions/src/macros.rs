use crate::Extension;
use crate::ExtensionError;
use crate::ExtensionIdTy;
use crate::ExtensionTuple;
macro_rules! impl_extension_tuple {
    ($($Ts:ident),*) => {
        impl<$($Ts: Extension),*> ExtensionTuple for ($($Ts,)*) {
            fn dispatch(extension_id: ExtensionIdTy, data: &[u8]) -> Result<Vec<u8>, ExtensionError> {
                $(
                    if extension_id == $Ts::EXTENSION_ID {
                        return $Ts::decode(&mut &data[..])
                            .map_err(ExtensionError::DecodeError)?
                            .dispatch()
                            .map_err(ExtensionError::DispatchError);
                    }
                )*
                Err(ExtensionError::UnsupportedExtension)
            }
        }
    };
}

// Use the macro to implement ExtensionTuple for tuples of different lengths
impl_extension_tuple!(T0);
impl_extension_tuple!(T0, T1);
impl_extension_tuple!(T0, T1, T2);
impl_extension_tuple!(T0, T1, T2, T3);
impl_extension_tuple!(T0, T1, T2, T3, T4);
impl_extension_tuple!(T0, T1, T2, T3, T4, T5);
impl_extension_tuple!(T0, T1, T2, T3, T4, T5, T6);
