use xcq_primitives::metadata_ir::ExtensionMetadataIR;
pub trait ExtensionMetadata {
    fn metadata() -> ExtensionMetadataIR;
}
