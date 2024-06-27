use super::Type;
// A field of a struct or enum variant.
pub struct Field {
    ident: Vec<u8>,
    ty: Type,
}
