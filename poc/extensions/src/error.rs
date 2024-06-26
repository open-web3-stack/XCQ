// TODO: contain source error
use crate::DispatchError;
use parity_scale_codec::Error as CodeCError;
#[derive(Debug)]
pub enum ExtensionError {
    PermissionError,
    PolkavmError,
    DecodeError(CodeCError),
    DispatchError(DispatchError),
    UnsupportedExtension,
}
