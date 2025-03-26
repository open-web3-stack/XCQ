// TODO: contain source error
use crate::DispatchError;
use parity_scale_codec::Error as CodecError;
use scale_info::prelude::fmt;
use scale_info::prelude::fmt::{Display, Formatter};

/// Errors that can occur when working with extensions
// Typically will be used as a UserError
#[derive(Debug)]
pub enum ExtensionError {
    /// Permission denied for the requested operation
    PermissionError,

    /// Failed to allocate memory
    MemoryAllocationError,

    /// Error accessing memory
    MemoryAccessError(polkavm::MemoryAccessError),

    /// Error decoding data
    DecodeError(CodecError),

    /// Error dispatching a call
    DispatchError(DispatchError),

    /// The requested extension is not supported
    UnsupportedExtension,
}

impl Display for ExtensionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::PermissionError => write!(f, "Permission denied"),
            Self::MemoryAllocationError => write!(f, "Failed to allocate memory"),
            Self::MemoryAccessError(e) => write!(f, "Memory access error: {:?}", e),
            Self::DecodeError(e) => write!(f, "Decode error: {:?}", e),
            Self::DispatchError(e) => write!(f, "Dispatch error: {:?}", e),
            Self::UnsupportedExtension => write!(f, "Unsupported extension"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ExtensionError {}

impl From<polkavm::MemoryAccessError> for ExtensionError {
    fn from(e: polkavm::MemoryAccessError) -> Self {
        Self::MemoryAccessError(e)
    }
}

impl From<CodecError> for ExtensionError {
    fn from(e: CodecError) -> Self {
        Self::DecodeError(e)
    }
}

impl From<DispatchError> for ExtensionError {
    fn from(e: DispatchError) -> Self {
        Self::DispatchError(e)
    }
}
