use crate::Field;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
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
/// gives
/// ```ignore
/// EnumType {
///     ident: "MyEnum".encode(),
///     variants: vec![
/// Variant{ident:"RustAllowsUnitVariants".encode(),vec![]},
/// Variant{ident:"AndAlsoForTupleStructs".encode(),vec![Field { ident: vec![], ty: XcqType::Primitive(PrimitiveType::Signed) }, Field { ident: vec![], ty: XcqType::Primitive(PrimitiveType::Bool) }]}
/// Variant{ident:"OrNamedStructs".encode(),vec![Field { ident: "name".encode(), ty: XcqType::Primitive(PrimitiveType::String) }, Field { ident: "age".encode(), ty: XcqType::Primitive(PrimitiveType::Unsigned) }]}
/// ]
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct EnumType {
    pub ident: Vec<u8>,
    pub variants: Vec<Variant>,
}
