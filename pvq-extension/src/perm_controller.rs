use crate::ExtensionIdTy;
pub trait PermissionController {
    fn is_allowed(extension_id: ExtensionIdTy, call: &[u8], source: InvokeSource) -> bool;
}

impl PermissionController for () {
    fn is_allowed(_extension_id: ExtensionIdTy, _call: &[u8], _context: InvokeSource) -> bool {
        true
    }
}

#[derive(Copy, Clone)]
pub enum InvokeSource {
    RuntimeAPI,
    XCM,
    Extrinsic,
    Runtime,
}
