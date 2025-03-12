use crate::ExtensionIdTy;

/// Source of an extension invocation
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InvokeSource {
    /// Invoked from a runtime API
    RuntimeAPI,

    /// Invoked from XCM (Cross-Consensus Message)
    XCM,

    /// Invoked from an extrinsic
    Extrinsic,

    /// Invoked from the runtime inside
    Runtime,
}

/// Controller for extension permissions
///
/// This trait is used to control access to extensions based on the extension ID,
/// call data, and invocation source.
pub trait PermissionController {
    /// Check if a call to an extension is allowed
    ///
    /// # Arguments
    ///
    /// * `extension_id` - The ID of the extension
    /// * `call` - The encoded call data
    /// * `source` - The source of the invocation
    ///
    /// # Returns
    ///
    /// `true` if the call is allowed, `false` otherwise
    fn is_allowed(extension_id: ExtensionIdTy, call: &[u8], source: InvokeSource) -> bool;
}

/// Default permission controller that allows everything
impl PermissionController for () {
    fn is_allowed(_extension_id: ExtensionIdTy, _call: &[u8], _source: InvokeSource) -> bool {
        true
    }
}
