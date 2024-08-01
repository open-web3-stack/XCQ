use crate::{vec::Vec, Field};
use parity_scale_codec::Encode;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode)]
pub struct Variant {
    pub ident: Vec<u8>,
    pub fields: Vec<Field>,
}

/// A Enum type, consisting of variants
///
/// # Example
///
/// ## A Rust enum with unit
/// ```
/// enum MyEnum {
///     RustAllowsUnitVariants,
///     AndAlsoForTupleStructs(i32, bool),
///     OrNamedStructs { name: String, age: u8 },
/// }
/// ```
///
/// ## A C-like enum type
///
/// ```
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
/// ```
///
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode)]
pub struct EnumType {
    pub ident: Vec<u8>,
    pub variants: Vec<Variant>,
}
