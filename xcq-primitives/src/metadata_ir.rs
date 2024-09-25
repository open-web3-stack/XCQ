use xcq_types::{vec::Vec, MetaType};
/// Metadata of a runtime method.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MethodMetadataIR {
    /// Method name.
    pub name: &'static str,
    /// Method parameters.
    pub inputs: Vec<MethodParamMetadataIR>,
    /// Method output.
    pub output: MetaType,
}

/// Metadata of a runtime method parameter.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MethodParamMetadataIR {
    /// Parameter name.
    pub name: &'static str,
    /// Parameter type.
    pub ty: MetaType,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExtensionMetadataIR {
    pub name: &'static str,
    pub methods: Vec<MethodMetadataIR>,
}

/// Metadata of extensions
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MetadataIR {
    pub extensions: Vec<ExtensionMetadataIR>,
}

pub trait RuntimeMetadata {
    fn runtime_metadata() -> MetadataIR;
}
