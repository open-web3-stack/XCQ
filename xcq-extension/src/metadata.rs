use xcq_primitives::metadata_ir::ExtensionMetadataIR;

use crate::extension_id;
// This trait is for CallData
pub trait CallMetadata {
    fn metadata() -> ExtensionMetadataIR;
}

// This trait is for runtime
pub trait ExtensionMetadata {
    fn extension_metadata(extension_id: extension_id::ExtensionIdTy) -> ExtensionMetadataIR;
}
