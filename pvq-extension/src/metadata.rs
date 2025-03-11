use crate::ExtensionIdTy;

// This trait is for ExtensionImpl
pub trait ExtensionImplMetadata {
    fn extension_metadata(extension_id: ExtensionIdTy) -> ExtensionMetadata;
}

use parity_scale_codec::Encode;
use scale_info::{
    form::{Form, MetaForm, PortableForm},
    prelude::vec::Vec,
    IntoPortable, PortableRegistry, Registry,
};
/// Metadata of extensions
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct Metadata {
    pub types: PortableRegistry,
    pub extensions: Vec<ExtensionMetadata<PortableForm>>,
}

impl Metadata {
    pub fn new(extensions: Vec<ExtensionMetadata>) -> Self {
        let mut registry = Registry::new();
        let extensions = registry.map_into_portable(extensions);
        Self {
            types: registry.into(),
            extensions,
        }
    }
}

/// Metadata of an extension.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct ExtensionMetadata<T: Form = MetaForm> {
    pub name: T::String,
    pub functions: Vec<FunctionMetadata<T>>,
}

impl IntoPortable for ExtensionMetadata {
    type Output = ExtensionMetadata<PortableForm>;

    fn into_portable(self, registry: &mut Registry) -> Self::Output {
        ExtensionMetadata {
            name: self.name.into_portable(registry),
            functions: registry.map_into_portable(self.functions),
        }
    }
}

/// Metadata of a runtime function.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct FunctionMetadata<T: Form = MetaForm> {
    /// Method name.
    pub name: T::String,
    /// Method parameters.
    pub inputs: Vec<FunctionParamMetadata<T>>,
    /// Method output.
    pub output: T::Type,
}

impl IntoPortable for FunctionMetadata {
    type Output = FunctionMetadata<PortableForm>;

    fn into_portable(self, registry: &mut Registry) -> Self::Output {
        FunctionMetadata {
            name: self.name.into_portable(registry),
            inputs: registry.map_into_portable(self.inputs),
            output: registry.register_type(&self.output),
        }
    }
}

/// Metadata of a runtime method parameter.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct FunctionParamMetadata<T: Form = MetaForm> {
    /// Parameter name.
    pub name: T::String,
    /// Parameter type.
    pub ty: T::Type,
}

impl IntoPortable for FunctionParamMetadata {
    type Output = FunctionParamMetadata<PortableForm>;

    fn into_portable(self, registry: &mut Registry) -> Self::Output {
        FunctionParamMetadata {
            name: self.name.into_portable(registry),
            ty: registry.register_type(&self.ty),
        }
    }
}
