mod composite;
mod field;
mod primitive;
mod variant;

pub use self::{composite::*, field::*, primitive::*, variant::*};
pub enum Type {
    Primitive(PrimitiveType),
    Struct(StructType),
    Enum(EnumType),
}

