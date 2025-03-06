use parity_scale_codec::Decode;
use scale_info::prelude::vec::Vec;

/// Type for extension IDs
pub type ExtensionIdTy = u64;

/// Trait for identifying extensions
pub trait ExtensionId {
    const EXTENSION_ID: ExtensionIdTy;
}

/// Trait for dispatching extension calls
pub trait Dispatchable {
    fn dispatch(self) -> Result<Vec<u8>, DispatchError>;
}

/// Error type for dispatch operations
#[derive(Debug)]
pub enum DispatchError {
    PhantomData,
}

/// Trait for extension call data
///
/// This trait combines several traits that are required for extension call data:
/// - `Dispatchable`: Allows dispatching calls to the extension functions
/// - `ExtensionId`: Identifies the extension
/// - `Decode`: Allows decoding the call data
pub trait CallData: Dispatchable + ExtensionId + Decode {}
impl<T> CallData for T where T: Dispatchable + ExtensionId + Decode {}
