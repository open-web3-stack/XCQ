use crate::metadata_ir::{ExtensionMetadataIR, MetadataIR, MethodMetadataIR, MethodParamMetadataIR};
use parity_scale_codec::Encode;
use xcq_types::{vec::Vec, XcqType};
/// Metadata of an extension method.
#[derive(Clone, PartialEq, Eq, Debug, Encode)]
pub struct MethodMetadata {
    /// Method name.
    pub name: &'static str,
    /// Method parameters.
    pub inputs: Vec<MethodParamMetadata>,
    /// Method output.
    pub output: XcqType,
}

/// Metadata of an method parameter.
#[derive(Clone, PartialEq, Eq, Debug, Encode)]
pub struct MethodParamMetadata {
    /// Parameter name.
    pub name: &'static str,
    /// Parameter type.
    pub ty: XcqType,
}

/// Metadata of an extension
#[derive(Clone, PartialEq, Eq, Debug, Encode)]
pub struct ExtensionMetadata {
    pub name: &'static str,
    pub methods: Vec<MethodMetadata>,
}

/// Metadata of extensions
#[derive(Clone, PartialEq, Eq, Debug, Encode)]
pub struct Metadata {
    pub extensions: Vec<ExtensionMetadata>,
}

impl From<MethodParamMetadataIR> for MethodParamMetadata {
    fn from(ir: MethodParamMetadataIR) -> Self {
        Self {
            name: ir.name,
            ty: ir.ty.type_info(),
        }
    }
}

impl From<MethodMetadataIR> for MethodMetadata {
    fn from(ir: MethodMetadataIR) -> Self {
        Self {
            name: ir.name,
            inputs: ir.inputs.into_iter().map(MethodParamMetadata::from).collect(),
            output: ir.output.type_info(),
        }
    }
}

impl From<ExtensionMetadataIR> for ExtensionMetadata {
    fn from(ir: ExtensionMetadataIR) -> Self {
        Self {
            name: ir.name,
            methods: ir.methods.into_iter().map(MethodMetadata::from).collect(),
        }
    }
}

impl From<MetadataIR> for Metadata {
    fn from(ir: MetadataIR) -> Self {
        Self {
            extensions: ir.extensions.into_iter().map(ExtensionMetadata::from).collect(),
        }
    }
}
