use super::{Field, Type};
// A Rust struct with named fields.
pub struct StructType {
    ident: Vec<u8>,
    fields: Fields,
}

pub enum Fields {
    Named(Vec<Field>),
    Unnamed(Vec<Type>),
    Unit,
}
