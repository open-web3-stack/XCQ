use parity_scale_codec::Encode;
mod r#enum;
mod field;
mod primitive;
mod r#struct;
use crate::{boxed::Box, vec::Vec};

pub use self::{field::*, primitive::*, r#enum::*, r#struct::*};

/// Note: no Array Type yet
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode)]
pub enum XcqType {
    Primitive(PrimitiveType),
    Struct(StructType),
    Enum(EnumType),
    // TODO: deal with self-referential types
    Tuple(Vec<XcqType>),
    // TODO: deal with self-referential types
    Sequence(Box<XcqType>),
}
