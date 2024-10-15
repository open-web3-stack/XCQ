// TODO: contain source error
use crate::DispatchError;
use parity_scale_codec::Error as CodeCError;
#[derive(Debug)]
pub enum ExtensionError {
    PermissionError,
    MemoryAllocationError,
    MemoryAccessError(polkavm::MemoryAccessError),
    DecodeError(CodeCError),
    DispatchError(DispatchError),
    UnsupportedExtension,
}

impl From<polkavm::MemoryAccessError> for ExtensionError {
    fn from(e: polkavm::MemoryAccessError) -> Self {
        Self::MemoryAccessError(e)
    }
}
