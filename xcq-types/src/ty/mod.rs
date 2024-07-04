mod r#enum;
mod field;
mod primitive;
mod r#struct;

pub use self::{field::*, primitive::*, r#enum::*, r#struct::*};

/// Note: no Array Type yet
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum XcqType {
    Primitive(PrimitiveType),
    Struct(StructType),
    Enum(EnumType),
    Tuple(Vec<XcqType>),
    Sequence(Box<XcqType>),
}
