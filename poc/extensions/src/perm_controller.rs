use crate::ExtensionIdTy;
pub trait PermController {
    fn is_allowed(extension_id: ExtensionIdTy, call: Vec<u8>, source: &InvokeSource) -> bool;
}

#[derive(Clone)]
pub enum InvokeSource {
    RuntimeAPI,
    XCM,
    Extrinsic,
    Runtime,
}

impl PermController for () {
    fn is_allowed(_extension_id: ExtensionIdTy, _call: Vec<u8>, _context: &InvokeSource) -> bool {
        true
    }
}
