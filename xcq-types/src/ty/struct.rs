use crate::prelude::vec::Vec;
use crate::Field;

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
///
/// ## A tuple struct with unnamed fields.
///
/// ```
/// struct Color(u8, u8, u8);
/// ```
///
/// ## A so-called unit struct
///
/// ```
/// struct JustAMarker;
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct StructType {
    ident: Vec<u8>,
    fields: Vec<Field>,
}
