pub type ExtensionIdTy = u64;

pub trait ExtensionId {
    const EXTENSION_ID: ExtensionIdTy;
}
