use crate::prelude::vec::Vec;
use crate::{Field, XcqType};

/// A struct type, consisting of a named (struct) or unnamed (tuple struct) fields or unit struct.
/// Note: in fact, it can represent a
///
/// # Example
///
/// ## A Rust struct with named fields.
///
/// ```
/// struct Person {
///     name: String,
///     age_in_years: u8,
///     friends: Vec<Person>,
/// }
/// ```
/// gives
/// ```ignore
/// StructType {
///     ident: "Person".encode(),
///     fields: vec![Field { ident: "name".encode(), ty: ... }, Field { ident: "age_in_years".encode(), ty: ... }, Field { ident: "friends".encode(), ty: ... }],
/// }
/// ```
///
/// ## A tuple struct with unnamed fields.
///
/// ```
/// struct Color(u8, u8, u8);
/// ```
/// gives
/// ```ignore
/// StructType {
///     ident: "Color".encode(),
///     fields: vec![Field { ident: vec![], ty: XcqType::Primitive(PrimitiveType::Unsigned) }, Field { ident: vec![],encode(), ty: XcqType::Primitive(PrimitiveType::Unsigned) }, Field { ident: vec![], ty: XcqType::Primitive(PrimitiveType::Unsigned) }],
/// ```
///
/// ## A so-called unit struct
///
/// ```
/// struct JustAMarker;
/// ```
/// gives
/// ```ignore
/// StructType { ident: "JustAMarker".encode(), fields: vec![] }
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct StructType {
    ident: Vec<u8>,
    fields: Vec<Field>,
}
