use crate::extension_id;
// This trait is for CallData
pub trait CallMetadata {
    fn call_metadata() -> ExtensionMetadata;
}

// This trait is for ExtensionImpl
pub trait ExtensionImplMetadata {
    fn extension_metadata(extension_id: extension_id::ExtensionIdTy) -> ExtensionMetadata;
}

#[cfg(feature = "decode")]
use parity_scale_codec::Decode;
#[cfg(feature = "serde_full")]
use serde::Serialize;

use parity_scale_codec::Encode;
use scale_info::{
    form::{Form, MetaForm, PortableForm},
    prelude::vec::Vec,
    IntoPortable, PortableRegistry, Registry,
};
/// Metadata of extensions
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
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
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
    feature = "serde_full",
    serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct ExtensionMetadata<T: Form = MetaForm> {
    pub name: T::String,
    pub methods: Vec<MethodMetadata<T>>,
}

impl IntoPortable for ExtensionMetadata {
    type Output = ExtensionMetadata<PortableForm>;

    fn into_portable(self, registry: &mut Registry) -> Self::Output {
        ExtensionMetadata {
            name: self.name.into_portable(registry),
            methods: registry.map_into_portable(self.methods),
        }
    }
}

/// Metadata of a runtime method.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
    feature = "serde_full",
    serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct MethodMetadata<T: Form = MetaForm> {
    /// Method name.
    pub name: T::String,
    /// Method parameters.
    pub inputs: Vec<MethodParamMetadata<T>>,
    /// Method output.
    pub output: T::Type,
}

impl IntoPortable for MethodMetadata {
    type Output = MethodMetadata<PortableForm>;

    fn into_portable(self, registry: &mut Registry) -> Self::Output {
        MethodMetadata {
            name: self.name.into_portable(registry),
            inputs: registry.map_into_portable(self.inputs),
            output: registry.register_type(&self.output),
        }
    }
}

/// Metadata of a runtime method parameter.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
    feature = "serde_full",
    serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct MethodParamMetadata<T: Form = MetaForm> {
    /// Parameter name.
    pub name: T::String,
    /// Parameter type.
    pub ty: T::Type,
}

impl IntoPortable for MethodParamMetadata {
    type Output = MethodParamMetadata<PortableForm>;

    fn into_portable(self, registry: &mut Registry) -> Self::Output {
        MethodParamMetadata {
            name: self.name.into_portable(registry),
            ty: registry.register_type(&self.ty),
        }
    }
}
